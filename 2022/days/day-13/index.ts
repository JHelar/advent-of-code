import fs from "fs/promises";
import path from "path";

const readInput = async () => {
  const content = await fs.readFile(path.resolve(__dirname, "input.txt"));
  return content.toString("utf8");
};

type Packet = number | Packet[];
type PacketPair = [left: Packet, right: Packet];

const parsePackets = async () => {
  const content = await readInput();
  return content
    .split("\n\n")
    .filter(Boolean)
    .map(
      (pairs) =>
        pairs
          .trim()
          .split("\n")
          .map((packet) => JSON.parse(packet.trim()) as Packet) as PacketPair
    );
};

const comparePacket = (left: Packet, right: Packet): number => {
  if (typeof left === "number" && typeof right === "number") {
    return Math.max(-1, Math.min(left - right, 1));
  }

  left = Array.isArray(left) ? left : [left];
  right = Array.isArray(right) ? right : [right];

  for (let i = 0; true; i++) {
    if (left.length === right.length && left.length === i) {
      return 0;
    }

    if (i < left.length && i < right.length) {
      const result = comparePacket(left[i], right[i]);
      if (result !== 0) {
        return result;
      }
      continue;
    }

    return Math.max(-1, Math.min(left.length - right.length, 1));
  }
};

const part1 = async () => {
  const packets = await parsePackets();
  const result = packets
    .map((pair, index) => ({ pair, index }))
    .filter(({ pair: [left, right] }) => comparePacket(left, right) === -1)
    .map(({ index }) => index + 1)
    .reduce((sum, indice) => sum + indice, 0);

  return `Result: ${result}`;
};

const part2 = async () => {
  const packets = (await parsePackets()).flat();

  const divider1 = JSON.parse("[[2]]");
  const divider2 = JSON.parse("[[6]]");

  packets.push(divider1);
  packets.push(divider2);

  packets.sort(comparePacket);
  const result = (packets.indexOf(divider1) + 1) * (packets.indexOf(divider2) + 1)

  return `Result: ${result}`;
};

// Generated code to run on cli
(async () => {
  const [, , part] = process.argv;
  if (part === "1") {
    const result = await part1();
    console.log(result);
  }
  if (part === "2") {
    const result = await part2();
    console.log(result);
  }
})();
