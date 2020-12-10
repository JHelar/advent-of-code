import { readFileByLine } from "../utils"

const readGroups = async () => {
    const groups = []
    let currentGroup: Record<string, number> = {}
    let memberCount = 0
    await readFileByLine('day-6/input.txt', (line) => {
        if(line.length) {
            const questions = line.split('')
            for (const question of questions) {
                if(!(question in currentGroup)) {
                    currentGroup[question] = 0
                }
                currentGroup[question] += 1
            }
            memberCount += 1
        } else {
            groups.push({...currentGroup, memberCount })
            currentGroup = {}
            memberCount = 0
        }
    })
    groups.push({...currentGroup, memberCount })
    return groups
}

export default async () => {
   const groups = await readGroups()
   return groups.reduce((sum, { memberCount, ...questions }) => {
       let groupSum = 0
        for (const count of Object.values(questions)) {
            if(count === memberCount) {
                groupSum += 1
            }
        }
        return sum + groupSum
   }, 0)
}