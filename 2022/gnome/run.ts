import { logger, parseArgs } from "./utils";
import path from "path";

(async () => {
  logger.headline("Run")
  const args = parseArgs()
  if (args.day === undefined || isNaN(args.day)) {
    logger.error(`Day "${args.day}", given is not a day!`);
    process.exit(-1);
  }

  if (args.part !== undefined && isNaN(args.part)) {
    logger.error(`Part "${args.part}", given is not a part!`);
    process.exit(-1);
  }
  args.part = args.part ?? 1

  if(!args.lang) {
    logger.error('Missing --lang option')
    process.exit(-1)
  }

  const dayDir = path.resolve(__dirname, '..', 'days', `day-${args.day}`)
  logger.info(`Running day ${args.day} part ${args.part}...`)
  const [result, perf] = await args.lang.runner({
    day: args.day,
    dayDir,
    part: args.part,
  });

  result.forEach(line => {
    logger.result(line)
  })

  logger.info(`${perf.duration.toFixed()}ms`)
  logger.divider()
})();
