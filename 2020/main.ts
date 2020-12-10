const [file, path, day] = process.argv

type Day = {
    default: () => any
}

;(async () => {
    const runner = require(`./${day}`) as Day
    if(runner) {
        console.log(await runner.default())
    }
})()