import readline from "readline"
import fs from 'fs/promises'
import { request } from 'undici'
import { promisify } from "util"
import chalk from "chalk"

const getSessionToken = async () => {
    let token = await fs.readFile('.token').then(buffer => buffer.toString()).catch(() => null)
    if(token === null) {
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout,
            terminal: true
        })
        const question = promisify(rl.question).bind(rl)
        token = (await question(chalk.green('Please provide your session token'))) as unknown as string
        await fs.writeFile('.token', token!)
    }
    return token as string
}

export const fetchInput = async (day: number) => {
    const token = await getSessionToken();
    
    const inputUrl = `https://adventofcode.com/2022/day/${day}/input`
    const input = await request(inputUrl, {
        headers: {
            cookie: `session=${token}`
        }
    }).then((res) => res.body.text())

    return input
}