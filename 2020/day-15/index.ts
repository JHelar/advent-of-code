import { readFileWithSeparator } from '../utils'

const runGame = (runs: number, startNums: number[]) => {
  const numBank: Record<number, { first: number, last: number }> = {}
  startNums.forEach((num, turnIndex) => {
    numBank[num] = {
      last: -1,
      first: turnIndex + 1
    }
  })

  let lastSpeak = startNums[startNums.length - 1]
  for (let turn = startNums.length + 1; turn <= runs; turn++) {
    const { last, first } = numBank[lastSpeak]
    let speak = 0
    if(last > -1) {
      speak = first - last
    }
    
    if(speak in numBank) {
      numBank[speak] = {
        last: numBank[speak].first,
        first: turn
      }
    } else {
      numBank[speak] = {
        last: -1,
        first: turn
      }
    }
    lastSpeak = speak
  }
  return lastSpeak
}

const part1 = () => {
  const startNums = readFileWithSeparator('day-15/input.txt', ',').map(i => parseInt(i))
  return runGame(2020, startNums)
}


export default async () => {
  const startNums = readFileWithSeparator('day-15/input.txt', ',').map(i => parseInt(i))
  return runGame(30000000, startNums)
}