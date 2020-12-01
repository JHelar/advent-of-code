const readline = require('readline');
const rl = readline.createInterface({
	input: process.stdin,
	output: process.stdout
});

const COLORS = {
	BLACK: '0',
	WHITE: '1'
};

const DIRECTIONS = [
	0, // Up
	1, // Right
	2, // Down
	3 // Left
];

const getMapKey = ({ x, y }) => `(${x},${y})`;
let debug = true;
console.tryLog = msg => {
	if (debug) {
		console.log(msg);
	}
};

const robotMap = program => {
	const map = {};
	let position = {
		x: 0,
		y: 0
	};
	let facing = DIRECTIONS[0];
	let startPanel = true;

	while (program.state === 'ON') {
		const currentColor = startPanel ? COLORS.WHITE : map[getMapKey(position)] || COLORS.BLACK;
		startPanel = false;
		const [nextColor, nextDirection] = program.run(currentColor);

		if (!nextColor) break;
		map[getMapKey(position)] = nextColor;

		if (nextDirection == '0') {
			facing = DIRECTIONS[(facing + 3) % 4];
		} else if (nextDirection == '1') {
			facing = DIRECTIONS[(facing + 1) % 4];
		} else {
			throw new Error('Invalid direction');
		}

		if (facing === 0) {
			position = {
				...position,
				y: position.y + 1
			};
		} else if (facing === 1) {
			position = {
				...position,
				x: position.x + 1
			};
		} else if (facing === 2) {
			position = {
				...position,
				y: position.y - 1
			};
		} else if (facing === 3) {
			position = {
				...position,
				x: position.x - 1
			};
		} else {
			throw new Error('Invalid position')
		}
	}
	return map;
};

module.exports = robotMap;
