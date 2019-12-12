const toDegrees = radians => (radians * 180 / Math.PI)

const angleTo = (p1, p2, rotation = 0) => {
	const radians = Math.atan2(p2.y - p1.y, p2.x - p1.x);
	const angle = -toDegrees(radians) - rotation;

	return angle % 360;
}

const getAngle = (origin, point) => {
	const deltaX = point.x - origin.x;
	const deltaY = point.y - origin.y;
	const radians = Math.atan2(deltaY, deltaX);
	const degrees = toDegrees(radians);

	return degrees;
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

const getDistance = (onePoint, anotherPoint) => Math.sqrt(Math.pow(anotherPoint.x - onePoint.x, 2) + Math.pow(anotherPoint.y - onePoint.y, 2));

module.exports = {
	angleTo,
	contains,
    getDistance,
    getAngle
}