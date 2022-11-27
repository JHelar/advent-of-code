import { PerformanceMeasure } from "perf_hooks"

type GenerateArgs = {
    day: number
    input: string
    outDir: string
}

export type Generate = (args: GenerateArgs) => Promise<void>

type RunnerArgs = {
    day: number
    dayDir: string
    part: 1 | 2
}

export type Runner = (args: RunnerArgs) => Promise<[result: string[], performance: PerformanceMeasure]>