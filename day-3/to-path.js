const Directions = {
    U: {
        x: 0,
        y: 1
    },
    D: {
        x: 0,
        y: -1
    },
    L: {
        x: -1,
        y: 0
    },
    R: {
        x: 1,
        y: 0
    }
}

const getCoordinate = (instruction, prevCoordinate) => {
    const direction = Directions[instruction[0]];
    const amount = instruction.slice(1) | 0;

    return {
        x: prevCoordinate.x + direction.x * amount,
        y: prevCoordinate.y + direction.y * amount,
        length: prevCoordinate.length + amount
    }
}

const toPath = instructions => instructions.reduce(({ prev, path }, instruction) => {
    const coordinate = getCoordinate(instruction, prev);
    path.push(coordinate);

    return {
        prev: coordinate,
        path
    }

}, { prev: { x: 0, y: 0, length: 0 }, path: [{x: 0, y: 0, length: 0}] })
    .path;

module.exports = {
    getCoordinate,
    toPath
}