export const getNodeKey = (x: number, y: number) => `(${x},${y})`;

export class Node {
	name: string;
	road: boolean;
	air: boolean;
	wall: boolean;
	x: number;
	y: number;
	h = 0;
	neighbours: Node[] = [];
	visited: boolean = false;

	constructor(x: number, y: number, sign: string) {
		this.x = x;
		this.y = y;
		this.name = sign;
		this.road = sign === ".";
		this.air = sign === " ";
		this.wall = sign === "#";
	}

	getKey() {
		return getNodeKey(this.x, this.y);
	}

	toString() {
		return `{${this.name}|${this.getKey()}}|[${this.neighbours
			.map((n) => `{${n.name}|${n.getKey()}}`)
			.join(",")}]}`;
	}
}

export class Portal extends Node {
	constructor(x: number, y: number, label: string) {
		super(x, y, ".");
		this.name = label;
	}
}
