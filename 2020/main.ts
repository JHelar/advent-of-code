import Days from './days'

const [file, path, day] = process.argv

;(async () => {
    const solution = Days[day]
    if(solution) {
        console.log(await solution())
    }
})()