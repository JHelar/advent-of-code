import { exec as execCB } from 'child_process'
import { performance, PerformanceMeasure } from 'perf_hooks'
import { promisify } from 'util'

const exec = promisify(execCB)

export const runProgram = async (...params: Parameters<typeof exec>): Promise<[result: string[], performance: PerformanceMeasure]> => {
    performance.mark('start')
    const { stdout } = await exec(...params)
    performance.mark('end')

    const result = stdout.toString().split('\n').filter(Boolean)

    return [result, performance.measure('start to end', 'start', 'end')]
}
