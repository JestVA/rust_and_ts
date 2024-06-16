import fs from 'fs'

const buffer = fs.readFileSync('project/lines')

const lines = buffer
  .toString()
  .split('\n')
  .filter((_, i) => i % 2 === 0)
  .forEach((l) => console.log(l))

console.log(lines)
