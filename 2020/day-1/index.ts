import { readFileWithSeparator } from "../utils";


export default () => {
    const input = readFileWithSeparator('day-1/input.txt', '\n').map(i => parseInt(i))
    const values: Record<number, number> = {}

    for (const value of input) {
        const lookFor = 2020 - value

        for (const innerValue of input) {
            const remainder = lookFor - innerValue

            if(remainder in values) {
                return value * innerValue * remainder
            }
        }
        values[value] = 1
    }
    return -1
}