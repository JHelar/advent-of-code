const clonedeep = require('lodash.clonedeep');
const { getMap, getMapKey } = require('./get-map');
const getHtmlMap = require('./get-html-map');
const { getAvailableKeys } = require('./a-star');

let KEY_COUNT = 0;
let SHORTEST_CHAIN = Infinity;
let CHAIN_MAP = {};

const getChainKey = (playerNode, keys) =>
	getMapKey(playerNode) +
	keys.reduce((sum, key, index) => sum + key.value, 0);

const getStepCount = chain => chain.reduce((sum, step) => sum + step.steps, 0);
const runWithKeys = (nodes, keys, theMap, allChains) => {
	const nodesCopy = clonedeep(nodes);
	const playerNode = nodesCopy.find(node => node.isPlayer);
	Object.assign(playerNode, {
		keys: keys.map(key => key.node.value)
	});
	const closestKeys = getAvailableKeys(nodesCopy, playerNode, theMap);
	if (!closestKeys.length) {
		if (keys.length === KEY_COUNT) {
			const chainLength = getStepCount(keys);
			if (chainLength < SHORTEST_CHAIN) {
				SHORTEST_CHAIN = chainLength;
				console.log({
					SHORTEST_CHAIN,
					keys: keys.map(k => k.node.charValue).join(','),
					date: new Date().toTimeString()
				});
				allChains.push(keys);
			} else {
				console.log({
					chainLength,
					keys: keys.map(k => k.node.charValue).join(','),
					date: new Date().toTimeString()
				});
			}
			// console.log('Chain done, chain count: ' + allChains.length)
		}
	} else {
		closestKeys.forEach(key => {
			playerNode.x = key.node.x;
			playerNode.y = key.node.y;
			const newKeys = [...keys, key];
			const remainingKeys = nodes.filter(
				node =>
					node.isKey &&
					!newKeys.some(nk => nk.node.value === node.value)
			);
			const chainCost = CHAIN_MAP[getChainKey(playerNode, remainingKeys)];
			const myChainCost = getStepCount(newKeys);
			if (!chainCost || myChainCost < chainCost) {
				CHAIN_MAP[getChainKey(playerNode, remainingKeys)] = myChainCost;
				// Only continue if it is worth it
				if (myChainCost < SHORTEST_CHAIN) {
					runWithKeys(nodesCopy, [...keys, key], theMap, allChains);
				}
			}
		});
	}
};
const input = require('../utils').getStringFromFile('./day-18/input.txt');
const theMap = getMap(input);
const doc = getHtmlMap(Object.values(theMap));

require('fs').writeFileSync('./day-18/map.html', doc);

const nodes = Object.values(theMap);
let allChains = [];
KEY_COUNT = nodes.filter(node => node.isKey).length;
console.log({
	KEY_COUNT
});
runWithKeys(nodes, [], theMap, allChains);
console.log('Done with result');
// let allChains = [];
// result.map(step => getChainSteps(step, [], allChains));
allChains = allChains.map(chain => {
	return chain.reduce(
		(newChain, step) => {
			newChain.keys.push(step.node.charValue);
			newChain.steps = newChain.steps + step.steps;
			return newChain;
		},
		{ keys: [], steps: 0 }
	);
});
require('fs').writeFileSync('./day-18/result.json', JSON.stringify(allChains));
