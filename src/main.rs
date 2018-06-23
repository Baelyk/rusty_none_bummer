#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate nom;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod db;
mod models;
mod schema;

use chrono::prelude::*;
use models::ActivitiesJSON;
use models::Activity;
use models::ActivityJSON;
use models::AddActivitiesForm;
use models::AddPointsForm;
use models::Friend;
use nom::eol;
use nom::is_alphabetic;
use nom::is_digit;
use nom::line_ending;
use rocket::http::RawStr;
use rocket::request::FlashMessage;
use rocket::request::Form;
use rocket::request::LenientForm;
use rocket::response::Flash;
use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket::Rocket;
use rocket_contrib::Template;
use serde_json::from_value;
use serde_json::to_string;
use serde_json::to_value;
use serde_json::Value as JSON;
use std::path::Path;
use std::path::PathBuf;
use std::str;

#[derive(Serialize)]
struct Context<'a> {
    activities: Vec<Activity>,
    friends: Vec<Friend>,
    msg: Option<&'a str>,
}

impl<'a> Context<'a> {
    pub fn raw(msg: Option<&'a str>, connection: &db::Connection) -> Context<'a> {
        Context {
            activities: Activity::all(connection),
            friends: Friend::all(connection),
            msg: msg,
        }
    }
}

#[derive(Serialize)]
struct FriendContext {
    friend: Friend,
    activities: String,
}

impl FriendContext {
    pub fn raw(connection: &db::Connection, id: i32) -> FriendContext {
        let friend = Friend::get_by_id(connection, id).expect("expected valid friend id");
        FriendContext {
            friend: friend.clone(),
            activities: match &friend.activities {
                Some(friend_acts) => to_string(friend_acts).expect("valid JSON string"),
                None => String::from("[]"),
            },
        }
    }
}

#[get("/<file..>", rank = 10)]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/")]
fn index(msg: Option<FlashMessage>, connection: db::Connection) -> Template {
    match msg {
        Some(ref msg) => Template::render("index", Context::raw(Some(msg.msg()), &connection)),
        None => Template::render("index", Context::raw(None, &connection)),
    }
}

#[get("/activities")]
fn activities(msg: Option<FlashMessage>, connection: db::Connection) -> Template {
    match msg {
        Some(ref msg) => Template::render("activities", Context::raw(Some(msg.msg()), &connection)),
        None => Template::render("activities", Context::raw(None, &connection)),
    }
}

#[get("/friends")]
fn friends(connection: db::Connection) -> Template {
    Template::render("friends", Context::raw(None, &connection))
}

#[get("/friends/<id>", rank = 1)]
fn friend_page_id(id: i32, connection: db::Connection) -> Redirect {
    Redirect::to(&format!(
        "/friends/{}",
        Friend::get_by_id(&connection, id).expect("valid id").name
    ))
}

#[get("/friends/<friend>", rank = 2)]
fn friend_page_name(friend: String, connection: db::Connection) -> Template {
    let id = Friend::get_id_from_name(&connection, friend);
    Template::render("friend", FriendContext::raw(&connection, id))
}

#[post("/add/points", data = "<form>")]
fn add_points(connection: db::Connection, form: LenientForm<AddPointsForm>) -> Flash<Redirect> {
    println!("{:?}", form.raw_form_string());
    named!(activities<&[u8], Vec<i32>>,
        dbg_dmp!(do_parse!(                                                                                  // friend=1&submit=Submit&activity=1&activity=32&footer=Footer
                            take_until!("&activity")                                          >>             //                       &activity=1&activity=32&footer=Footer
                activities: many0!(preceded!(tag!("&activity="), take_while!(is_digit))) >>                  //                                              &footer=Footer

                (activities.into_iter().map(|x| str::from_utf8(x).unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>()) // Ok(("&submit=Submit", vec!["1", "32"]))
            ))
    );
    let act_ids = activities(form.raw_form_string().as_bytes())
        .expect("expected activities")
        .1;
    println!("{:?}: {:?}", form.get().friend, act_ids);
    let mut friend = Friend::get_by_id(&connection, form.get().friend).unwrap();
    let activities = act_ids
        .into_iter()
        .map(|id| Activity::get_by_id(&connection, id).unwrap())
        .collect::<Vec<Activity>>();
    activities.into_iter().for_each(|act| {
        let new_act = ActivityJSON {
            when: Utc::now().timestamp(),
            activity: act.description,
            previousPoints: friend.points,
            currentPoints: friend.points + act.points,
        };
        let new_friend_activites: Option<JSON>;
        match friend.activities {
            Some(ref friend_acts) => {
                let mut new_friend_activites_vec =
                    from_value::<ActivitiesJSON>(friend_acts.clone()).unwrap();
                new_friend_activites_vec.push(new_act);
                new_friend_activites =
                    Some(to_value::<ActivitiesJSON>(new_friend_activites_vec).unwrap());
            }
            None => {
                new_friend_activites = Some(to_value::<ActivitiesJSON>(vec![new_act]).unwrap());
            }
        };
        friend.points += act.points;
        friend.activities = new_friend_activites;
    });

    println!("{:?}", friend.points);
    friend.update(&connection);
    Flash::success(Redirect::to("/"), "Points added!")
}

#[post("/add/activities", data = "<form>")]
fn add_activities(
    connection: db::Connection,
    form: LenientForm<AddActivitiesForm>,
) -> Flash<Redirect> {
    println!("{:?}", form.raw_form_string());
    let mut name = form.get().actname.clone();
    name.retain(|c| c.is_alphabetic());

    let success = Activity::insert(
        &connection,
        Activity {
            id: 0, // Use zero because it isn't going to be inserted anyway
            name: name,
            description: form.get().actname.clone(),
            points: form.get().actpoints,
        },
    );
    if success {
        Flash::success(Redirect::to("/activities"), "Activity added!")
    } else {
        Flash::success(Redirect::to("/activities"), "Something went wrong :(")
    }
}

fn rocket() -> (Rocket, Option<db::Connection>) {
    let pool = db::init_pool();
    let connection = if cfg!(test) {
        Some(db::Connection(
            pool.get().expect("database connection for testing"),
        ))
    } else {
        None
    };

    let rocket = rocket::ignite()
        .manage(pool)
        .mount(
            "/",
            routes![
                index,
                activities,
                friend_page_id,
                friend_page_name,
                friends,
                add_points,
                add_activities,
                static_files
            ],
        )
        .attach(Template::fairing());

    (rocket, connection)
}

fn main() {
    rocket().0.launch();
}
