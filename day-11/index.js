const { Readable } = require('stream');

const makeProgram = require('./read-intcode');
const robotMap = require('./robot-map');

const intcode = require('../utils').readArray('./day-11/input.txt');
const toPixel = node => `<span style="--x:${node.x};--x-end:${node.x + 1};--y:${node.y};--y-end:${node.y + 1};--color:${node.color.toLowerCase()}"></span>`
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
            grid-template-columns: repeat(${100}, 10px);
            grid-template-rows: repeat(${100}, 10px);
        }
        span {
            display: block;
            grid-column: var(--x)/var(--x-end);
            grid-row: var(--y)/var(--y-end);
            background-color: var(--color);
        }
    </style>
</head>
<body>
    ${pixels}
</body>
</html>`


const robotOutputStream = new Readable();
robotOutputStream._read = () => { };

const robotInputStream = new Readable();
robotInputStream._read = () => { }; // redundant? see update below

const robotProgram = makeProgram(robotInputStream, robotOutputStream);
robotProgram(intcode);

robotMap(robotOutputStream, robotInputStream)
    .then(map => {
        const pixels = Object.values(map).map(toPixel)
        const htmlDoc = toHtmlDocument(pixels.join(''));
        require('fs').writeFileSync('./day-11/map.html', htmlDoc);

    })