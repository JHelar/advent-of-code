import chalk from "chalk";

const PREFIX = chalk.greenBright.bold('HoHoHo')

export const logger = {
    log: (...args: Parameters<typeof console['log']>) => {
        console.log(`${PREFIX}: ${args.at(0)}`, ...args.slice(1))
    },
    error: (...args: Parameters<typeof console['error']>) => {
        console.error(`${PREFIX}: ${chalk.red(args.at(0))}`, ...args.slice(1))
    },
    info: (...args: Parameters<typeof console['log']>) => {
        console.log(`${PREFIX}: ${chalk.yellow(args.at(0))}`, ...args.slice(1))
    },
    result: (...args: Parameters<typeof console['log']>) => {
        console.log(`${chalk.magenta(args.at(0))}`, ...args.slice(1))
    },
}