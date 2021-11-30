import fs from 'fs'
import path from 'path'

const input = fs.readFileSync(path.resolve(__dirname, process.argv[2], process.argv[3] === 'test' ? 'test.txt' : 'input.txt')).toString()
process.env.INPUT = input
require('./' + process.argv[2]);