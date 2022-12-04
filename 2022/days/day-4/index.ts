import fs from "fs/promises";
import path from "path";

type Section = [number, number]
type Pair = [Section, Section]

const readInput = async () => {
    const content = await fs.readFile(path.resolve(__dirname, "input.txt"))
    return content.toString('utf8')
}

const overlaps = (s1: Section, s2: Section) => s1[0] <= s2[0] && s1[1] >= s2[1]
const isOverlaped = ([s1, s2]: Pair) => overlaps(s1, s2) || overlaps(s2, s1)

const partialOverlaps = (s1: Section, s2: Section) => s1[0] <= s2[0] && s1[1] >= s2[0] || s1[0] <= s2[1] && s1[1] >= s2[1]
const isPartiallyOverlaped = ([s1, s2]: Pair) => partialOverlaps(s1, s2) || partialOverlaps(s2, s1)

const part1 = async () => {
    const pairs = (await readInput()).split('\n').filter(Boolean).map((line) => {
        const [p1, p2] = line.split(',')

        return [p1.split('-').map(Number) as Section, p2.split('-').map(Number) as Section] as Pair
    }).filter(isOverlaped)

    return `Result: ${pairs.length} overlaps`
}

const part2 = async () => {
    const pairs = (await readInput()).split('\n').filter(Boolean).map((line) => {
        const [p1, p2] = line.split(',')

        return [p1.split('-').map(Number) as Section, p2.split('-').map(Number) as Section] as Pair
    }).filter(isPartiallyOverlaped)

    return `Result: ${pairs.length} overlaps`
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