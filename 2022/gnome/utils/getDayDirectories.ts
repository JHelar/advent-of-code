import fs from "fs/promises";
import path from "path";

export type DayDirectoryName = `day-${number}`;
const isDayDirectory = (value: string): value is DayDirectoryName =>
  value.match(/day\-/) !== null;

export const getDayDirectories = async () => {
  const files = await fs.readdir("./days");
  const dayDirectories: [dir: string, num: number][] = [];
  for (const file of files) {
    if (isDayDirectory(file)) {
      const [, numStr] = file.split("-");
      dayDirectories.push([path.resolve("days", file), Number(numStr)]);
    }
  }

  dayDirectories.sort((a, b) => a[1] - b[1]);
  return dayDirectories;
};
