import { Node, Portal } from "./node";

export const toPixel = (node: Node) =>
	`<span style="--x:${node.x};--x-end:${node.x + 1};--y:${node.y};--y-end:${
		node.y + 1
	};--color:${
		node.visited
			? "green"
			: node.road
			? "lightgray"
			: node.air
			? "black"
			: node.wall
			? "#333"
			: "black"
	};--color-text:${node instanceof Portal ? "blue" : "black"}">${
		node instanceof Portal ? node.name : node.road ? node.level : ""
	}</span>`;
export const toHtmlDocument = (
	pixels: string[],
	width: number,
	height: number
) => `<!DOCTYPE html>
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
            grid-template-columns: repeat(${width}, 20px);
            grid-template-rows: repeat(${height}, 20px);
        }
        span {
            display: block;
            grid-column: var(--x);
            grid-row: var(--y);
			background-color: var(--color);
        }
    </style>
</head>
<body>
    ${pixels.join('')}
</body>
</html>`;
