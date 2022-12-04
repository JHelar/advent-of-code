import fs from "fs/promises";
import path from "path";

const readInput = async () => {
  const content = await fs.readFile(path.resolve(__dirname, "input.txt"));
  return content.toString("utf8");
};

const isUpperCase = (char: string) => char === char.toUpperCase();

const chunk = <T>(arr: T[], size: number) =>
  Array.from({ length: Math.ceil(arr.length / size) }, (v, i) =>
    arr.slice(i * size, i * size + size)
  );

const part1 = async () => {
  const content = await readInput();
  const compartements = content
    .split("\n")
    .map((line) => [
      line.slice(0, line.length / 2),
      line.slice(line.length / 2),
    ]);
  const result = compartements
    .reduce((uniques, [comp1, comp2]) => {
      const item = comp1.split("").find((item) => comp2.indexOf(item) > -1);
      uniques.push(item!);
      console.log([comp1, comp2]);
      return uniques;
    }, [] as string[])
    .filter(Boolean)
    .map((item) => {
      if (isUpperCase(item)) {
        return 27 + item.charCodeAt(0) - 65;
      }
      return item.charCodeAt(0) - 96;
    })
    .reduce((sum, item) => sum + item);

  return `Result part 1 lol: ${result}`;
};

const part2 = async () => {
  const content = await readInput();
  const compartements = chunk(content.split("\n"), 3);

  const result = compartements
    .reduce(
      (uniques, [comp1, comp2, comp3]) => [
        ...uniques,
        comp1
          .split("")
          .find(
            (item) => comp2.indexOf(item) > -1 && comp3.indexOf(item) > -1
          )!,
      ],
      [] as string[]
    )
    .filter(Boolean)
    .map((item) => {
      if (isUpperCase(item)) {
        return 27 + item.charCodeAt(0) - 65;
      }
      return item.charCodeAt(0) - 96;
    })
    .reduce((sum, item) => sum + item);

  return `Result part 2 lol: ${result}`;
};

// Generated code to run on cli
(async () => {
  const [, , part] = process.argv;
  if (part === "1") {
    const result = await part1();
    console.log(result);
  }
  if (part === "2") {
    const result = await part2();
    console.log(result);
  }
})();
