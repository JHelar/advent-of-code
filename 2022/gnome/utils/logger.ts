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
    divider: () => console.log('.:*~*:._.:*~*:._.:*~*:._.:*~*:._.:*~*:._.:*~*:._.:*~*:._.:*~*:.'.replaceAll('*', chalk.yellowBright('*')).replaceAll('~', chalk.redBright('~')).replaceAll(/[:._]/g, (str) => chalk.green(str))),
    headline: (text: string) => console.log(`.:*~*:._.:*~*:._.:*~*:._.:${chalk.bgYellow.black(` ${text} `)}:._.:*~*:._.:*~*:._.:*~*:.`.replaceAll('*', chalk.yellowBright('*')).replaceAll('~', chalk.redBright('~')).replaceAll(/[:._]/g, (str) => chalk.green(str))),
}