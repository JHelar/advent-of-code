import Days, { Day } from './days'

const [file, path, day] = process.argv

;(async () => {
    const solution = (Days as Record<string, Day>)[day]
    if(solution) {
        console.log(await solution())
    }
})()