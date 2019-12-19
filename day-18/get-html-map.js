const toPixel = node => {
    let color;
    switch(node.value) {
        case '#':
            color = 'black';
            break;
        case '.':
            color = 'lightblue';
            break;
        default:
            if(node.isKey) {
                color = 'lightgoldenrodyellow';
            } else if(node.isDoor) {
                color = 'goldenrod';
            } else {
                color = 'green'
            }
            break;

    }
    const html = `<span 
                style="
                    --x:${node.x + 1};
                    --y:${node.y + 1};
                    --color:${color};"
                >
                ${node.isKey || node.isDoor ? node.charValue : ''}
                </span>`;
    return html;
}
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
            grid-template-columns: repeat(${width}, 20px);
            grid-template-rows: repeat(${height}, 20px);
        }
        span {
            display: block;
            grid-column: var(--x);
            grid-row: var(--y);
            background-color: var(--color);
            text-align: center;
            font-size: 16px;
            font-weight: bold;
        }
    </style>
</head>
<body>
    ${pixels}
</body>
</html>`;

const getHtmlDocument = nodes => {
    
    const mapWidth = Math.max(...nodes.map(({ x }) => x));
    const mapHeight = Math.max(...nodes.map(({ y }) => y));

    const pixels = nodes
        .map(toPixel)
        .join('');

    const doc = toHtmlDocument(pixels, mapWidth + 1, mapHeight + 1);

    return doc;
}

module.exports = getHtmlDocument;