import { ensureDir } from "https://deno.land/std@0.78.0/fs/mod.ts";
import { normalize } from "https://deno.land/std@0.78.0/path/mod.ts";
import { renderFile } from "https://deno.land/x/mustache/mod.ts";
import { logger } from "./logger.ts";

const DAY_REGEX = /day-([0-9]{1,2})/;

logger.greeting();
logger.headline("CREATING A WHOLE NEW DAY!");

const days: number[] = [];
for await (const dir of Deno.readDir(".")) {
  if (dir.isDirectory) {
    const dayMatch = DAY_REGEX.exec(dir.name);
    if (dayMatch !== null) {
      const [_, day] = dayMatch;
      days.push(Number(day));
    }
  }
}

let theDay = Math.max(...days) + 1;
if (theDay === -Infinity) {
  theDay = 1;
}
const dirName = `day-${theDay}`;
await ensureDir(normalize(`./${dirName}`));

const indexFileContents = await renderFile(
  normalize("./template/index.mustache"),
  { theDay },
);
await Deno.writeTextFile(normalize(`./${dirName}/index.ts`), indexFileContents);

const daysFile = await Deno.readTextFile(normalize("./days.ts"));
await Deno.writeTextFile(
  normalize("./days.ts"),
  [
    ...daysFile.split("\n"),
    `export * as day${theDay} from './${dirName}/index.ts'`,
  ].join("\n"),
);

logger.headline(`DAY ${theDay} HAS BEEN CREATED!`);
