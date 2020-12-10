import { constants } from 'buffer'
import { readFileWithSeparator } from '../utils'

const part1 = (adapters: number[]) => {
    const adapterMap: Record<number, number> = {}
    let rating = 0
    for (const adapter of adapters) {
        const diff = adapter - rating
        rating = adapter
        if(!(diff in adapterMap)) adapterMap[diff] = 0
        adapterMap[diff] += 1
    }
    adapterMap[3]++

    return adapterMap[1] * adapterMap[3]
}

const setAdaptersForRating = (adapters: number[], rating: number, ratingIndex: number, adapterBucket: Record<number, number[]>) => {
    const searchSpace = adapters.slice(ratingIndex, ratingIndex + 3)
    const ratingAdapters = searchSpace.filter(sp => sp - rating < 4 && sp - rating > 0)
    adapterBucket[rating] = ratingAdapters

    for (let i = 0; i < ratingAdapters.length; i++) {
        const adapter = ratingAdapters[i];
        if(!(adapter in adapterBucket)) {
            setAdaptersForRating(adapters, adapter, ratingIndex + 1 + i, adapterBucket)
        }
    }
}

export default () => {
    const adapters = readFileWithSeparator('day-10/input.txt', '\n').map(adapter => parseInt(adapter)).sort((a, b) => a - b) as number[]
    

    let rating = 0
    const adapterBucket: Record<number, number[]> = {}
    setAdaptersForRating(adapters, 0, 0, adapterBucket)
    console.log('Set ratings')
    const combinations: Record<number, number> = {}
    const adapterKeys = Object.keys(adapterBucket).reverse()
    
    for (const adapter of adapterKeys) {
        let adapterCombinations = 0
        adapterBucket[adapter].forEach(rating => {
            adapterCombinations += combinations[rating]
        })
        combinations[adapter] = adapterCombinations === 0 ? 1 : adapterCombinations
    }
    return combinations[0]
}