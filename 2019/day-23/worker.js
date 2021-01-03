"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const intcode_1 = require("../intcode");
const lodash_chunk_1 = __importDefault(require("lodash.chunk"));
class Worker {
    constructor(address, input) {
        this.address = address;
        this.idle = false;
        this.input = [this.address];
        this.program = new intcode_1.IntProgram(input);
    }
    run(queue) {
        const output = [];
        if (queue.length) {
            this.input.push(...queue);
        }
        else {
            this.input.push(-1n);
        }
        this.program.running = true;
        while (this.program.running) {
            this.program.exec(this.input, output);
        }
        if (output.length) {
            const packets = lodash_chunk_1.default(output, 3);
            return packets;
        }
        return [];
    }
}
exports.default = Worker;
