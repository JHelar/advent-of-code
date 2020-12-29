import fs from 'fs'
import { chunk } from '../day-20/util'

const deckCache: Map<number[], boolean> = new Map()

const COMMANDS: Record<string, (command: string) => () => [bigint, bigint]> = {
    'deal into new stack': (command) => () => {
        return [-1n, -1n]
    },
    'cut': (command) => () => {
        const count = BigInt(command.slice(4))
        return [count, 0n]
    },
    'deal with increment': (command) => () => {
        const increment = BigInt(command.slice(20))
        return [1n, -increment]
    }
}

const inv = (a: bigint, n: bigint) => BigInt(Math.pow(Number(a), Number(n) - 2)) % n

const shuffles = fs.readFileSync('day-22/input.txt').toString().split('\n').map(command => {
    const commandKey = Object.keys(COMMANDS).find(c => command.startsWith(c))
    return COMMANDS[commandKey!](command)
})

const MOD = 119315717514047n
const GOAL = 2020n
let a = 1n
let b = 0n
shuffles.forEach(shuffle => {
    const [la, lb] = shuffle()
    a = (la * a) % MOD
    b = (la * b + lb) % MOD
})
const M = 101741582076661
const Ma = BigInt(Math.pow(Number(a), Number(M))) % MOD
const Mb = (b * (Ma - 1n) * inv(a - 1n, MOD)) % MOD

console.log(((GOAL - Mb) * inv(Ma, MOD)) % MOD)