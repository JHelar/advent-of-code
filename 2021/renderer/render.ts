import {
  Color,
  createColor,
  TerminalCanvas,
} from "https://deno.land/x/terminal@0.1.0-dev.3/src/mod.ts";
export * from "./colors.ts";

let running = false;
let terminal: TerminalCanvas;
const buffer: (() => void)[] = [];

const tryReadTerminal = async (): Promise<string | undefined> => {
  const buffer = new Uint8Array(512);
  const bytesRead = await Deno.stdin.read(buffer);
  if (bytesRead == null || bytesRead === 0) {
    return;
  }

  const sequence = new TextDecoder().decode(buffer.subarray(0, bytesRead));
  return sequence;
};

const terminalLoop = async (terminal: TerminalCanvas) => {
  const input = await tryReadTerminal();
  if (input === "q") {
    await terminal.close();
    Deno.exit(0);
  }

  if (running) {
    setTimeout(terminalLoop, 10, terminal);
  }
};

export const createRenderer = async (columns?: number, rows?: number) => {
  terminal = new TerminalCanvas();
  await terminal.initialize();
  if (columns && rows) {
    terminal.terminal.setSize(columns, rows);
    terminal.updateSize();
  }
  running = true;

  await terminalLoop(terminal);
  return terminal;
};

export const clearBuffer = () => {
  buffer.splice(0);
};

export const setBackground = (color: Color) => {
  terminal.blankScreen = [...Array(terminal.height)].map(() =>
    Array(terminal.width).fill({ ...color })
  );
};

export const drawPixel = (
  x: number,
  y: number,
  color: Color,
  persist = true,
) => {
  terminal.drawPixel(x, y, color);
  if (persist) {
    buffer.push(() => terminal.drawPixel(x, y, color));
  }
};

export const drawChar = (
  x: number,
  y: number,
  char: string,
  color: Color,
  persist = true,
) => {
  terminal.terminal.setCell(x, y, char, color);
  if (persist) {
    buffer.push(() => terminal.terminal.setCell(x, y, char, color));
  }
};

export const renderToScreen = async () => {
  await terminal.render();
  buffer.forEach((b) => b());
};

export const sleep = (ms: number) =>
  new Promise<void>((resolve) => {
    setTimeout(() => {
      resolve();
    }, ms);
  });

export const makeColor = createColor;
