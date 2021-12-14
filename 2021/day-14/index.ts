// INPUT URL: https://adventofcode.com/2021/day/14/input
type Template = Record<string, number>;
type Insertions = Record<string, string>;

const getTemplate = (templateString: string) => {
  const template: Template = {};
  for (let i = 0; i < templateString.length - 1; i++) {
    const pair = templateString[i] + templateString[i + 1];
    template[pair] = ++template[pair] || 1;
  }

  return template;
};

const getInput = async () => {
  const inputString = await Deno.readTextFile("./day-14/input.txt");
  const [templateString, ...insertionPairs] = inputString.split("\n").map((
    line,
  ) => line.trim()).filter(Boolean);
  const insertions = insertionPairs.reduce((insertions, pair) => {
    const [elements, results] = pair.split(" -> ");
    return {
      ...insertions,
      [elements]: results,
    };
  }, {} as Insertions);

  return {
    template: getTemplate(templateString),
    templateString,
    insertions,
  };
};

const runInsertions = (
  template: Template,
  insertions: Insertions,
) => {
  const newTemplate: Template = {};

  for (const [pair, element] of Object.entries(insertions)) {
    if (pair in template) {
      const count = template[pair];

      const newPair1 = pair[0] + element;
      const newPair2 = element + pair[1];

      newTemplate[newPair1] = (newTemplate[newPair1] || 0) + count;
      newTemplate[newPair2] = (newTemplate[newPair2] || 0) + count;
    }
  }

  return newTemplate;
};

const getOccurrances = (template: Template, initialElement: string) => {
  const elements = Object.entries(template).reduce(
    (acc, [pair, count]) => ({
      ...acc,
      [pair[1]]: (acc[pair[1]] || 0) + count,
    }),
    {} as Record<string, number>,
  );
  elements[initialElement] = ++elements[initialElement] || 1;

  return Object.entries(elements).sort(([, a], [, b]) => a - b);
};

export const part1 = async () => {
  let { template, templateString, insertions } = await getInput();

  const steps = 10;
  for (let step = 0; step < steps; step++) {
    template = runInsertions(template, insertions);
  }

  const occurances = getOccurrances(template, templateString[0]);
  return occurances[occurances.length - 1][1] - occurances[0][1];
};

export const part2 = async () => {
  let { template, templateString, insertions } = await getInput();

  const steps = 40;
  for (let step = 0; step < steps; step++) {
    template = runInsertions(template, insertions);
  }

  const occurances = getOccurrances(template, templateString[0]);
  return occurances[occurances.length - 1][1] - occurances[0][1];
};
