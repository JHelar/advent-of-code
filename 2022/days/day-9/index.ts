import fs from "fs/promises";
import path from "path";

const readInput = async () => {
    const content = await fs.readFile(path.resolve(__dirname, "input.txt"))
    return content.toString('utf8')
}

const part1 = async () => {
    return null
}

const part2 = async () => {
    return null
}

// Generated code to run on cli
(async () => {
    const [,,part] = process.argv
    if(part === '1') {
        const result = await part1();
        console.log(result)
    }
    if(part === '2') {
        const result = await part2();
        console.log(result)
    }
})()