const { Readable } = require('stream');

const makeProgram = require('./read-intcode');
const robotMap = require('./robot-map');

const intcode = require('../utils').readArray('./day-11/input.txt');
const toPixel = node =>
	`<span style="--x:${node.x};--x-end:${node.x + 1};--y:${
		node.y
	};--y-end:${node.y + 1};--color:${
		node.color === '0' ? 'black' : 'white'
	};--color-text:${node.color === '0' ? 'white' : 'black'}"></span>`;
const toHtmlDocument = (pixels, width, height) => `<!DOCTYPE html>
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
            grid-template-columns: repeat(${width}, 10px);
            grid-template-rows: repeat(${height}, 10px);
        }
        span {
            display: block;
            grid-column: var(--x);
            grid-row: var(--y);
			background-color: var(--color);
			color: var(--color-text);
        }
    </style>
</head>
<body>
    ${pixels}
</body>
</html>`;

const robotProgram = makeProgram(intcode);

const theMap = robotMap(robotProgram);
const mapNodes = Object.keys(theMap).map(key => {
	// Destruct key to position
	const [x, y] = key.replace(/[\(\)]*/g, '').split(',');
	const node = {
		x: x | 0,
		y: y | 0,
		color: theMap[key]
	}

	return node;
});
const minWidth = Math.min(...mapNodes.map(({ x }) => x));
const minHeight = Math.min(...mapNodes.map(({ y }) => y));

const mapWidth = Math.max(...mapNodes.map(({ x }) => x)) - minWidth;
const mapHeight = Math.max(...mapNodes.map(({ y }) => y)) - minHeight;

const pixels = mapNodes
	.map(node => ({
		...node,
		x: node.x + Math.abs(minWidth) + 1,
		y: node.y + Math.abs(minHeight) + 1
	}))
	.map(toPixel)
	.join('');

console.log({ count: mapNodes.length })
const htmlDoc = toHtmlDocument(pixels, mapWidth + 1, mapHeight + 1);
require('fs').writeFileSync('./day-11/map.html', htmlDoc);
