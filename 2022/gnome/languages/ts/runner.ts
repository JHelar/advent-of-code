import path from 'path'
import { runProgram } from '../../utils'
import type { Runner } from '../types'

export const runner: Runner = async ({ part, dayDir }) => {
    const dayFilePath = path.resolve(dayDir, 'index.ts')
    const [[,...result], perf] = await runProgram(`yarn esr ${dayFilePath} ${part}`)

    return [result, perf]
}