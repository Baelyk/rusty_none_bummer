/* eslint-disable semi, no-global-assign */
/* globals friendActivities, moment */

let activityTableDiv = document.querySelector('#activity-table');
let activityTable = document.createElement('table')
activityTableDiv.innerHTML = ''
activityTableDiv.appendChild(activityTable)

console.log(friendActivities)
friendActivities = friendActivities.replace(/&quot;/gi, '"')
console.log(friendActivities)
friendActivities = JSON.parse(friendActivities)
console.log(friendActivities)
friendActivities.reverse()
console.log(friendActivities)

moment.locale('locale', {
  relativeTime: {
    s: 'A few seconds',
    m: 'A minute',
    h: 'An hour'
  },
  calendar: {
    sameElse: 'YYYY-MM-DD'
  }
})

// Friend.activities JSON
// [
//   {
//     when:  Utc::now().timestamp(),
//     activity: Activity.description,
//     previousPoints: Friend.points,
//     currentPoints: Friend.points + Activity.points
//   }, ...
// ]
let tableRows = '<tr><th>When</th><th>Activity</th><th>Previous Points</th><th>Current Points</th></tr>'
friendActivities.forEach((activity, index) => {
  let when = moment(activity.when, 'X')
  if (moment().diff(when, 'hours') < 10) {
    when = when.fromNow()
  } else {
    when = when.calendar()
  }
  tableRows += `<tr><td id="when">${when}</td><td id="activity">${activity.activity}</td><td id="current-points">${activity.previousPoints}</td><td id="current-points">${activity.currentPoints}</td></tr>`
})
activityTable.innerHTML = tableRows
