const findShortestPath = graph => {
    const nodes = Object.keys(graph);
    return nodes.map(node => graph[node])
}

module.exports = findShortestPath;