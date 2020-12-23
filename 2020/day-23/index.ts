import { readFileWithSeparator } from '../utils'
import Node from './node'

const calcP1 = (node: Node) => {
  let curr = node!.next!!
  let result = ''
  while(true) {
    result += curr.val
    curr = curr!.next!!
    if(curr.val === 1) break
  }
  return result
}

const calcP2 = (node: Node) => {
  return node!.next!!.val * node!.next!.next!!.val
}

export default () => {
    const circleNums = readFileWithSeparator('day-23/input.txt', '\n')[0].split('').map(i => parseInt(i))
    const circleMax = 1000000
    const circleMin = 1
    let oneNode = new Node(-1)
    let head: Node = new Node(-1)
    let current: Node = new Node(-1)
    const cupMap: Record<number, Node> = {}
    let circle: Node[] = Array(circleMax).fill(0).map((_, i) => {
      const node = new Node(i < circleNums.length ? circleNums[i] : i + 1)
      if(i === 0) {
        head = node
        current = head
      } else {
        current.next = node
        current = node
      }
      if(node.val === 1) oneNode = node

      cupMap[node.val] = node
      return node
    })
    current.next = head

    let currentCup = head
    const rounds = 10000000
    let round = 1
    while(round <= rounds) {
      const pickUpNodes = [currentCup.next!!, currentCup.next!.next!!, currentCup.next!.next!.next!!]
      const pickUp = pickUpNodes.map(p => p.val)

      let destination = currentCup.val - 1
      let maxCup = circleMax
      if(pickUp.includes(maxCup)) {
        for (maxCup = circleMax; pickUp.includes(maxCup); maxCup--) {}
      }
      let minCup = circleMin
      if(pickUp.includes(minCup)) {
        for (minCup = circleMin + 1; pickUp.includes(minCup); minCup++) {}
      }

      while(true) {
        if(pickUp.includes(destination)) destination--
        if(destination < minCup) {
          destination = maxCup 
          break
        }
        if(!pickUp.includes(destination)) break
      }

      const destinationNode = cupMap[destination]!!
      if(!destinationNode) console.log({
        destination,
        minCup,
        maxCup
      })
      const pickUpNext = destinationNode!.next!!
      const nextCup = pickUpNodes[2]!.next!!

      destinationNode.next = pickUpNodes[0]
      pickUpNodes[2].next = pickUpNext
      currentCup.next = nextCup
      currentCup = nextCup

      round++
    }
    
    return calcP2(oneNode)
}
