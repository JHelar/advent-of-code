
export const toPixel = ([x, y, z]: number[], bug: boolean) =>
	`<span style="--x:${x + 1};--y:${y + 1};--color:${bug ? "green" : "#333"};">${z}</span>`;
export const toHtmlDocument = (
	pixels: string[][],
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
            display: flex;
            flex-flow: row wrap;
            margin: -10px;
            padding: 10px;
        }
        .row {
            display: grid;
            grid-template-columns: repeat(5, 20px);
            grid-template-rows: repeat(5, 20px);
            margin: 10px;
        }
        span {
            display: block;
            grid-column: var(--x);
            grid-row: var(--y);
            background-color: var(--color);
            border: 1px solid black;
            margin: -1px;
        }
    </style>
</head>
<body>
    ${pixels.map((row) => `<div class="row">${row.join('')}</div>`).join('')}
</body>
</html>`;
