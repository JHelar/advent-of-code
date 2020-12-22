import { readFileByLine } from '../utils'

interface Player {
  no: number
  cards: number[]
}

const playCombat = (p1: Player, p2: Player) => {
  while(true) {
    console.log("Player 1's deck: " + p1.cards.join(','))
    console.log("Player 2's deck: " + p2.cards.join(','))
    const p1Card = p1.cards.shift()
    const p2Card = p2.cards.shift()

    if(p1Card && p2Card) {
      console.log('Player 1 plays: ' + p1Card)
      console.log('Player 2 plays: ' + p2Card)
      if(p1Card > p2Card) {
        p1.cards.push(...[p1Card, p2Card])
        console.log('Player 1 wins the round!')
      } else {
        p2.cards.push(...[p2Card, p1Card])
        console.log('Player 2 wins the round!')
      }
    }

    if(!p1.cards.length) {
      console.log('Player 2 wins the game!')
      return calcScore(p2)
    }
    if(!p2.cards.length) {
      console.log('Player 1 wins the game!')
      return calcScore(p1)
    }
  }
}

const getCacheString = (p1: Player, p2: Player) => p1.cards.join(',') + '|' + p2.cards.join(',')

const recursiveCombat = (() => {
  const gameCache: Record<string, number> = {}
  return (p1: Player, p2: Player, gameNo: number = 1) => {
    const roundCache: Record<string, boolean> = {}
    const cacheString = getCacheString(p1, p2)
    if(cacheString in gameCache) {
      console.log('Player ' + gameCache[cacheString] + ' wins the game ' + gameNo + '!')
      return gameCache[cacheString]
    }

    let winner = 0
    let round = 1
    while(true) {
      const roundCacheString = getCacheString(p1, p1)
      if(roundCacheString in roundCache) {
        return 1
      }
      roundCache[roundCacheString] = true
      console.log('--------------------------')
      console.log("Player 1's deck: " + p1.cards.join(','))
      console.log("Player 2's deck: " + p2.cards.join(','))
      const p1Card = p1.cards.shift()!!
      const p2Card = p2.cards.shift()!!
      console.log('Player 1 plays: ' + p1Card)
      console.log('Player 2 plays: ' + p2Card)

      if(p1.cards.length >= p1Card && p2.cards.length >= p2Card) {
        console.log('Playing a sub-game to get winner....')
        winner = recursiveCombat({ ...p1, cards: p1.cards.slice(0, p1Card) }, {...p2, cards: p2.cards.slice(0, p2Card) }, gameNo + 1)
      
      } else if(p1Card > p2Card) {
        winner = 1
      } else {
        winner = 2
      }
      console.log('Player ' + winner + ' wins round ' + round + ' of game ' + gameNo + '!')
      console.log('--------------------------')
      round++

      if(winner === 1) {
        p1.cards.push(...[p1Card, p2Card])
      } else {
        p2.cards.push(...[p2Card, p1Card])
      }
  
      if(!p1.cards.length) {
        console.log('Player 2 wins game ' + gameNo + '!')
        gameCache[cacheString] = 2
        return 2
      }
      if(!p2.cards.length) {
        console.log('Player 1 wins the game ' + gameNo + '!')
        gameCache[cacheString] = 1
        return 1
      }
    }
  }
})()

const calcScore = ({ cards }: Player) => cards.reverse().reduce((sum, card, i) => sum + (card * (i + 1)), 0)

export default async () => {
  const p1: Player = {
    no: 1,
    cards: []
  }
  const p2: Player = {
    no: 2,
    cards: []
  }
  let currentPlayer: Player;

  await readFileByLine('day-22/test.txt', (line) => {
    if(line === 'Player 1:') {
      currentPlayer = p1
    } else if(line === 'Player 2:') {
      currentPlayer = p2
    } else if(line) {
      currentPlayer.cards.push(parseInt(line))
    }
  })

  console.log = () => {}
  recursiveCombat(p1, p2, 1)

  if(p1.cards.length) {
    return calcScore(p1)
  } else {
    return calcScore(p2)
  }
}
