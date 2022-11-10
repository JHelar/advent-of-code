// INPUT URL: https://adventofcode.com/2021/day/16/input
import {
  Color,
  TerminalCanvas,
} from "https://deno.land/x/terminal@0.1.0-dev.3/src/mod.ts";
import {
  createRenderer,
  drawPixel,
  mult,
  PALETTE,
  renderToScreen,
  setBackground,
  sleep,
} from "../renderer/render.ts";
const enum PACKET_TYPE {
  OP = -1,
  LITERAL = 4,
  SUM = 0,
  PROD = 1,
  MIN = 2,
  MAX = 3,
  GT = 5,
  LE = 6,
  EQ = 7,
}

const enum LENGTH_TYPE {
  BITS_15 = 0,
  BITS_11 = 1,
}

const enum PART_LENGTH {
  VERSION = 3,
  PACKET_TYPE = 3,
  LITERAL_VALUE = 4,
  LITERAL_PREFIX = 1,
  HEADER = 6,
  LENGTH_TYPE = 1,
  BITS_15 = 15,
  BITS_11 = 11,
}

type PacketNode = {
  type: PACKET_TYPE;
  version: number;
  newPointer: number;
};

interface LiteralPacket extends PacketNode {
  type: PACKET_TYPE.LITERAL;
  value: number;
}

interface OperatorPacket extends PacketNode {
  type: PACKET_TYPE.OP;
  opType: PACKET_TYPE;
  children: PacketNode[];
}

const b2d = <T extends number = number>(bin: string) => parseInt(bin, 2) as T;

const getSinglePacket = (pointer: number, binary: string): PacketNode => {
  const header = binary.substr(pointer, PART_LENGTH.HEADER);
  const versionBin = header.substr(0, PART_LENGTH.VERSION);
  const packetTypeBin = header.substr(PART_LENGTH.VERSION, PART_LENGTH.VERSION);

  const version = b2d(versionBin);
  const packetType = b2d<PACKET_TYPE>(packetTypeBin);

  const packetPointer = pointer + PART_LENGTH.HEADER;
  switch (packetType) {
    case PACKET_TYPE.LITERAL:
      {
        let valueBin = "";
        for (
          let i = packetPointer;
          i < binary.length;
          i += PART_LENGTH.LITERAL_PREFIX + PART_LENGTH.LITERAL_VALUE
        ) {
          const prefix = b2d(binary[i]);
          valueBin += binary.substr(
            i + PART_LENGTH.LITERAL_PREFIX,
            PART_LENGTH.LITERAL_VALUE,
          );
          if (prefix === 0) {
            const packet: LiteralPacket = {
              type: packetType,
              version,
              value: b2d(valueBin),
              newPointer: i + PART_LENGTH.LITERAL_PREFIX +
                PART_LENGTH.LITERAL_VALUE,
            };
            return packet;
          }
        }
      }
      break;
    default: {
      const lengthType = b2d<LENGTH_TYPE>(binary[packetPointer]);
      const packet: OperatorPacket = {
        type: PACKET_TYPE.OP,
        opType: packetType,
        newPointer: -1,
        version,
        children: [],
      };
      if (lengthType === LENGTH_TYPE.BITS_15) {
        const subpacketLen = b2d(
          binary.substr(
            packetPointer + PART_LENGTH.LENGTH_TYPE,
            PART_LENGTH.BITS_15,
          ),
        );
        const subpacketStart = packetPointer + PART_LENGTH.LENGTH_TYPE +
          PART_LENGTH.BITS_15;
        let subpacketPointer = subpacketStart;
        while (subpacketPointer < subpacketStart + subpacketLen) {
          const subpacket = getSinglePacket(subpacketPointer, binary);
          packet.children.push(subpacket);
          subpacketPointer = subpacket.newPointer;
        }
        packet.newPointer = subpacketPointer;
        return packet;
      } else if (lengthType === LENGTH_TYPE.BITS_11) {
        const subpacketCount = b2d(
          binary.substr(
            packetPointer + PART_LENGTH.LENGTH_TYPE,
            PART_LENGTH.BITS_11,
          ),
        );
        const subpacketStart = packetPointer + PART_LENGTH.LENGTH_TYPE +
          PART_LENGTH.BITS_11;
        let subpacketPointer = subpacketStart;
        for (let count = 0; count < subpacketCount; count++) {
          const subpacket = getSinglePacket(subpacketPointer, binary);
          packet.children.push(subpacket);
          subpacketPointer = subpacket.newPointer;
        }
        packet.newPointer = subpacketPointer;
        return packet;
      }
    }
  }
  throw new Error("Unable to handle package type");
};

