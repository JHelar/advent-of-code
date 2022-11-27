import { fetchInput, getLanguage, logger } from "./utils";
import fs from "fs/promises";
import path from "path";
import { DayDirectoryName, getDayDirectories } from "./utils";


const createNextDayDirectory = async (): Promise<
  [dir: string, day: number]
> => {
  const dayDirectories = await getDayDirectories();
  const previousDay = dayDirectories.at(-1);
  let day = 1;
  if (previousDay) {
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
  const language = getLanguage()

  logger.log('Creating a whole new day... directory')
  const [outDir, day] = await createNextDayDirectory();
  
  logger.log(`Fetching day ${day} input...`)
  const input = await fetchInput(day);
  
  await language.generate({
    outDir,
    day,
    input,
  });
  logger.log(`Day ${day} generated! ${outDir}`)
})();
