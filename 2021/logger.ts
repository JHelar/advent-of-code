import * as ink from "https://deno.land/x/ink/mod.ts";

const TREE = await Deno.readTextFile("./tree.txt");
export const logger = {
  greeting: () => console.log(ink.colorize(`<green>${TREE}</green>`)),
  log: ink.terminal.log,
  headline: (text: string) =>
    console.log(
      ink.colorize(`<red>*~"~*~"~*</red> ${text} <red>*~"~*~"~*</red>`),
    ),
  footer: () =>
    ink.terminal.log(
      '<red>*~"~*~"~*~"~*~"~*~"~*~"~*~"~*~"~*~"~*~"~*~"~*~"~*</red>',
    ),
  header: (title: string) =>
    ink.terminal.log(
      `<red>*~"~*~"~*~"~*~"~*~"~*~"~</red><bg-black><green> ${title} </green></bg-black><red>~"~*~"~*~"~*~"~*~"~*~"~*</red>`,
    ),
  error: (text: string, error?: Error) =>
    console.error(ink.colorize(`<bg-red> ${text} </bg-red>`), error),
};
