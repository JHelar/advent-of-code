import { runProgram } from '../../utils'
import type { Runner } from '../types'

export const runner: Runner = async ({ part, dayDir }) => {
    const [result, perf] = await runProgram(`cargo run -- ${part}`, { cwd: dayDir })

    return [result, perf]
}