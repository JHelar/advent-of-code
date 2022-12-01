import { compile } from "handlebars";
import fs from "fs/promises";
import { Generate } from "../types";
import path from "path";
import { logger } from "../../utils";

export const generate: Generate = async ({ day, input, outDir }) => {
  logger.log(`Generating Kotlin files...`);
  const templateStr = (
    await fs.readFile(
      path.resolve(__dirname, "languages", "kt", "day.handlebars")
    )
  ).toString();

  const generateTemplate = compile(templateStr);

  logger.log("Writing input file...");
  const inputPath = "input.txt";
  await fs.writeFile(path.resolve(outDir, inputPath), input);

  logger.log("Generating template...");
  const result = generateTemplate({
    inputPath,
  });

  await fs.writeFile(path.resolve(outDir, "main.kt"), result);
};
