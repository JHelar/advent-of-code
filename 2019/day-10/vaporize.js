const {
	angleTo,
	getAngle,
	getDistance,
} = require('./line');

let baseAngle = -270;
const isNotPoint = myPoint => point => point.name !== myPoint.name;

const sortByClosest = (a, b) => a.distance - b.distance;
const setAngle = (lazor, astroidMap, angle) => astroidMap.map(point => ({angle: angleTo(lazor, point, baseAngle), ...point}));
const setDistanceGroups = (lazor, astroidMap) => Object.values(astroidMap
	.map(point => ({...point, distance: getDistance(lazor, point)}))
	.reduce((angleMap, point) => {
		if(point.angle in angleMap) {
			angleMap[point.angle].push(point)
			angleMap[point.angle].sort(sortByClosest)
		} else {
			angleMap[point.angle] = [point]
		}
		return angleMap
	}, {}))
	
const vaporize = (lazor, astroidMap) => {
	astroidMap = astroidMap.filter(isNotPoint(lazor));
	let newMap = setDistanceGroups(lazor, setAngle(lazor, astroidMap)).sort((a, b) => a[0].angle - b[0].angle);
	// Set angle 0 to end of map
	const angle0Group = newMap.filter(a => a[0].angle === 0);
	newMap = newMap.filter(a => a[0].angle !== 0);
	newMap.push(...angle0Group);
	return newMap.reverse();
}

module.exports = vaporize;