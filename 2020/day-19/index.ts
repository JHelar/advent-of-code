import { readFileByLine } from '../utils'

interface Rule {
  values?: number[][]
  value?: string
  result: string
  reacured: number
  no: string
}

type RuleMap = Record<string, Rule>

const parseInput = async (path: string) => {
  const rules: RuleMap = {}
  const messages: string[] = []
  let parseRules = true
  await readFileByLine(path, (line) => {
    if(!line) {
      parseRules = false
    } else if (parseRules) {
      const [ruleNo, value] = line.split(': ')
      const isEndvalue = value?.includes('"')
      if(!isEndvalue) {
        const values = value?.split(' | ').map(parts => parts.split(' ').map(r => parseInt(r)))
        rules[ruleNo] = {
          values,
          result: '',
          reacured: 0,
          no: ruleNo
        }
      } else {
        rules[ruleNo] = {
          value: value?.replace(/"/g, ''),
          result: '',
          reacured: 0,
          no: ruleNo
        }
      }
    } else {
      messages.push(line)
    }
  })

  return {
    rules,
    messages
  }
}

const runRule = (rule: Rule, map: RuleMap) => {
  if(rule.result) return rule.result

  if(rule.value) {
    rule.result = rule.value
    return rule.value
  }

  if(rule.values) {
    rule.result = `(${rule.values.map(part => part.reduce((str, ruleNo) => str + runRule(map[ruleNo], map), "")).join('|')})`
    return rule.result
  }
  
  return ''
}

export default async () => {
  const { rules, messages } = await parseInput('day-19/input.txt')

  runRule(rules['0'], rules)
  
  rules['8'].values = [[42], [42, 8]]
  rules['11'].values = [[42, 31], [42, 11, 31]]
  runRule(rules['42'], rules)
  runRule(rules['31'], rules)

  const rule = new RegExp('^(?<group42>('+rules[42].result+')+)(?<group31>('+rules[31].result+')+)$');
  let sum=0;
  for (const message of messages) {
      const matches = rule.exec(message);
      if(matches) {
          const {groups} = matches;
          const matches42 = groups?.group42?.match(new RegExp(rules[42].result, 'g'))?.length || 0;
          const matches31 = groups?.group31?.match(new RegExp(rules[31].result, 'g'))?.length || 0;
          if(matches42 > matches31) {
              sum++;
          }
      }
  }
  return sum
}
