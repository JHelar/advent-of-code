import { readFileWithSeparator } from '../utils'

export default () => {
  const ingredients = readFileWithSeparator('day-21/input.txt', '\n').map(ingredients => {
    const [foreign, allergens] = ingredients.split('(')

    return {
      allergens: allergens.slice(9).split(',').map(a => a.trim().replace(')','')),
      foreign: foreign.split(' ').filter(Boolean)
    }
  })

  const wordEntries = Object.entries(ingredients.reduce((acc, { allergens, foreign }) => {
    allergens.forEach(a => {
      if(!(a in acc)) {
        acc[a] = foreign
      } else {
        acc[a] = acc[a].filter(f => foreign.includes(f))
      }
    })
    return acc
  },{} as Record<string, string[]>))
  .sort((a, b) => a[1].length - b[1].length)
  
  
  let run = wordEntries[0]
  let runIndex = 0
  const usedWords: string[] = []
  const translations: Record<string, string> = {}
  while(true) { 
    const [allergen, words] = run
    if(words.length === 1 && !(words[0] in translations)) {
      translations[words[0]] = allergen
      usedWords.push(words[0])
    }
    wordEntries.forEach(entry => {
      entry[1] = entry[1].filter(w => !usedWords.includes(w))
    })
    if(usedWords.length === wordEntries.length) break
    runIndex = (runIndex + 1) % wordEntries.length
    run = wordEntries[runIndex]
  }

  // return translations
  

  return Object.entries(translations).sort((a, b) => a[1].localeCompare(b[1])).map(([word, _]) => word).join(',')
  // const missingWords: string[] = []
  // ingredients.reduce((acc, { foreign }) => {
  //   foreign.forEach(f => acc.add(f))
  //   return acc
  // }, new Set<string>()).forEach(f => {
  //   if(!(f in translations)) {
  //     missingWords.push(f)
  //   }
  // })

  // const wordMap = ingredients.reduce((acc, i) => {
  //   i.foreign.forEach(f => {
  //     if(!(f in acc)) {
  //       acc[f] = 0
  //     }
  //     acc[f]++
  //   })
  //   return acc
  // },{} as Record<string, number>)

  // return missingWords.reduce((sum, word) => sum + wordMap[word], 0)
}
