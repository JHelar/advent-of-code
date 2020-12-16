import { readFileByLine } from '../utils'
 
type Ticket = Array<{ val: number, rules: string[] }>
interface Rule {
  name: string
  minRange: number[]
  maxRange: number[]
}
 
 
const readInput = async (path: string) => {
  const rules: Array<Rule> = []
  const nearbyTickets: Array<Ticket> = []
  let myTicket: Ticket = []
  let readState = 0
  await readFileByLine(path, (line) => {
    if(line === '')  {
      readState++
    } else if(readState === 0) {
      const [ruleName, ruleRanges] = line.split(': ')
      const [minRange, maxRange] = ruleRanges.split(' or ').map(range => range.split('-').map(r => parseInt(r)))
      rules.push({
        name: ruleName,
        minRange,
        maxRange
      })
    } else if (readState === 1) {
      if(line !== 'your ticket:') {
        const values = line.split(',').map(v => parseInt(v)).map((v) => {
          const valid = rules.filter(rule => isFieldValid(v, rule)).map(({ name }) => name)
          return {
            val: v,
            rules: valid
          }
        })
 
        myTicket = values
      }
    } else if (readState === 2) {
      if(line !== 'nearby tickets:') {
        let isValid = true
        const values = line.split(',').map(v => parseInt(v)).map((v) => {
          const valid = rules.filter(rule => isFieldValid(v, rule)).map(({ name }) => name)
          if(!valid.length) isValid = false
          return {
            val: v,
            rules: valid
          }
        }, {} as Ticket)
 
        if(isValid) {
          nearbyTickets.push(values)
        }
      }
    }
  })
 
  return {
    rules,
    nearbyTickets,
    myTicket
  }
}
 
const isFieldValid = (v: number, { minRange, maxRange }: Rule) => {
  const [minMin, minMax] = minRange
  const [maxMin, maxMax] = maxRange
 
  const min = v >= minMin && v <= minMax
  const max = v >= maxMin && v <= maxMax
 
  return min || max
}
 
const findRuleFor = (valueIndex: number, takenRules: Array<string>, myTicket: Ticket, ruleMap: Record<string, Set<number>>): null | {index: number, rule: string}[] => {
  if(valueIndex === myTicket.length) {
    return []
  }
 
  const { rules } = myTicket[valueIndex]
 
  const lookupRules = rules.filter(r => !takenRules.includes(r))
  if(!lookupRules.length) return null
 
  for (const rule of lookupRules) {
    const isValidRule = ruleMap[rule].has(valueIndex)
 
    if(isValidRule) {
      const assumptions = findRuleFor(valueIndex + 1, [...takenRules, rule], myTicket, ruleMap)
      if(assumptions) {
        return [...assumptions, { index: valueIndex, rule }]
      }
    }
  }
 
  return null
}
 
export default async () => {
  const {
    myTicket,
    nearbyTickets,
    rules
  } = await readInput('day-16/input.txt')
  const ruleMap = myTicket.reduce((acc, { val, rules }, i) => {
    const validRules = rules.filter(rule => nearbyTickets.every(nt => nt[i].rules.includes(rule)))
    validRules.forEach(rule => {
      if(!(rule in acc)) {
        acc[rule] = new Set<number>()
      }
      acc[rule].add(i)
    })
    return acc
  }, {} as Record<string, Set<number>>)
 
  const result = findRuleFor(0, [], myTicket, ruleMap)
  console.log(result)
  return result?.reduce((sum, { index, rule }) => {
    if(rule.startsWith('departure')) {
      sum *= myTicket[index].val
    }
    return sum
  },1)
}