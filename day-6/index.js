const getOrbitGraph = require('./get-orbit-graph');
const findShortestPath = require('./find-shortest-path');

const modules = require('../utils').readModules('./day-6/input.txt');

const graph = getOrbitGraph(modules);
const paths = findShortestPath(graph);

const checksum = paths.map(path => path.distance).reduce((sum, distance) => sum + distance, 0);
const distance = graph['YOU'].distance - 2;
// require('fs').writeFileSync('dump.json', JSON.stringify(graph))
console.log({
    checksum,
    distance
})