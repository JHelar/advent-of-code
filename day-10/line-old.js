const getSlope = (onePoint, anotherPoint) => (anotherPoint.y - onePoint.y) / (anotherPoint.x - onePoint.x);
const getB = (onePoint, anotherPoint) => {
    const slope = getSlope(onePoint, anotherPoint);
    return onePoint.y - slope * onePoint.x;
}
const getSlopeFromAngle = angle => {
	const radians = angle * Math.PI / 180;
	return angle === 90 ? 0 : Math.tan(radians);
}
const getMFromAngle = (point, angle) => {
	// y = kx + m
	const k = getSlopeFromAngle(angle);
	// m = kx - y;
	const m = (k * point.x) + point.y;

	return m;
}
const containsWithAngle = (fromPoint, withPoint, angle) => {
	const k = getSlopeFromAngle(angle);
	const m = getMFromAngle(fromPoint, angle);
	
	// y = kx + m
	return withPoint.y === k * withPoint.x + m;
}

const isVertical = (onePoint, anotherPoint) => onePoint.x === anotherPoint.x && onePoint.y !== anotherPoint.y;
const isHorizontal = (onePoint, anotherPoint) => onePoint.x !== anotherPoint.x && onePoint.y === anotherPoint.y;
const getDistance = (onePoint, anotherPoint) => Math.sqrt(Math.pow(anotherPoint.x - onePoint.x, 2) + Math.pow(anotherPoint.y - onePoint.y, 2));
const getAngle = (origin, point) => {
	const deltaX = point.x - origin.x;
	const deltaY = origin.y - point.y;
	const radians = Math.atan2(deltaY, deltaX);
	const degrees = radians * (Math.PI / 180);

	return degrees;
}
const getPointFromAngleAndDistance = (point, distance, angle) => {
	const radians = angle * (Math.PI / 180);
	const x = distance * Math.cos(radians) + point.x;
	const y = distance * Math.sin(radians) + point.y;

	return {
		x,
		y
	}
}
const getPointFromAngle = (origin, point, angle) => {
	const radians = angle * (Math.PI / 180);
	const s = Math.sin(radians);
	const c = Math.cos(radians);

	const x = ((point.x - origin.x) * c - (point.y - origin.y) * s) + origin.x;
	const y = ((point.x - origin.x) * s + (point.y - origin.y) * c) + origin.y;

	return {
		x,
		y
	}
}
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
	getDistance,
	getPointFromAngle,
	getPointFromAngleAndDistance,
	getAngle,
	containsWithAngle
}