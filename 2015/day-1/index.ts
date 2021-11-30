const input = process.env.INPUT!

const values: Record<string, number> = {
    '(': 1,
    ')': -1
}

const p1 = () => input.split('').reduce((sum, p) => sum + values[p], 0)
const p2 = () => {
    let sum = 0
    let indx = 0
    for (const p of input.split('')) {
        sum += values[p]
        indx++;
        if(sum === -1) break
    }
    return indx
}

console.log({
    result: p2()
})