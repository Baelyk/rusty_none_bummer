use chrono::offset::Utc;
use chrono::DateTime;
use diesel;
use diesel::pg::types::sql_types::Jsonb;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::http::RawStr;
use schema::activities;
use schema::activities::columns::name as act_col_name;
use schema::friends;
use schema::friends::columns::activities as friend_activities;
use schema::friends::columns::id as friend_id;
use schema::friends::columns::points as friend_points;
use serde_json::Value as JSON;
use std::iter::FromIterator;

#[derive(FromForm)]
pub struct AddPointsForm {
    pub friend: i32,
}

#[derive(FromForm)]
pub struct AddActivitiesForm {
    pub actname: String,
    pub actpoints: i32,
}

#[derive(Clone, Queryable, Serialize, Insertable)]
#[table_name = "activities"]
pub struct Activity {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub points: i32,
}

impl Activity {
    pub fn all(connection: &PgConnection) -> Vec<Activity> {
        activities::table
            .order(activities::id.desc())
            .load::<Activity>(connection)
            .unwrap()
    }
    pub fn get_by_id(connection: &PgConnection, id: i32) -> Option<Activity> {
        let found_acts = activities::table
            .find(id)
            .load::<Activity>(connection)
            .unwrap();
        if found_acts.len() == 1 {
            Some(found_acts[0].clone())
        } else {
            None
        }
    }
    pub fn insert(connection: &PgConnection, act: Activity) -> bool {
        diesel::insert_into(activities::table)
            .values((
                activities::name.eq(&act.name),
                activities::description.eq(&act.description),
                activities::points.eq(&act.points),
            ))
            .execute(connection)
            .is_ok()
    }
}

#[derive(Clone, Queryable, Serialize, Insertable)]
#[table_name = "friends"]
pub struct Friend {
    pub id: i32,
    pub name: String,
    pub points: i32,
    pub activities: Option<JSON>,
}

impl Friend {
    pub fn all(connection: &PgConnection) -> Vec<Friend> {
        friends::table
            .order(friends::name)
            .load::<Friend>(connection)
            .unwrap()
    }
    pub fn get_by_id(connection: &PgConnection, id: i32) -> Option<Friend> {
        let found_acts = friends::table.find(id).load::<Friend>(connection).unwrap();
        if found_acts.len() == 1 {
            Some(found_acts[0].clone())
        } else {
            None
        }
    }
    pub fn get_id_from_name(connection: &PgConnection, name: String) -> i32 {
        friends::table
            .filter(friends::name.eq(name))
            .order(friends::name)
            .select(friends::id)
            .first(connection)
            .expect("expected valid id")
    }
    pub fn update(&self, connection: &PgConnection) -> bool {
        println!("Updating {}", self.name);
        diesel::update(friends::table.find(self.id))
            .set((
                friend_points.eq(self.points),
                friend_activities.eq(self.clone().activities),
            ))
            .execute(connection)
            .is_ok()
    }
}

// Friend.activities JSON
// [
//   {
//     when:  Utc::now().timestamp(),
//     activity: Activity.description,
//     previousPoints: Friend.points,
//     currentPoints: Friend.points + Activity.points
//   }, ...
// ]
pub type ActivitiesJSON = Vec<ActivityJSON>;
#[derive(Deserialize, Serialize)]
pub struct ActivityJSON {
    pub when: i64,
    pub activity: String,
    pub previousPoints: i32,
    pub currentPoints: i32,
}
