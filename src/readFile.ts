import fs from 'fs'

const buffer = fs.readFileSync('project/lines')

buffer
  .toString()
  .split('\n')
  .filter((_, i) => i % 2 === 0)
  .filter((_, i) => i > 1 && i < 4)
  .forEach((l) => console.log(l))
