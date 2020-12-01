import Days from './days'

const [file, path, day] = process.argv

const solution = Days[day]

if(solution) {
    console.log(solution())
}