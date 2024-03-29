import { compile } from "handlebars";
import fs from "fs/promises";
import { Generate } from "../types";
import path from "path";
import { logger } from "../../utils";

export const generate: Generate = async ({ day, input, outDir }) => {
  logger.log(`Generating typescript files...`);
  const templateStr = (
    await fs.readFile(
      path.resolve(__dirname, "languages", "ts", "day.handlebars")
    )
  ).toString();

  const generateTemplate = compile(templateStr);

  logger.log("Writing input file...");
  const inputFilename = "input.txt";
  await fs.writeFile(path.resolve(outDir, inputFilename), input);

  logger.log("Generating template...");
  const result = generateTemplate({
    inputPath: inputFilename,
  });

  await fs.writeFile(path.resolve(outDir, "index.ts"), result);
};
