export const getNodeKey = (x: number, y: number, level: number = 0) => `(${x},${y},${level})`;

export class Node {
	name: string;
	road: boolean;
	air: boolean;
	wall: boolean;
	x: number;
	y: number;
	h = 0;
	level = 0;
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
		return getNodeKey(this.x, this.y, this.level);
	}

	toString() {
		return `{${this.name}|${this.getKey()}}|[${this.neighbours
			.map((n) => `{${n.name}|${n.getKey()}}`)
			.join(",")}]}`;
	}
}

export class Portal extends Node {
	portalExit: Portal | null

	constructor(x: number, y: number, label: string) {
		super(x, y, ".");
		this.name = label;
		this.portalExit = null
	}
}
