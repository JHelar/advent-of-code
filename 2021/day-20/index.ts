// INPUT URL: https://adventofcode.com/2021/day/20/input
const enum Pixel {
  ON = "#",
  OFF = ".",
}
type Image = Record<string, Pixel>;
type Decoder = Pixel[];
type Position = [x: number, y: number];

const toImageKey = ([x, y]: Position) => `${x},${y}`;
const toPosition = (imageKey: string) =>
  imageKey.split(",").map(Number) as Position;

const getDecoderImage = async () => {
  const inputString = await Deno.readTextFile("./day-20/input.txt");
  const [decoderString, imageString] = inputString.split("\n\n");
  const imageRaw = imageString
    .trim()
    .split("\n")
    .map((row) => row.trim());
  const image = imageRaw.reduce<Image>(
    (image, row, y) => ({
      ...image,
      ...row.split("").reduce<Image>(
        (imageRow, pixel, x) => ({
          ...imageRow,
          [toImageKey([x, y])]: pixel as Pixel,
        }),
        {},
      ),
    }),
    {},
  );

  const decoder: Decoder = decoderString
    .split("\n")
    .flatMap<Pixel>((line) => line.trim().split("") as Pixel[]);

  return {
    image,
    imageRaw,
    decoder,
  };
};

const getPixelRange = ([x, y]: Position) =>
  Array(3)
    .fill(y)
    .flatMap((_, yi) =>
      Array(3)
        .fill(x)
        .map<Position>((_, xi) => [x - 1 + xi, y - 1 + yi])
    );

const extendImage = (range: Position[], image: Image): Image => {
  const newImage: Image = { ...image };
  for (const position of range) {
    const key = toImageKey(position);
    newImage[key] = image[key] || Pixel.OFF;
  }
  return newImage;
};

const getDecodingString = (
  range: Position[],
  image: Image,
  nullPixel: Pixel,
): Decoder => range.map((position) => image[toImageKey(position)] || nullPixel);

const decodeStringToBinary = (decodeString: Pixel[]): string =>
  decodeString.reduce(
    (bin, pixel) => (bin += pixel === Pixel.ON ? "1" : "0"),
    "",
  );

const decodeImageOnce = (
  image: Image,
  decoder: Decoder,
  nullPixel: Pixel,
  [xMin, yMax]: Position,
  [xMax, yMin]: Position,
) => {
  const newImage: Image = {};
  let cutCount = 0;
  let sameInArow = 0;
  let prevRow = "";
  for (let y = Math.min(yMax, yMin); y < Math.max(yMax, yMin); y++) {
    let row = "";
    for (let x = Math.min(xMax, xMin); x < Math.max(xMax, xMin); x++) {
      const position: Position = [x, y];
      const range = getPixelRange(position);
      const decodingString = getDecodingString(range, image, nullPixel);
      const binary = decodeStringToBinary(decodingString);
      const dec = parseInt(binary, 2);
      const pixel = decoder[dec];
      newImage[toImageKey(position)] = pixel;
      // row += pixel;
    }
    // if (row === prevRow) {
    //   sameInArow++;
    //   if (sameInArow === 3) {
    //     cutCount++;
    //     sameInArow--;
    //   }
    // } else {
    //   sameInArow--;
    // }
    // prevRow = row;
  }

  return {
    image: newImage,
    topLeftPosition: [xMin - 2 + cutCount, yMax - 2 + cutCount] as Position,
    bottomRightPosition: [xMax + 2 - cutCount, yMin + 2 - cutCount] as Position,
  };
};

const decodeImageN = (
  steps: number,
  image: Image,
  decoder: Decoder,
  topLeftPosition: Position,
  bottomRightPosition: Position,
) => {
  for (let step = 0; step < steps; step++) {
    const nullPixel = decoder[step % 2 === 0 ? decoder.length - 1 : 0];
    const result = decodeImageOnce(
      image,
      decoder,
      nullPixel,
      topLeftPosition,
      bottomRightPosition,
    );
    image = result.image;
    topLeftPosition = result.topLeftPosition;
    bottomRightPosition = result.bottomRightPosition;
  }

  return {
    image,
    topLeftPosition,
    bottomRightPosition,
  };
};

const printImage = (
  image: Image,
  nullPixel: Pixel,
  [xMin, yMax]: Position,
  [xMax, yMin]: Position,
) => {
  for (let y = Math.min(yMax, yMin); y < Math.max(yMax, yMin); y++) {
    let row = "";
    for (let x = Math.min(xMax, xMin); x < Math.max(xMax, xMin); x++) {
      const position: Position = [x, y];
      row += image[toImageKey(position)] || nullPixel;
    }
    console.log(row);
  }
};

export const part1 = async () => {
  const { image, decoder, imageRaw } = await getDecoderImage();
  const topLeftPosition: Position = [-2, -2];
  const bottomRightPosition: Position = [
    imageRaw.length + 2,
    imageRaw[0].length + 2,
  ];

  const result = decodeImageN(
    2,
    image,
    decoder,
    topLeftPosition,
    bottomRightPosition,
  );

  return Object.values(result.image).reduce(
    (s, pixel) => s + (Pixel.ON === pixel ? 1 : 0),
    0,
  );
};

export const part2 = async () => {
  const { image, decoder, imageRaw } = await getDecoderImage();
  const topLeftPosition: Position = [-2, -2];
  const bottomRightPosition: Position = [
    imageRaw.length + 2,
    imageRaw[0].length + 2,
  ];

  const result = decodeImageN(
    50,
    image,
    decoder,
    topLeftPosition,
    bottomRightPosition,
  );
  return Object.values(result.image).reduce(
    (s, pixel) => s + (Pixel.ON === pixel ? 1 : 0),
    0,
  );
};
