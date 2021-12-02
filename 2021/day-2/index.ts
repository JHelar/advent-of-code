// INPUT URL: https://adventofcode.com/2021/day/2/input
const enum DIRECTION {
  FORWARD = "forward",
  UP = "up",
  DOWN = "down",
}

type Position = {
  x: number;
  y: number;
  aim: number;
};
type Command = {
  magnitude: number;
  direction: DIRECTION;
};

const toCommand = (commandStr: string): Command => {
  const [direction, magnitudeStr] = commandStr.split(" ");
  const magnitude = Number(magnitudeStr);
  return {
    direction: direction as DIRECTION,
    magnitude,
  };
};

const getCommands = async () => {
  const inputString = await Deno.readTextFile("./day-2/input.txt");
  const commands = inputString.split("\n").map(toCommand);
  return commands;
};

export const part1 = async () => {
  const commands = await getCommands();
  const submarine: Position = { x: 0, y: 0, aim: 0 };
  for (const { direction, magnitude } of commands) {
    switch (direction) {
      case DIRECTION.FORWARD:
        submarine.x += magnitude;
        break;
      case DIRECTION.DOWN:
        submarine.y += magnitude;
        break;
      case DIRECTION.UP:
        submarine.y -= magnitude;
        break;
      default:
        break;
    }
  }

  return submarine.x * submarine.y;
};

export const part2 = async () => {
  const commands = await getCommands();
  const submarine: Position = { x: 0, y: 0, aim: 0 };
  for (const { direction, magnitude } of commands) {
    switch (direction) {
      case DIRECTION.FORWARD:
        submarine.x += magnitude;
        submarine.y += submarine.aim * magnitude;
        break;
      case DIRECTION.DOWN:
        submarine.aim += magnitude;
        break;
      case DIRECTION.UP:
        submarine.aim -= magnitude;
        break;
      default:
        break;
    }
  }

  return submarine.x * submarine.y;
};
