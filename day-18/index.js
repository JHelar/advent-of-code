const clonedeep = require('lodash.clonedeep')
const { getMap } = require('./get-map');
const getHtmlMap = require('./get-html-map');
const { getAvailableKeys } = require('./a-star');

const runWithKeys = (nodes, keys, theMap) => {
    const nodesCopy = clonedeep(nodes);
    const playerNode = nodesCopy.find(node => node.isPlayer);
    Object.assign(playerNode, {
        keys: [...keys.map(key => key.node.value)]
    });
    const closestKeys = getAvailableKeys(nodesCopy, playerNode, theMap);
    const chains = [];
    closestKeys.forEach(key => {
        const chain = [key];
        playerNode.x = key.node.x;
        playerNode.y = key.node.y;
        
        chain.push(...runWithKeys(nodesCopy, [...keys, key], theMap))
        chains.push(chain);
    });

    return chains;
}
const input = require('../utils').getStringFromFile('./day-18/test.txt');

const theMap = getMap(input);
const nodes = Object.values(theMap);
const result = runWithKeys(nodes, [], theMap)

const doc = getHtmlMap(Object.values(theMap));

require('fs').writeFileSync('./day-18/map.html', doc);
require('fs').writeFileSync('./day-18/result.json', JSON.stringify(result));