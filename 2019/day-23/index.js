"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const fs_1 = __importDefault(require("fs"));
const worker_1 = __importDefault(require("./worker"));
const programInput = fs_1.default.readFileSync('day-23/input.txt').toString();
const computers = new Map();
const queues = new Map();
for (let address = 0n; address < 50n; address++) {
    const worker = new worker_1.default(address, programInput);
    computers.set(address, worker);
    queues.set(address, []);
}
let lastNATY = -1000000000n;
while (true) {
    let stuffInQueue = false;
    for (const [address, computer] of computers) {
        const queue = queues.get(address);
        queues.set(address, []);
        const packets = computer.run(queue);
        if (packets.length) {
            stuffInQueue = true;
            packets.forEach(([to, x, y]) => {
                if (to === 255n) {
                    queues.set(to, [x, y]);
                }
                else {
                    queues.set(to, [...queues.get(to), x, y]);
                }
            });
        }
    }
    if (!stuffInQueue) {
        const NATPacket = queues.get(255n);
        if (NATPacket) {
            if (lastNATY === NATPacket[1]) {
                console.log('ALL GOOD!', lastNATY);
                break;
            }
            lastNATY = NATPacket[1];
            queues.set(0n, NATPacket);
        }
        else {
            console.log('NO NAT PACKET!');
            break;
        }
    }
}
