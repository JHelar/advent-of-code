import { readFileSync, createReadStream } from 'fs'
import { resolve } from 'path'
import { createInterface } from 'readline'

export const readFileWithSeparator = (file: string, sep: string) => readFileSync(resolve(__dirname, file)).toString().split(sep)

export const readFileByLine = (file: string, lineCallback: (line: string) => void) => new Promise(res => {
    const readInterface = createInterface({
        input: createReadStream(resolve(__dirname, file)),
    })

    readInterface.on('line', lineCallback)
    readInterface.on('close', res)
})