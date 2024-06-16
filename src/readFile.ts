import fs from 'fs'

const buffer = fs.readFileSync('project/lines')

const lines = buffer.toString().split(`\n`).length

console.log(lines)