const getBinary = async () => {
  const hexStr = await Deno.readTextFile("./day-16/input.txt");
  const binary = hexStr
    .trim()
    .split("")
    .map((v) => parseInt(v, 16).toString(2).padStart(4, "0"))
    .join("");
  return binary;
};

const isOperatorPacket = (packet: PacketNode): packet is OperatorPacket =>
  packet.type === PACKET_TYPE.OP;

const isLiteralPacket = (packet: PacketNode): packet is LiteralPacket =>
  packet.type === PACKET_TYPE.LITERAL;

const sumPacketVersions = (packet: PacketNode): number => {
  let sum = 0;
  sum += packet.version;
  if (isOperatorPacket(packet)) {
    for (const child of packet.children) {
      sum += sumPacketVersions(child);
    }
  }
  return sum;
};

const evaluatePacket = (packet: PacketNode): number => {
  if (isOperatorPacket(packet)) {
    const childValues = packet.children.map(evaluatePacket);
    switch (packet.opType) {
      case PACKET_TYPE.SUM:
        return childValues.reduce((sum, value) => sum + value, 0);
      case PACKET_TYPE.PROD:
        return childValues.reduce((sum, value) => sum * value, 1);
      case PACKET_TYPE.MAX:
        return Math.max(...childValues);
      case PACKET_TYPE.MIN:
        return Math.min(...childValues);
      case PACKET_TYPE.LE:
        return childValues[0] < childValues[1] ? 1 : 0;
      case PACKET_TYPE.GT:
        return childValues[0] > childValues[1] ? 1 : 0;
      case PACKET_TYPE.EQ:
        return childValues[0] === childValues[1] ? 1 : 0;
    }
  }
  if (isLiteralPacket(packet)) return packet.value;
  throw new Error("Unable to handle package type");
};

const renderPacket = async (
  packet: PacketNode,
  canvas: TerminalCanvas,
  row = 0,
  column = 0,
): Promise<number> => {
  const drawOperator = (op: string, value: string) => {
    canvas.terminal.setCell(column, row, "Operator:", PALETTE.TEXT);
    canvas.terminal.setCell(
      column + "Operator:".length + 1,
      row,
      op,
      PALETTE.GREEN_LIGHT,
    );
    canvas.terminal.setCell(column, row + 1, "Value:", PALETTE.TEXT);
    canvas.terminal.setCell(
      column + "Value:".length + 1,
      row + 1,
      value.toString(),
      PALETTE.GREEN_LIGHT,
    );
  };
  if (isOperatorPacket(packet)) {
    const childValues = await Promise.all(
      packet.children.map((node) =>
        renderPacket(node, canvas, row + 2, column + 1)
      ),
    );
    switch (packet.opType) {
      case PACKET_TYPE.SUM: {
        const value = childValues.reduce((sum, value) => sum + value, 0);
        drawOperator("ADD", value.toString());
        return value;
      }
      case PACKET_TYPE.PROD: {
        const value = childValues.reduce((sum, value) => sum * value, 1);
        drawOperator("PROD", value.toString());
        return value;
      }
      case PACKET_TYPE.MAX: {
        const value = Math.max(...childValues);
        drawOperator("MAX", value.toString());
        return value;
      }
      case PACKET_TYPE.MIN: {
        const value = Math.min(...childValues);
        drawOperator("MIN", value.toString());
        return value;
      }
      case PACKET_TYPE.LE: {
        const value = childValues[0] < childValues[1] ? 1 : 0;
        drawOperator("LE", value.toString());
        return value;
      }
      case PACKET_TYPE.GT: {
        const value = childValues[0] > childValues[1] ? 1 : 0;
        drawOperator("GT", value.toString());
        return value;
      }
      case PACKET_TYPE.EQ: {
        const value = childValues[0] === childValues[1] ? 1 : 0;
        drawOperator("EQ", value.toString());
        return value;
      }
    }
  }
  if (isLiteralPacket(packet)) return packet.value;
  throw new Error("Should not happen");
};

export const part1 = async () => {
  const binary = await getBinary();
  const packet = getSinglePacket(0, binary);

  return sumPacketVersions(packet);
};

export const part2 = async () => {
  const binary = await getBinary();
  const packet = getSinglePacket(0, binary);

  return evaluatePacket(packet);
};

export const render = async () => {
  const binary = await getBinary();
  const packet = getSinglePacket(0, binary);
  const canvas = await createRenderer();
  await renderPacket(packet, canvas);
};
