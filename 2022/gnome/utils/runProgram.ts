import { exec } from "child_process";
import { performance, PerformanceMeasure } from "perf_hooks";
import { logger } from "./logger";

export const runProgram = (
  ...params: Parameters<typeof exec>
): Promise<[result: string[], performance: PerformanceMeasure]> =>
  new Promise((res, rej) => {
    performance.mark("start");
    const proc = exec(...params);
    
    proc.stdout?.on("data", (message) => {
      logger.result(message.replaceAll('\n', ''));
    });

    proc.on("close", (code) => {
      performance.mark("end");
      if(code === -1) {
        rej()
      } else {
        res([[], performance.measure("start to end", "start", "end")]);
      }
    });
  });
