const makeNode = (name) => ({
    connections: [],
    name,
    visited: false,
    isStart: name === 'COM'
})

const setNodeLength = (graph, nodeName, predecessor) => {
    const node = graph[nodeName];
    if(!node.visited) {
        Object.assign(node, {
            visited: true,
            predecessorName: predecessor.name,
            distance: predecessor.distance + 1
        })

        node.connections.forEach(connectedNode => setNodeLength(graph, connectedNode, node))
    }
}

const setDistances = graph => {
    const rootNode = graph[Object.keys(graph).find(key => graph[key].isStart)];
    Object.assign(rootNode, {
        distance: 0,
        visited: true
    })
    rootNode.connections.forEach(connectedNode => setNodeLength(graph, connectedNode, rootNode));
}

const getOrbitGraph = modules => {
    const graph = modules.map(mod => mod.replace('\r', '').split(')')).reduce((graph, [ oneOrbitName, anotherOrbitName ]) => {
        const oneOrbit = graph[oneOrbitName] || makeNode(oneOrbitName);
        const anotherOrbit = graph[anotherOrbitName] || makeNode(anotherOrbitName);
        oneOrbit.connections.push(anotherOrbitName);
        anotherOrbit.connections.push(oneOrbitName);
    
        graph[oneOrbitName] = oneOrbit;
        graph[anotherOrbitName] = anotherOrbit;
    
        return graph
    }, {});
    setDistances(graph);
    return graph;
}

module.exports = getOrbitGraph;