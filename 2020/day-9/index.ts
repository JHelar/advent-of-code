import { readFileWithSeparator } from '../utils'

const validate = (num: number, preamble: number[]) => {
    const sortedPreamble = [...preamble].filter(pNum => pNum < num).sort()
    
    for (let i = 0; i < sortedPreamble.length; i++) {
        const pFront = sortedPreamble[i];
        
        for (let j = sortedPreamble.length - 1; j >= 0; j--) {
            const pBack = sortedPreamble[j];
            
            if((pFront + pBack) === num) {
                return true
            }
        }
    }
    return false
}

const getEncryptionKey = (inputs: number[]) => {
    const preambleLength = 25
    let preambleOffset = 0

    for (let i = preambleLength; i < inputs.length; i++) {
        const num = inputs[i];
        const preamble = inputs.slice(preambleOffset, preambleOffset + preambleLength)
        if(!validate(num, preamble)) return num
        preambleOffset++
    }
    return -1
}

export default () => {
    const inputs = readFileWithSeparator('day-9/input.txt', '\n').map(i => parseInt(i))
    const key = getEncryptionKey(inputs)
    
    let contiguousSet = []
    let currentSum = 0

    for (let i = 0; i < inputs.length; i++) {
        for (let j = i; j < inputs.length; j++) {
            const num = inputs[j]
            currentSum += num
            if(currentSum > key) {
                currentSum = 0
                contiguousSet = []
                break;
            }
            else if(currentSum === key) {
                contiguousSet.push(num)
                const min = Math.min(...contiguousSet)
                const max = Math.max(...contiguousSet)
                return min + max
            }
            else {
                contiguousSet.push(num)
            }
        }
    }
    return -1
}