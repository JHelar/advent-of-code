const { getImageData, getImageChunks } = require('./get-image-data');

const inputString = require('../utils').getStringFromFile('./day-8/input.txt');

const getPixel = (imageData, rowPointer, colPointer) => {
    const pixels = imageData.map(layer => layer[rowPointer][colPointer]);
    return pixels.find(pixel => pixel === '1' || pixel === '0') || '2';
}

const htmlPixel = value => `<span data-pixel="${value}"></span>`
const toHtmlDocument = pixels => `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title>Document</title>
    <style>
        body {
            background-color: rebeccapurple;
            height: 100vh;
            width: 100vw;
            display: grid;
            grid-template-columns: repeat(${IMAGE_WIDTH}, 20px);
            grid-template-rows: repeat(${IMAGE_HEIGHT}, 20px);
        }
        span[data-pixel="0"]{
            background-color: #000;
        }
        span[data-pixel="1"]{
            background-color: #fff;
        }
    </style>
</head>
<body>
    ${pixels}
</body>
</html>`
const IMAGE_WIDTH = 25;
const IMAGE_HEIGHT = 6;

const imageData = getImageData(inputString, IMAGE_WIDTH, IMAGE_HEIGHT);

let colPointer = 0;
let rowPointer = 0;

let image = '';
let pixelRow = ''

for(let pixelPointer = 0; pixelPointer < IMAGE_WIDTH * IMAGE_HEIGHT; pixelPointer++) {
    const pixel = getPixel(imageData, rowPointer, colPointer);

    pixelRow += htmlPixel(pixel);
    colPointer = (colPointer + 1) % IMAGE_WIDTH;
    if(colPointer === 0) {
        image += pixelRow + '\n';
        pixelRow = '';
        rowPointer++;
    }
}

require('fs').writeFileSync('./day-8/image.html', toHtmlDocument(image))