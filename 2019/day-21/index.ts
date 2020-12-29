import { IntProgram } from '../intcode'
import fs from 'fs'

const input = fs.readFileSync('day-21/input.txt').toString()
const program = new IntProgram(input)

const programInputP1 = `NOT A J
NOT B T
AND D T
OR T J
NOT C T
AND D T
OR T J
WALK\n`.split('').map(m => BigInt(m.charCodeAt(0)))

const programInputP2 = `NOT C T
NOT B J
OR T J
AND D J
AND H J
NOT A T
OR T J
RUN\n`.split('').map(m => BigInt(m.charCodeAt(0)))

const error: number[] = []
while(program.running) {
    const result = program.exec(programInputP2)
    if(result !== Infinity) {
        error.push(Number(result))
    }
}

if(error.find(e => e > 65535)) {
    const hullDamage = error.find(e => e > 65535)
    console.log(String.fromCharCode(...error.filter(e => e <= 65535)))
    console.log({
        hullDamage
    })
} else {
    console.log(String.fromCharCode(...error))
}