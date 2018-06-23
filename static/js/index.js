/* eslint-disable semi, no-global-assign */
/* globals activities */

// <tr>
//   <td></td>
//   <th>Activity</th>
//   <th>Points</th>
// </tr>
// <tr>
//   <td></td>
//   <td id="search-row"><input type="text" id="search-input"></td>
//   <td></td>
// </tr>
// {% for activity in activities %}
// <tr id="{{ activity.name }}-row">
//   <td><input type="checkbox" name="activity" value="{{ activity.id }}" id="{{ activity.name }}-box"></td>
//   <td><label for="{{ activity.name }}">{{ activity.description }}</label></td>
//   <td>{{ activity.points }}</td>
// </tr>
// {% endfor %}

let order = 'points-highest' // Sort by points, highest first
activities.sort((a, b) => { // Sort by points, highest first
  if (a.points < b.points) {
    return 1
  } else if (a.points > b.points) {
    return -1
  } else {
    return 0
  }
})

function generateActivitiesTable (activities) {
  let activityTable = document.querySelector('#activity-table')
  let tableContents = '<tr><div><td id="header-box" class="check-column"></div></td><th id="header-activity">Activity</th><th id="header-points">Points</th></tr><tr><td class="check-column"><div></div></td><td id="search-row"><input type="text" id="search-input" placeholder="Search..."></td><td></td></tr>'
  activities.forEach(activity => {
    tableContents += `
    <tr id="${activity.name}-row">
      <td class="check-column"><div><input type="checkbox" name="activity" value="${activity.id}" id="${activity.name}-box"></div></td>
      <td><label for="${activity.name}">${activity.description}</label></td>
      <td>${activity.points}</td>
    </tr>
    `
  })
  activityTable.innerHTML = tableContents

  activities.forEach(activity => {
    // console.log(`${activity.name}-row`)
    let box = document.querySelector(`#${activity.name}-box`)
    let row = document.querySelector(`#${activity.name}-row`)

    box.addEventListener('click', event => {
      event.stopPropagation()
    })

    row.addEventListener('click', event => {
      if (box.checked) {
        box.checked = false
      } else {
        box.checked = true
      }
    })
  })

  let search = document.querySelector('#search-input')

  search.addEventListener('input', event => {
    let term = search.value.toLowerCase()
    let results = activities.filter(activity => activity.description.toLowerCase().includes(term))
    console.log(results)

    activities.forEach(activity => {
      if (results.indexOf(activity) === -1) {
        document.querySelector(`#${activity.name}-row`).classList.add('hide')
      } else {
        document.querySelector(`#${activity.name}-row`).classList.remove('hide')
      }
    })
  })
  search.addEventListener('keypress', event => { if (event.keyCode === 13) event.preventDefault() })

  document.querySelector('#header-box').addEventListener('click', event => {
    if (order === 'points-highest') return
    order = 'points-highest'
    activities.sort((a, b) => { // Sort by points, highest first
      if (a.points < b.points) {
        return 1
      } else if (a.points > b.points) {
        return -1
      } else {
        return 0
      }
    })
    generateActivitiesTable(activities)
  })
  document.querySelector('#header-activity').addEventListener('click', event => {
    console.log(order)
    if (order === 'alpha-first') {
      order = 'alpha-last' // Sort by alpha, a first
      console.log(order)
      activities.sort((a, b) => { // Sort by points, highest first
        if (a.description.toLowerCase() < b.description.toLowerCase()) {
          return 1
        } else if (a.description.toLowerCase() > b.description.toLowerCase()) {
          return -1
        } else {
          return 0
        }
      })
    } else {
      order = 'alpha-first' // Sort by alpha, z first
      console.log(order)
      activities.sort((a, b) => { // Sort by points, highest first
        if (a.description.toLowerCase() < b.description.toLowerCase()) {
          return -1
        } else if (a.description.toLowerCase() > b.description.toLowerCase()) {
          return 1
        } else {
          return 0
        }
      })
    }
    generateActivitiesTable(activities)
  })
  document.querySelector('#header-points').addEventListener('click', event => {
    if (order === 'points-highest') {
      order = 'points-lowest' // Sort by points, lowest first
      activities.sort((a, b) => {
        if (a.points < b.points) {
          return -1
        } else if (a.points > b.points) {
          return 1
        } else {
          return 0
        }
      })
    } else {
      order = 'points-highest' // Sort by points, highest first
      activities.sort((a, b) => {
        if (a.points < b.points) {
          return 1
        } else if (a.points > b.points) {
          return -1
        } else {
          return 0
        }
      })
    }
    generateActivitiesTable(activities)
  })
}

generateActivitiesTable(activities)

document.querySelector('#submit').addEventListener('click', event => {
  let send = false
  activities.forEach(activity => {
    let box = document.querySelector(`#${activity.name}-box`)
    if (box.checked) send = true
  })
  if (!send) {
    event.preventDefault()
    document.querySelector('#help-msg').classList.remove('hide')
  }
})

document.querySelector('#view-friend').addEventListener('click', ev => {
  window.location = `friends/${document.querySelector('#select-friend').value}`
})
