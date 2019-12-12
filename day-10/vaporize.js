const {
	contains,
	getDistance,
	getAngle,
	getPointFromAngle,
	getPointFromAngleAndDistance
} = require('./line');
const isNotPoint = myPoint => point => point.name !== myPoint.name;

const setDistance = (lazor, astroidMap) => astroidMap.map(target => ({ distance: getDistance(lazor, target), ...target }));
const sortByClosest = (a, b) => a.distance - b.distance;

const getTargets = (lazor, toPoint, astroidMap) =>
	astroidMap
		.filter(point => contains(lazor, toPoint, point))
		.map(target => ({ distance: getDistance(lazor, target), ...target }));

const getTargetsByAngle = (lazor, astroidMap, lazorAngle) => astroidMap.filter(point => {
	const angle = getAngle(lazor, point) | 0;
	return angle === lazorAngle;
})

const getClosestTarget = targets => targets.sort(sortByClosest)[0];

const rotate = (lazor, astroidMap) => {
	let lookUpMap = [...astroidMap];
	let vaporized = [];
	let stopAt = 0;
	for (let angle = 90; angle >= -90; angle -= 1) {
	
		const targets = getTargetsByAngle(lazor, lookUpMap, angle);
		if (targets.length) {
			const vaporize = getClosestTarget(targets);
			vaporized.push({...vaporize, angle});

			lookUpMap = lookUpMap.filter(isNotPoint(vaporize));
		}
	}

	return {
		astroidMap: lookUpMap,
		vaporized
	};
};

const machineGun = (lazor, astroidMap) => {
	let lookUpMap = setDistance(lazor, [...astroidMap]).filter(isNotPoint(lazor)).sort(sortByClosest);
	let rotations = 0;
	let vaporized = [];
	let shoot = true;
	
	while (shoot) {
		const rotationResult = rotate(lazor, lookUpMap);
		lookUpMap = rotationResult.astroidMap;
		if (rotationResult.vaporized.length) {
			vaporized = vaporized.concat(rotationResult.vaporized);
			rotations++;
		} else {
			break;
		}
	}

	return {
		rotations,
		vaporized
	};
};

module.exports = {
	rotate,
	machineGun
};
