import { ParseOptions } from "querystring";
import { readFileWithSeparator } from "../utils";

interface Tile {
	x: number;
	y: number;
	black: boolean;
}

type Vector = [x: number, y: number];

const goTowards = (point: Tile, [x, y]: Vector): Tile => ({
	x: point.x + x,
	y: point.y + y,
	black: point.black,
});

const getTileKey = ({ x, y }: Tile): string => `(${x},${y})`;

const calcBlackTiles = (map: Record<string, Tile>) =>
	Object.values(map).reduce((sum, { black }) => sum + (black ? 1 : 0), 0);

export default () => {
	const commands = readFileWithSeparator("day-24/input.txt", "\n").map(
		(tileString) => {
			const commands: Vector[] = [];
			for (let i = 0; i < tileString.length; i++) {
				const one = tileString[i];
				const two = tileString[i + 1];

				if (one === "e") {
					commands.push([2, 0]);
				} else if (one === "s") {
					if (two === "e") {
						commands.push([1, -1]);
						i++;
					} else if (two === "w") {
						commands.push([-1, -1]);
						i++;
					}
				} else if (one === "w") {
					commands.push([-2, 0]);
				} else if (one === "n") {
					if (two === "w") {
						commands.push([-1, 1]);
						i++;
					} else if (two === "e") {
						commands.push([1, 1]);
						i++;
					}
				}
			}
			return commands;
		}
	);
	let map: Record<string, Tile> = {};
	commands.forEach((command) => {
		let currentTile: Tile = { x: 0, y: 0, black: true };
		command.forEach((vector) => {
			currentTile = goTowards(currentTile, vector);
		});
		const tileKey = getTileKey(currentTile);
		if (tileKey in map) {
			delete map[tileKey];
		} else {
			map[tileKey] = currentTile;
		}
	});

	const days = 100;
	const adjacentVec: Vector[] = [
		[2, 0],
		[-2, 0],
		[1, -1],
		[-1, -1],
		[1, 1],
		[-1, 1],
	];
	for (let day = 0; day < days; day++) {
		const newMap: Record<string, Tile> = {};
		let minX = Infinity;
		let maxX = -Infinity;
		let minY = Infinity;
		let maxY = -Infinity;

		Object.values(map).forEach(({ x, y }) => {
			minX = Math.min(minX, x);
			maxX = Math.max(maxX, x);

			minY = Math.min(minY, y);
			maxY = Math.max(maxY, y);
		});

		for (let j = minX - 2; j < maxX + 2 + 1; j++) {
			for (let k = minY - 1; k < maxY + 1 + 1; k++) {
				const thisTileKey = getTileKey({ x: j, y: k, black: true });
				const blackTiles = adjacentVec
					.map(([x, y]) =>
						getTileKey({ x: x + j, y: y + k, black: true })
					)
					.reduce((sum, key) => {
						if (key in map) {
							return sum + 1;
						}
						return sum;
					}, 0);
				if (thisTileKey in map) {
					if (blackTiles > 0 && blackTiles <= 2) {
						newMap[thisTileKey] = {
							x: j,
							y: k,
							black: true,
						};
					}
				} else if (blackTiles === 2) {
					newMap[thisTileKey] = {
						x: j,
						y: k,
						black: true,
					};
				}
			}
		}
		map = newMap;
	}

	// return calcBlackTiles(tempMap);
	return Object.values(map).length;
};
