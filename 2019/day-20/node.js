"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Portal = exports.Node = exports.getNodeKey = void 0;
const getNodeKey = (x, y, level = 0) => `(${x},${y},${level})`;
exports.getNodeKey = getNodeKey;
class Node {
    constructor(x, y, sign) {
        this.h = 0;
        this.level = 0;
        this.neighbours = [];
        this.visited = false;
        this.x = x;
        this.y = y;
        this.name = sign;
        this.road = sign === ".";
        this.air = sign === " ";
        this.wall = sign === "#";
    }
    getKey() {
        return exports.getNodeKey(this.x, this.y, this.level);
    }
    toString() {
        return `{${this.name}|${this.getKey()}}|[${this.neighbours
            .map((n) => `{${n.name}|${n.getKey()}}`)
            .join(",")}]}`;
    }
}
exports.Node = Node;
class Portal extends Node {
    constructor(x, y, label) {
        super(x, y, ".");
        this.name = label;
        this.portalExit = null;
    }
}
exports.Portal = Portal;
