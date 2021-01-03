declare class Operator {
    program: IntProgram;
    constructor(program: IntProgram);
    add(): void;
    mul(): void;
    input(value: bigint): void;
    output(): bigint;
    jumpIfTrue(): void;
    jumpIfFalse(): void;
    lesserThan(): void;
    equals(): void;
    adjust(): void;
}
export declare class IntProgram {
    memory: Record<string, bigint>;
    originInput: string;
    cursor: bigint;
    relativeBase: bigint;
    running: boolean;
    opCode: number;
    paramModes: number[];
    operator: Operator;
    constructor(input: string);
    getValueAt(address: bigint): bigint;
    getValueFromPointerAt(address: bigint): bigint;
    getValueRelativeFromPointerAt(address: bigint): bigint;
    setValueAt(value: bigint, address: bigint): void;
    setValueFromPointerAt(value: bigint, address: bigint): void;
    setValueRelativeFromPointerAt(value: bigint, address: bigint): void;
    getValueForParam(paramNo: number, address: bigint): bigint;
    setValueForParam(paramNo: number, value: bigint, address: bigint): void;
    advanceCursor(): void;
    advanceProgram(): void;
    reset(): void;
    initializeMemory(): void;
    exec(input: bigint[], output?: bigint[]): number | bigint;
}
export {};
