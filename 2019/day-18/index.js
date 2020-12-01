const clonedeep = require('lodash.clonedeep');
const { getMap, getMapKey } = require('./get-map');
const getHtmlMap = require('./get-html-map');
const { getAvailableKeys } = require('./a-star');

let KEY_COUNT = 0;
let CHAIN_MAP = {};
let KEY_MAP = [];

const getChainKey = (playerNode, keys) => {
	const playerKey = getMapKey(playerNode);
	const keyMap = [...KEY_MAP];
	keys.forEach(key => {
		keyMap[key.node.value] = 1
	})
	return `${playerKey}${keyMap.join('')}`;
}

const getStepScores = (theMap, playerPositions) => {
	let playerIndex = 0;
	let keys = [];
	let steps = 0
	while(true) {
		const playerPosition = playerPositions[playerIndex];
		
		const stepScore = getStepScore(theMap, playerPosition, keys);
		keys = stepScore.keys;
		steps += stepScore.steps;
		playerPositions[playerIndex] = stepScore.playerPosition;

		playerIndex = (playerIndex + 1) % playerPositions.length;

		if(keys.length === KEY_COUNT) {
			return {
				steps,
				keys
			}
		}
	}
}

const getStepScore = (theMap, playerPosition, keys) => {
	const storedResult = CHAIN_MAP[getChainKey(playerPosition, keys)];
	if (storedResult) {
		return storedResult;
	}

	Object.assign(playerPosition, {
		keys: keys.map(key => key.node.value)
	})

	const closestKeys = getAvailableKeys(
		playerPosition,
		theMap
	);

	const minCount = closestKeys.reduce((best, newKey) => {
		const newKeys = [...keys, newKey];
		const keyScore = getStepScore(theMap, { x: newKey.node.x, y: newKey.node.y }, newKeys);
		if(best.steps > (keyScore.steps + newKey.steps)) {
			return {
				pickedKey: newKey,
				keys: keyScore.keys,
				playerPosition: keyScore.playerPosition,
				steps: keyScore.steps + newKey.steps,
			};
		}
		return best;
	}, { steps: Infinity, keys: [] })

	const chainKey = getChainKey(playerPosition, keys);
	if(minCount.steps === Infinity) {
		CHAIN_MAP[chainKey] = {
			steps: 0,
			keys,
			playerPosition
		};
		return {
			steps: 0,
			keys,
			playerPosition
		}
	} else {
		CHAIN_MAP[chainKey] = minCount;
		return {
			steps: minCount.steps,
			keys: minCount.keys,
			playerPosition: minCount.playerPosition
		}
	}
}

const input = require('../utils').getStringFromFile('./day-18/test.txt');
const theMap = getMap(input);
const doc = getHtmlMap(Object.values(theMap));

require('fs').writeFileSync('./day-18/map.html', doc);

const nodes = Object.values(theMap);
KEY_COUNT = nodes.filter(node => node.isKey).length;
console.log({
	KEY_COUNT
});
KEY_MAP = new Array(KEY_COUNT).fill(0);

const players = nodes.filter(n => n.isPlayer);
console.time('Took')
const score = getStepScores(theMap, players.map(({ x, y }) => ({ x, y })));//getStepScore(theMap, players.map(({ x, y }) => ({ x, y }))[0], []);
console.log('Done');
console.log(score.steps)
console.log(score.keys.map(k => k.node.charValue))
console.timeEnd('Took')
