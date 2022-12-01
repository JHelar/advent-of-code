import { fetchInput, logger, parseArgs } from "./utils";
import fs from "fs/promises";
import path from "path";
import { DayDirectoryName, getDayDirectories } from "./utils";

const createDayDirectory = async (day?: number): Promise<
  [dir: string, day: number]
> => {
  const dayDirectories = await getDayDirectories();
  const existingDayDir = dayDirectories.find(([,dayDir]) => day === dayDir)
  if(existingDayDir) {
    return existingDayDir
  }

  const previousDay = dayDirectories.at(-1);
  day = day ?? 1;
  if (previousDay && day === undefined) {
    day = previousDay[1] + 1;
  }

  if(day > 25) {
    logger.error('Can only create 25 days, you will have to wait until next season!');
    process.exit(0);
  }

  const dayName: DayDirectoryName = `day-${day}`;
  const directoryPath = path.resolve("days", dayName);
  await fs.mkdir(directoryPath, { recursive: true });

  return [directoryPath, day];
};

(async () => {
  logger.headline("Create");
  const args = parseArgs()
  if(!args.lang) {
    logger.error('Missing --lang option')
    process.exit(-1)
  }

  logger.log(`Creating day...`)
  const [outDir, day] = await createDayDirectory(args.day);
  
  logger.log(`Fetching day ${day} input...`)
  const input = await fetchInput(day);
  
  await args.lang.generate({
    outDir,
    day,
    input,
  });
  logger.log(`Day ${day} generated, open the day in code: "code days/day-${day}"`)
  logger.divider();
})();
