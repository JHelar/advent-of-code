import { readFileWithSeparator } from "../utils"

interface Container {
    color: string,
    count: number
}

interface Bag {
    mainColor: string,
    childOf: Array<Bag>
    containers: Record<string, Container>
}

const parseRule = (rule: string) => {
    const [mainBag, containers] = rule.split('contain')
    const main = mainBag.replace(/bag[s.]*/, '').trim()
    const hasContainers = !containers.includes('no other')
    
    const cont = (hasContainers ? containers.split(',').map(container => {
        const sanitized = container.replace(/bag[s.]*/, '').trim()
        const matchCount = sanitized.match(/^[\d]+/)
        let count = -1
        if(matchCount) {
            count = parseInt(matchCount[0])
        }
        const color = sanitized.replace(count.toString(), '').trim()

        const c: Container = {
            color,
            count
        }

        return c
    }) : []).reduce((acc, container) => {
        acc[container.color] = container
        return acc
    }, {})

    const bag: Bag = {
        mainColor: main,
        containers: cont,
        childOf: [],
    }

    return bag
}

const countParents = (bag: Bag, countedParents: Record<string, number>) => {
    let count = 0
    countedParents[bag.mainColor] = 1
    for (const parent of bag.childOf) {
        if(!(parent.mainColor in countedParents)) {
            
            count += countParents(parent, countedParents)
        }
    }
    return count
}

const createBag = (bag: Bag, bags: Record<string, Bag>) => {
    let cost = 0
    for (const container of Object.values(bag.containers)) {
        const containerBag = bags[container.color]
        if(containerBag) {
            cost += container.count + container.count * createBag(containerBag, bags)
        }
    }
    return cost
}

export default () => {
    const rules = readFileWithSeparator('day-7/input.txt', '\n').map(parseRule)
    rules.forEach((rule, i) => {
        const containedIn = rules.filter(r => rule.mainColor in r.containers)
        rule.childOf = containedIn
    })
    const bagMap: Record<string, Bag> = rules.reduce((bags, bag) => {
        bags[bag.mainColor] = bag
        return bags
    }, {})
    
    const shinyGoldBag = bagMap['shiny gold']
    const cost = createBag(shinyGoldBag, bagMap)

    return cost
}