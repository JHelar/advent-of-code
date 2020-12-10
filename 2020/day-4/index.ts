import { readFileByLine } from "../utils"

const FIELDS = {
    byr: (value: string) => {
        if(value.length !== 4) return false

        const year = parseInt(value)
        return year >= 1920 && year <= 2002
    },
    iyr: (value: string) => {
        if(value.length !== 4) return false

        const year = parseInt(value)
        return year >= 2010 && year <= 2020
    },
    eyr: (value: string) => {
        if(value.length !== 4) return false

        const year = parseInt(value)
        return year >= 2020 && year <= 2030
    },
    hgt: (value: string) => {
        const unit = value.slice(value.length - 2)
        const input = parseInt(value.slice(0, value.length - 2))


        if(unit === 'cm') return input >= 150 && input <= 193
        if(unit === 'in') return input >= 59 && input <= 76

        return false
    },
    hcl: (value: string) => /#[0-9a-f]{6}/.test(value),
    ecl: (value: string) => ["amb","blu","brn","gry","grn","hzl","oth"].includes(value),
    pid: (value: string) => value.length === 9 && parseInt(value) !== NaN,
}

const getPassports = async () => {
    const passports = []
    let currentPassport: Record<string, string> = {}

    await readFileByLine('day-4/input.txt', (line) => {
        if(line.length) {
            line.split(' ').map(pair => pair.split(':')).forEach(([ key, value ]) => currentPassport[key] = value)
        } else {
            passports.push({...currentPassport})
            currentPassport = {}
        }
    })

    passports.push(currentPassport)

    return passports
}

const validatePassport = (passport: Record<string, string>) => {
    const entries = Object.entries(FIELDS)
    for (const [field, validate] of entries) {
        if(!(field in passport) || !validate(passport[field])) {
            return false
        }
    }
    return true
}

export default async () => {
    const passports = await getPassports()
    return passports.filter(validatePassport).length
}