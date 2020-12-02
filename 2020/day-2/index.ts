import { readFileWithSeparator } from "../utils"

const parsePasswordRule = (input: string) => {
    const [range, delimiter, password] = input.split(' ')

    const [min, max] = range.split('-').map(r => parseInt(r))

    return {
        min,
        max,
        delimiter: delimiter.replace(':', ''),
        password
    }
}

const validatePassword = ({ min, max, delimiter, password }: ReturnType<typeof parsePasswordRule>) => {
    const parsedPassword = password.split('').reduce((acc, c, index) => {
        if(!(c in acc)) {
            acc[c] = []
        }
        acc[c].push(index + 1)
        return acc
    }, {} as Record<string, number[]>)

    if(!(delimiter in parsedPassword)) return false

    return parsedPassword[delimiter].includes(min) !== parsedPassword[delimiter].includes(max)
}

export default () => readFileWithSeparator('day-2/input.txt', '\n').map(parsePasswordRule).filter(validatePassword).length