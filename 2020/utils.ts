import { readFileSync } from 'fs'
import { resolve } from 'path'

export const readFileWithSeparator = (file: string, sep: string) => readFileSync(resolve(__dirname, file)).toString().split(sep)