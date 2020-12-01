const getMapKey = ({ x, y }) => `(${x},${y})`;

const setIntersection = (theMap, node) => {
    if(node.value !== '.') {
        const northPos = {
            ...node.pos,
            y: node.pos.y - 1
        }
        const southPos = {
            ...node.pos,
            y: node.pos.y + 1
        }
        const eastPos = {
            ...node.pos,
            x: node.pos.x + 1
        }
        const westPos = {
            ...node.pos,
            x: node.pos.x - 1
        }
    
        const neighbours = [northPos, southPos, eastPos, westPos].filter(pos => {
            const mapValue = theMap[getMapKey(pos)];
            return mapValue && mapValue.value !== '.'
        }).map(pos => theMap[getMapKey(pos)]);
    
        const isIntersection = neighbours.length > 2;

        Object.assign(node, {
            neighbours,
            isIntersection
        })
    } else {
        Object.assign(node, {
            neighbours: [],
            isIntersection: false
        })
    }
}

const getMap = program => {
    const theMap = {};
    const position = {
        x: 0,
        y: 0
    }
    while(program.state === 'ON') {
        let [code] = program.run(undefined);
        code = code |0;
        if(code !== 10 && code !== 0) {
           theMap[getMapKey(position)] = {
               pos: {
                   ...position
               },
               value: String.fromCharCode(code),
               code
           }
           position.x = position.x + 1;
        } else {
            position.y = position.y + 1;
            position.x = 0;
        }
    }
    Object.values(theMap).forEach(node => {
        setIntersection(theMap, node);
    })
    return theMap;
}

module.exports = getMap;