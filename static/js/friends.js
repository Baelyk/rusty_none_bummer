/* eslint-disable semi, no-global-assign */
/* globals friends */

let order = 'alpha-first'
friends.sort((a, b) => {
  if (a.name.toLowerCase() < b.name.toLowerCase()) {
    return -1
  } else if (a.name.toLowerCase() > b.name.toLowerCase()) {
    return 1
  } else {
    return 0
  }
})

function generateFriendsTable (friends) {
  let friendTable = document.querySelector('#friend-table')
  let tableContents = '<tr><div><td id="header-box" class="check-column"></div></td><th id="header-friend">Name</th><th id="header-points">Points</th></tr><tr><td class="check-column"><div></div></td><td id="search-row"><input type="text" id="search-input" placeholder="Search..."></td><td></td></tr>'
  friends.forEach(friend => {
    tableContents += `
    <tr id="${friend.name}-row">
      <td class="check-column"><button type="button" id="view-${friend.name}">View</button></td>
      <td>${friend.name}</td>
      <td>${friend.points}</td>
    </tr>
    `
  })
  friendTable.innerHTML = tableContents

  friends.forEach(friend => {
    let button = document.querySelector(`#view-${friend.name}`)
    let row = document.querySelector(`#${friend.name}-row`)

    // button.addEventListener('click', event => {
    //   event.stopPropagation()
    // })

    row.addEventListener('click', event => {
      window.location = `/friends/${friend.name}`
    })
  })

  let search = document.querySelector('#search-input')

  search.addEventListener('input', event => {
    let term = search.value.toLowerCase()
    let results = friends.filter(friend => friend.description.toLowerCase().includes(term))
    console.log(results)

    friends.forEach(friend => {
      if (results.indexOf(friend) === -1) {
        document.querySelector(`#${friend.name}-row`).classList.add('hide')
      } else {
        document.querySelector(`#${friend.name}-row`).classList.remove('hide')
      }
    })
  })
  search.addEventListener('keypress', event => { if (event.keyCode === 13) event.preventDefault() })

  document.querySelector('#header-box').addEventListener('click', event => {
    if (order === 'alpha-first') return
    order = 'alpha-first'
    friends.sort((a, b) => { // Sort by points, highest first
      if (a.name.toLowerCase() < b.name.toLowerCase()) {
        return -1
      } else if (a.name.toLowerCase() > b.name.toLowerCase()) {
        return 1
      } else {
        return 0
      }
    })
    generateFriendsTable(friends)
  })
  document.querySelector('#header-friend').addEventListener('click', event => {
    console.log(order)
    if (order === 'alpha-first') {
      order = 'alpha-last' // Sort by alpha, a first
      console.log(order)
      friends.sort((a, b) => { // Sort by points, highest first
        if (a.name.toLowerCase() < b.name.toLowerCase()) {
          return 1
        } else if (a.name.toLowerCase() > b.name.toLowerCase()) {
          return -1
        } else {
          return 0
        }
      })
    } else {
      order = 'alpha-first' // Sort by alpha, z first
      console.log(order)
      friends.sort((a, b) => { // Sort by points, highest first
        if (a.name.toLowerCase() < b.name.toLowerCase()) {
          return -1
        } else if (a.name.toLowerCase() > b.name.toLowerCase()) {
          return 1
        } else {
          return 0
        }
      })
    }
    generateFriendsTable(friends)
  })
  document.querySelector('#header-points').addEventListener('click', event => {
    if (order === 'points-highest') {
      order = 'points-lowest' // Sort by points, lowest first
      friends.sort((a, b) => {
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
      friends.sort((a, b) => {
        if (a.points < b.points) {
          return 1
        } else if (a.points > b.points) {
          return -1
        } else {
          return 0
        }
      })
    }
    generateFriendsTable(friends)
  })
}

generateFriendsTable(friends)
