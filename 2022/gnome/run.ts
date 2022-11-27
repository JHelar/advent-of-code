import { getLanguage, logger } from "./utils";
import path from "path";

const isPart = (value: number): value is 1 | 2 => value === 1 || value === 2;

(async () => {
  const [, , , dayStr, partStr] = process.argv;
  const day = Number(dayStr);
  if (isNaN(day)) {
    logger.error(`Day "${day}", given is not a day!`);
    process.exit(-1);
  }

  let part = partStr ? Number(partStr) : 1;
  if (isNaN(part) || !isPart(part)) {
    logger.error(`Part "${part}", given is not a part!`);
    process.exit(-1);
  }

  const language = getLanguage();

  const dayDir = path.resolve(__dirname, '..', 'days', `day-${day}`)
  logger.info(`Running day ${day} part ${part}...`)
  const [result, perf] = await language.runner({
    day,
    dayDir,
    part,
  });

  result.forEach(line => {
    logger.result(line)
  })

  logger.info(`Run took ${perf.duration.toFixed()}ms`)
})();
