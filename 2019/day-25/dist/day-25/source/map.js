"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.MapView = void 0;
const react_1 = __importStar(require("react"));
const ink_1 = require("ink");
const getPointFromVector = (point, [x, y]) => ([point.x + x, point.y + y]);
const getMapKey = ([x, y]) => `${x},${y}`;
const MapView = ({ output, command }) => {
    const currentRoom = react_1.useRef({ x: 0, y: 0, north: false, south: false, east: false, west: false, active: true, visited: true });
    const map = react_1.useRef({}).current;
    const [pov, setPov] = react_1.useState([]);
    let relativeY = -1;
    let relativeX = -1;
    const setMapPov = () => {
        let newPov = Array(9).fill(0).map(() => Array(9).fill(0).map(() => ({
            active: false,
            door: 0,
            visited: false,
            x: 0,
            y: 0
        })));
        for (let y = 0; y < 3; y++) {
            for (let x = 0; x < 3; x++) {
                let room = map[getMapKey(getPointFromVector(currentRoom.current, [relativeX, relativeY]))];
                for (let innerY = 0; innerY < 3; innerY++) {
                    const roomY = (y * 3) + innerY;
                    for (let innerX = 0; innerX < 3; innerX++) {
                        const roomX = (x * 3) + innerX;
                        let node = newPov[roomY][roomX];
                        if ((relativeX === 0 && relativeY === -1 || relativeX === -1 && relativeY === 0 || relativeX === 0 && relativeY === 0 || relativeX === 1 && relativeY === 0 || relativeX === 0 && relativeY === 1) && room) {
                            if (innerY === 0 && innerX === 1) {
                                node.door = room.visited ? room.north ? 1 : -1 : 0;
                            }
                            else if (innerY === 1 && innerX === 0) {
                                node.door = room.visited ? room.west ? 1 : -1 : 0;
                            }
                            else if (innerY === 1 && innerX === 2) {
                                node.door = room.visited ? room.east ? 1 : -1 : 0;
                            }
                            else if (innerY === 2 && innerX === 1) {
                                node.door = room.visited ? room.south ? 1 : -1 : 0;
                            }
                            else if (innerY === 1 && innerX === 1) {
                                node.visited = room.visited;
                                node.door = 1;
                                node.active = relativeY === 0 && relativeX === 0;
                            }
                            else {
                                node.door = -1;
                            }
                        }
                    }
                }
                relativeX++;
                if (relativeX > 1) {
                    relativeX = -1;
                }
            }
            relativeY++;
        }
        setPov(newPov);
    };
    react_1.useEffect(() => {
        map[getMapKey([currentRoom.current.x, currentRoom.current.y])] = currentRoom.current;
    });
    react_1.useEffect(() => {
        if (command) {
            let newRoom = undefined;
            switch (command) {
                case 'north':
                    newRoom = getMapKey(getPointFromVector(currentRoom.current, [0, -1]));
                    break;
                case 'south':
                    newRoom = getMapKey(getPointFromVector(currentRoom.current, [0, 1]));
                    break;
                case 'east':
                    newRoom = getMapKey(getPointFromVector(currentRoom.current, [1, 0]));
                    break;
                case 'west':
                    newRoom = getMapKey(getPointFromVector(currentRoom.current, [-1, 0]));
                    break;
                default:
                    break;
            }
            if (newRoom) {
                if (newRoom in map) {
                    map[getMapKey([currentRoom.current.x, currentRoom.current.y])].active = false;
                    map[getMapKey([currentRoom.current.x, currentRoom.current.y])].visited = true;
                    map[newRoom].active = true;
                    map[newRoom].visited = true;
                    currentRoom.current = map[newRoom];
                }
            }
        }
        const doorsLeadIndex = output.findIndex(o => o.startsWith('Doors here lead:'));
        if (doorsLeadIndex > -1) {
            const spaceIndex = output.indexOf(' ', doorsLeadIndex);
            const positions = output.slice(doorsLeadIndex + 1, spaceIndex - 2);
            currentRoom.current.north = false;
            currentRoom.current.south = false;
            currentRoom.current.east = false;
            currentRoom.current.west = false;
            positions.map(p => p.slice(2)).forEach(pos => {
                let newRoom = undefined;
                switch (pos) {
                    case 'north':
                        newRoom = getPointFromVector(currentRoom.current, [0, -1]);
                        currentRoom.current.north = true;
                        break;
                    case 'south':
                        newRoom = getPointFromVector(currentRoom.current, [0, 1]);
                        currentRoom.current.south = true;
                        break;
                    case 'east':
                        newRoom = getPointFromVector(currentRoom.current, [1, 0]);
                        currentRoom.current.east = true;
                        break;
                    case 'west':
                        newRoom = getPointFromVector(currentRoom.current, [-1, 0]);
                        currentRoom.current.west = true;
                        break;
                    default:
                        break;
                }
                if (newRoom) {
                    const newRoomKey = getMapKey(newRoom);
                    if (!(newRoomKey in map)) {
                        map[newRoomKey] = {
                            north: false,
                            south: false,
                            east: false,
                            west: false,
                            active: false,
                            visited: false,
                            x: newRoom[0],
                            y: newRoom[1]
                        };
                    }
                }
            });
        }
        setMapPov();
    }, [output, command]);
    return react_1.default.createElement(ink_1.Box, { flexDirection: "column", borderColor: "cyanBright", borderStyle: "classic" }, pov.map((row, rowIndex) => react_1.default.createElement(ink_1.Box, { key: `row${rowIndex}`, flexDirection: "row" }, row.map((t, tileIndex) => react_1.default.createElement(ink_1.Box, { key: `row${rowIndex}-tile${tileIndex}`, width: 1, height: 1, borderColor: t.active ? 'greenBright' : t.visited ? 'cyan' : t.door === 1 ? 'white' : t.door === -1 ? 'red' : 'black', borderStyle: "double" })))));
};
exports.MapView = MapView;
