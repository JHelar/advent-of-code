import { parse } from 'path'
import { readFileWithSeparator } from '../utils'

const MUL = '*'
const ADD = '+'
const SUB = '-'
const DIV = '/'
const LPAREN = '('
const RPAREN = ')'
const INTEGER = 'number'
const EOF = 'EOF'

const HIGHEST_PRIO_OPERATIONS = [
  ADD
]

const PRIO_OPERTATIONS = [
  MUL,
  SUB,
  DIV
]

type TokenType = typeof MUL | typeof ADD | typeof SUB | typeof DIV | typeof LPAREN | typeof RPAREN | typeof INTEGER | typeof EOF

interface Token {
  type: TokenType,
  value: string | number,
  toString: () => string
}

interface BinOp {
  left: Token | BinOp
  op: Token
  right: Token | BinOp
}

const Token = (type: TokenType, value: string | number): Token => ({
  type,
  value,
  toString: () => `Token(${type}, ${value})`
})

const BinOp = (left: Token | BinOp, op: Token, right: Token | BinOp): BinOp => ({
  left,
  op,
  right
})

const createLexer = (value: string) => {
  const input = value.split('').filter(i => i !== ' ')
  let tokenPos = 0

  const integer = () => {
    let int = '' 
    let value = input[tokenPos]
    while(!isNaN(value as any)) {
      int += value
      tokenPos++
      value = input[tokenPos]
    }
    return parseInt(int)
  }

  const nextToken = () => {
    const value = input[tokenPos]

    if(!value) return Token(EOF, -1)

    if(isNaN(value as any)) {
      tokenPos++
      return Token(value as TokenType, value)
    }
    return Token(INTEGER, integer())
  }


  return {
    nextToken
  }
}

const createAST = (value: string) => {
  const lexer = createLexer(value)
  
  let currentToken: Token = lexer.nextToken()

  const eat = () => {
    currentToken = lexer.nextToken()
    return currentToken
  }

  const factor = (): Token | BinOp => {
    if(currentToken.type === INTEGER) {
      const node = { ...currentToken }
      eat()
      return node
    } else if(currentToken.type === LPAREN) {
      eat()
      const node = expr()
      eat()
      return node
    }
    return currentToken
  }

  const term = (): Token | BinOp => {
    let node = factor()

    while(HIGHEST_PRIO_OPERATIONS.includes(currentToken.type)) {
      const token = currentToken
      if(HIGHEST_PRIO_OPERATIONS.includes(token.type)) {
        eat()
      }
      node = BinOp({...node}, {...token}, {...factor()})
    }

    return node
  } 

  const expr = () => {
    let node = term()

    while(PRIO_OPERTATIONS.includes(currentToken.type)) {
      const token = currentToken
      if(PRIO_OPERTATIONS.includes(token.type)) {
        eat()
      }
      node = BinOp({...node}, {...token}, {...term()})
    }

    return node
  }

  const parse = () => {
    return expr()
  }

  return {
    parse
  }
} 

const solve = (node: BinOp | Token): number => {
  if('type' in node) {
    if(typeof node.value === 'number') {
      return node.value
    }
  }
  const binOp = node as BinOp
  const left = solve(binOp.left)
  const right = solve(binOp.right)

  switch(binOp.op.type) {
    case ADD:
      return left + right
    case SUB:
      return left - right
    case MUL:
      return left * right
    case DIV:
      return left / right
    default:
      console.log('NOT VALID OPERATOR')
      return Infinity
  }
}

export default async () => {
  return readFileWithSeparator('day-18/input.txt', '\n').map(input => createAST(input)).reduce((sum, ast) => {
    const tree = ast.parse()
    const result = solve(tree)
    return sum + result
  }, 0)
  
}
