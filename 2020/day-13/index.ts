import { readFileWithSeparator } from '../utils'

const part1 = () => {
    const [minTimeStr, busses] = readFileWithSeparator('day-13/input.txt', '\n')
    const minTime = parseInt(minTimeStr)
    const bussIds = busses.split(',').filter(b => b !== 'x').map(b => parseInt(b))
    
    const [best, ...next] = bussIds.map(buss => {
        let next = ((minTime/buss) | 1) * buss
        if(next < minTime) {
            next += buss
        }
        return {
            id: buss,
            next
        }
    }).sort((a, b) => a.next - b.next)
    
    return (best.next - minTime) * best.id
}

interface Bus {
    time: number,
    ID: number,
    multiplier: number
}

const part2 = () => {
    const [minTimeStr, busString] = readFileWithSeparator('day-13/input.txt', '\n')
    const [firstBus, ...buses] = busString
    .split(',')
    .map((n, i) => [parseInt(n, 10), i])
    .filter(([n]) => !Number.isNaN(n));

  let multiplier = firstBus[0];
  let i = 0;

  buses.forEach(([bus, busIndex]) => {
    while (true) {
      if ((i + busIndex) % bus === 0) {
        multiplier *= bus;
        break;
      }
      i += multiplier;
    }
  });

  return i;
}

export default () => {
    return part2()
}