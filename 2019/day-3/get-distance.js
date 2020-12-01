const getDistance = (fromPoint, toPoint) => {
    const x = Math.abs(fromPoint.x - toPoint.x);
    const y = Math.abs(fromPoint.y - toPoint.y);

    return x + y;
}

module.exports = getDistance;