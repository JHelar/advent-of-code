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

const OPERATIONS = [
  MUL,
  ADD,
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
  const input = value.split(' ').reduce((acc, i) => {
    if(i.includes('(')) return [...acc, ...i.replace('(', '( ').split(' ')]
    if(i.includes(')')) return [...acc, ...i.replace(')', ' )').split(' ')]
    return [
      ...acc,
      i
    ]
  },[] as string[])
  let tokenPos = -1
  const nextToken = () => {
    tokenPos++
    const value = input[tokenPos]

    if(!value) return Token(EOF, -1)

    if(!isNaN(value as any)) return Token(INTEGER, value)
    else return Token(value as TokenType, value)
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
    console.log({
      currentToken
    })
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

    while(OPERATIONS.includes(currentToken.type)) {
      const token = currentToken
      if(OPERATIONS.includes(token.type)) {
        eat()
      }
      node = BinOp({...node}, {...token}, {...factor()})
    }

    return node
  } 

  const expr = () => {
    return term()
  }

  const parse = () => {
    return expr()
  }

  return {
    parse
  }
}

export default async () => {
  const ast = readFileWithSeparator('day-18/test.txt', '\n').map(input => createAST(input))[0]
  if(ast) {
    const tree = ast.parse()

    console.log(tree)
  }
  
}