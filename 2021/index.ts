import * as days from "./days.ts";
import { logger } from "./logger.ts";

const [day] = Deno.args;
const key = `day${day}`;
logger.header(`RUNNING DAY ${day}`);
if (key in days) {
  try {
    const { part1, part2 } = (days as any)[key];
    performance.mark("part1");
    const part1Result = await part1();
    const part1Perf = performance.measure("part1");

    performance.mark("part2");
    const part2Result = await part2();
    const part2Perf = performance.measure("part2");

    logger.headline(`HO HO HO DAY ${day} RESULTS`);
    logger.headline(`PART1`);
    logger.log(`Result: ${part1Result}`);
    logger.log(`Duration: ${part1Perf.duration}`);
    logger.headline(`PART2`);
    logger.log(`Result: ${part2Result}`);
    logger.log(`Duration: ${part2Perf.duration}`);
  } catch (error) {
    logger.error("HO HO HONO!", error);
  }
} else {
  logger.error(`HO HO HO INVLID DAY ${day}!`);
}
logger.footer();
