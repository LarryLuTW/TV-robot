// Type "Hello World" then press enter.
const robot = require('robotjs')
const express = require('express')

const app = express()

app.use(express.static('.'))

const timeout = 100

app.post('/api/space', (req, res) => {
  robot.keyTap('space')
  setTimeout(() => {
    res.end('success')
  }, timeout)
})

app.post('/api/right', (req, res) => {
  robot.keyTap('right')
  setTimeout(() => {
    res.end('success')
  }, timeout)
})

app.post('/api/left', (req, res) => {
  robot.keyTap('left')
  setTimeout(() => {
    res.end('success')
  }, timeout)
})

app.listen(3000)
