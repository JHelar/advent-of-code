import path from 'path'
import { logger, runProgram } from '../../utils'
import type { Runner } from '../types'

export const runner: Runner = async ({ part, dayDir }) => {
    const dayFilePath = path.resolve(dayDir, 'main.kt')
    logger.log('Compiling kotlin...')
    const [compileResult] = await runProgram(`kotlinc ${dayFilePath} -d ${dayDir}`)
    compileResult.forEach(logger.info)

    logger.log('Running...')
    return runProgram(`kotlin MainKt.class ${part}`, { cwd: dayDir })
}