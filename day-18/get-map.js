const getMapKey = ({ x, y }) => `(${x},${y})`;

const getMap = input => {
    const nodes = input.split('\n').reduce((nodes, row, rowIndex) => {
        const columns = row.split('').map((column, columnIndex) => {
            const isNotKeyOrDoor = column === '.' || column === '#';
            let isKey = false;
            let isDoor = false;
            let isPlayer = false;
            let value = column;

            if (!isNotKeyOrDoor) {
                if(column === '@') {
                    isPlayer = true;
                } else {
                    isKey = (column.charCodeAt(0) / 97 | 0) === 1;
                    isDoor = (column.charCodeAt(0) / 97 | 0) === 0;
    
                    if(isKey) {
                        value = (column.charCodeAt(0) % 97);
                    } else if(isDoor) {
                        value = (column.charCodeAt(0) % 65);
                    } else {
                        return null;
                    }
                }
            }

            return {
                x: columnIndex,
                y: rowIndex,
                value,
                isKey,
                isDoor,
                isPlayer,
                charValue: column
            }
        })

        nodes.push(...columns.filter(c => c ? true : false));
        return nodes;
    }, []);
    const theMap = nodes.reduce((theMap, node) => {
        theMap[getMapKey(node)] = node;
        return theMap;
    }, {})
    return theMap;
}

module.exports = {
    getMap,
    getMapKey
}