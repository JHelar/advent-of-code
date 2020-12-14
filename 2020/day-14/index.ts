import { readFileByLine } from '../utils'

interface Instruction {
  address: number
  binAddress: string
  intValue: number
  binValue: string
  mask: string
}

const getValueFromInstruciton = ({ binValue, mask }: Instruction) => {
  let restultBin = ''
  for (let i = 0; i < 36; i++) {
    const m = mask[i]
    const b = binValue[i]

    if(m === 'X') restultBin += b
    else restultBin += m
  }

  return parseInt(restultBin, 2)
}

const getAddressesFromInstruction = ({ binAddress, mask }: Instruction) => {
  let restultBin = ''
  const xPositions = []
  const addresses: Array<number> = []
  for (let i = 0; i < 36; i++) {
    const m = mask[i]
    const b = binAddress[i]

    if(m === '0') restultBin += b
    else {
      if(m === 'X') {
        xPositions.push(i)
      }
      restultBin += m
    }
  }
  for(let j = 0; j < Math.pow(2, xPositions.length); j++) {
    const binValue = j.toString(2).padStart(xPositions.length, '0')
    const addressBin = xPositions.reduce((str, position, pI) => str.substring(0, position) + binValue[pI] + str.substring(position + 1), restultBin)
    const address = parseInt(addressBin, 2)
    addresses.push(address)
  }

  return addresses
}

const getInstructions = async (path: string) => {
  const instructions: Array<Instruction> = []
  let currentMask = ''
  await readFileByLine(path, (line) => {
    if(line.includes('mask')) {
      currentMask = line.split('=')[1].trim()
    } else {
      const addressStart = line.indexOf('[')
      const addressEnd = line.indexOf(']')
      const address = line.slice(addressStart + 1, addressEnd)

      const [_, value] = line.split('=')
      const intValue = parseInt(value.trim())
      const binValue = intValue.toString(2).padStart(36, '0')

      instructions.push({
        address: parseInt(address),
        binAddress: parseInt(address).toString(2).padStart(36, '0'),
        intValue,
        binValue,
        mask: currentMask
      } as Instruction)
    }
  })

  return instructions
}

const part1 = async () => {
  const instructions = await getInstructions('day-14/input.txt')
  const memory: Record<number, number> = {}
  instructions.forEach(writer => {
    const value = getValueFromInstruciton(writer)
    memory[writer.address] = value
  })

  return Object.values(memory).reduce((sum, val) => sum + val, 0)
}

export default async () => {
  const instructions = await getInstructions('day-14/input.txt')
  const memory: Record<number, number> = {}
  instructions.forEach(instruction => {
    const addresses = getAddressesFromInstruction(instruction)
    addresses.forEach(address => memory[address] = instruction.intValue)
  })

  return Object.values(memory).reduce((sum, val) => sum + val, 0)
}