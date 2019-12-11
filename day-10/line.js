const getSlope = (onePoint, anotherPoint) => (anotherPoint.y - onePoint.y) / (anotherPoint.x - onePoint.x);
const getB = (onePoint, anotherPoint) => {
    const slope = getSlope(onePoint, anotherPoint);
    return onePoint.y - slope * onePoint.x;
}
const isVertical = (onePoint, anotherPoint) => onePoint.x === anotherPoint.x && onePoint.y !== anotherPoint.y;
const isHorizontal = (onePoint, anotherPoint) => onePoint.x !== anotherPoint.x && onePoint.y === anotherPoint.y;
const getLength = (onePoint, anotherPoint) => Math.sqrt(Math.pow(anotherPoint.x - onePoint.x, 2) + Math.pow(anotherPoint.y - onePoint.y, 2));
const contains = (point1, point2, withPoint) => {
    const crossProduct = (withPoint.y - point1.y) * (point2.x - point1.x) - (withPoint.x - point1.x) * (point2.y - point1.y);
    if (Math.abs(crossProduct) > Number.EPSILON) return false;

    const dotProduct = (withPoint.x - point1.x) * (point2.x - point1.x) + (withPoint.y - point1.y) * (point2.y - point1.y);
    if (dotProduct < 0) return false;

    const squaredLength = (point2.x - point1.x) * (point2.x - point1.x) + (point2.y - point1.y) * (point2.y - point1.y);
    if (dotProduct > squaredLength) return false;

    return true;
}

module.exports = {
    contains,
    getSlope,
    getB,
    getLength
}