import { IntProgram } from '../intcode'
import chunk from 'lodash.chunk'

export default class Worker {
    address: bigint
    idle: boolean
    input: bigint[]
    program: IntProgram

    constructor(address: bigint, input: string) {
        this.address = address
        this.idle = false
        this.input = [this.address]
        this.program = new IntProgram(input)
    }

    run(queue: bigint[]) {
        const output: bigint[] = []
        if(queue.length) {
            this.input.push(...queue)
        } else {
            this.input.push(-1n)
        }
        
        this.program.running = true
        while(this.program.running) {
            this.program.exec(this.input, output)
        }

        if(output.length) {
            const packets = chunk(output, 3) as bigint[][]
            return packets
        }
        return []
    }
}