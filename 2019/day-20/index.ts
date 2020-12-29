import fs from "fs";
import { Portal, Node, getNodeKey } from "./node";
import { aStar } from "./aStar";
import { toHtmlDocument, toPixel } from "./toHtml";

const labelPerRow: Record<
	number,
	{ name: string[]; rowIndex: number; colIndex: number }[]
> = {};
const labelPerColumn: Record<
	number,
	{ name: string[]; rowIndex: number; colIndex: number }[]
> = {};
const stringMap = fs
	.readFileSync("day-20/input.txt")
	.toString()
	.split("\n")
	.map((row, rowIndex) => {
		const column: string[] = [];
		row.split("").forEach((c, colIndex) => {
			column.push(c);
			if (c !== "." && c !== "#" && c !== " ") {
				if (!(colIndex in labelPerColumn)) {
					labelPerColumn[colIndex] = [];
				}

				const colLabel = labelPerColumn[colIndex].find(
					(c) => c.rowIndex + 1 === rowIndex
				);
				if (colLabel) {
					colLabel.name.push(c);
				} else {
					labelPerColumn[colIndex].push({
						name: [c],
						rowIndex,
						colIndex,
					});
				}

				if (!(rowIndex in labelPerRow)) {
					labelPerRow[rowIndex] = [];
				}
				const rowLabel = labelPerRow[rowIndex].find(
					(r) => r.colIndex + 1 === colIndex
				);
				if (rowLabel) {
					rowLabel.name.push(c);
				} else {
					labelPerRow[rowIndex].push({
						name: [c],
						colIndex,
						rowIndex,
					});
				}
			}
		});
		return column;
	})
	.filter((row) => row.length);

const isMapItem = (x: number, y: number) => {
	const row = stringMap[y];
	if (row) {
		return row[x] === "." ? true : false;
	}
	return false;
};

const labels = [
	...Object.values(labelPerColumn).flatMap((labels) =>
		labels.map(({ name, colIndex, rowIndex }) => ({
			name: name.join(""),
			x: colIndex,
			y:
				rowIndex +
				(isMapItem(colIndex, rowIndex + 2)
					? 2
					: isMapItem(colIndex, rowIndex - 1)
					? -1
					: 0),
		}))
	),
	...Object.values(labelPerRow).flatMap((labels) =>
		labels.map(({ name, colIndex, rowIndex }) => ({
			name: name.join(""),
			x:
				colIndex +
				(isMapItem(colIndex + 2, rowIndex)
					? 2
					: isMapItem(colIndex - 1, rowIndex)
					? -1
					: 0),
			y: rowIndex,
		}))
	),
].filter(({ name }) => name.length === 2);

const map = stringMap.reduce((map, row, rowIndex) => {
	row.forEach((col, columnIndex) => {
		const label = labels.find(
			(l) => l.x === columnIndex && l.y === rowIndex
		);
		if (label) {
			const node = new Portal(columnIndex, rowIndex, label.name);
			map[node.getKey()] = node;
		} else {
			const node = new Node(columnIndex, rowIndex, col);
			map[node.getKey()] = node;
		}
	});
	return map;
}, {} as Record<string, Node>);

const nodes = Object.values(map)
	.map((node, ni, nodes) => {
		node.neighbours = [
			[0, 1],
			[0, -1],
			[1, 0],
			[-1, 0],
		]
			.map(([x, y]) => map[getNodeKey(node.x + x, node.y + y)])
			.filter((n) => n && n.road);
		if (node instanceof Portal) {
			const portalEnd = nodes.find(
				(n) =>
					n instanceof Portal &&
					n.name === node.name &&
					n.getKey() !== node.getKey()
			);
			if (portalEnd) {
				node.neighbours.push(portalEnd);
			}
		}
		return node;
	})
	.filter((node) => node.road);

const path = aStar(nodes);
let count = 0;
if (path) {
	path.node.visited = true;
	let lookAt = path.prev;
	while (lookAt) {
		count++;
		lookAt.node.visited = true;
		lookAt = lookAt.prev;
	}
}

const pixels = Object.values(map).map(toPixel);
const document = toHtmlDocument(pixels, 800, 800);
fs.writeFileSync("day-20/map.html", document);
console.log({ count });

// // console.log(JSON.stringify(labelPerColumn, null, 2));

// nodes.forEach((n) => {
// 	if (n instanceof Portal) {
// 		console.log(n.toString());
// 	}
// });
