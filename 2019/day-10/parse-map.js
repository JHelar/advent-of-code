const getPositionRow = (mapRow, rowIndex) => mapRow.split('').reduce((row, data, columnNum) => {
    if (data === '#') {
        row.push({
            x: columnNum,
            y: rowIndex,
            name: `(${columnNum},${rowIndex})`
        })
    } else if (data === 'X') {
        row.push({
            x: columnNum,
            y: rowIndex,
            name: `LAZOR`
        })
    }
    return row;
}, [])

const parseMap = mapString => {
    const rowStrings = mapString.split('\n');
    const astroidMap = rowStrings.reduce((positions, mapRow, rowIndex) => {
        const row = getPositionRow(mapRow, rowIndex);
        if(row.length) {
            return positions.concat(row);
        }
        return positions;
    }, []);

    const lazor = astroidMap.find(point => point.name === 'LAZOR')

    return {
        astroidMap: astroidMap.filter(point => point.name !== 'LAZOR'),
        mapHeight: rowStrings.length,
        mapWidth: rowStrings[0].length,
        lazor
    }
}

module.exports = parseMap;