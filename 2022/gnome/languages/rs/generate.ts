import { compile } from "handlebars";
import fs from "fs/promises";
import { Generate } from "../types";
import path from "path";
import { logger } from "../../utils";

export const generate: Generate = async ({ day, input, outDir }) => {
  logger.log("Writing input file...");
  const inputPath = "input.txt";
  await fs.writeFile(path.resolve(outDir, inputPath), input);

  logger.log(`Generating Rust files...`);
  const templates = ["Cargo.toml", "main.rs"];
  for (const template of templates) {
    const templateStr = (
      await fs.readFile(
        path.resolve(
          __dirname,
          "languages",
          "rs",
          "templates",
          `${template}.handlebars`
        )
      )
    ).toString();
    const generateTemplate = compile(templateStr);

    const result = generateTemplate({
      day,
      inputPath,
    });

    await fs.writeFile(path.resolve(outDir, template), result);

  }
  logger.log(
    `Rust files generated, remember to run "code days/day-${day}" in order to get the rust-analyzer to work!`
  );
};
