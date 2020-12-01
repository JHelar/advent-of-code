const toPath = require('./to-path').toPath;
const getIntersection = require('./get-instersection');
const getDistance = require('./get-distance');
const utils = require('../utils');

const chunk = size => array => {
    const chunked_arr = [];
    let index = 0;
    while (index < array.length) {
        chunked_arr.push(array.slice(index, size + index));
        index += size;
    }
    return chunked_arr;
}

const [ oneSet, anotherSet ] = utils.readModules('./day-3/input.txt')
    .map(module => module.split(','))
    .map(toPath);

const origin = {
    x: 0,
    y: 0
}

const intersections = [];

for(let setIndex = 0; setIndex < oneSet.length; setIndex++) {
    const point = [
        oneSet[setIndex],
        oneSet[setIndex + 1]
    ];

    for(let withIndex = 0; withIndex < anotherSet.length; withIndex++) {
        const anotherPoint = [
            anotherSet[withIndex],
            anotherSet[withIndex + 1]
        ]

        const intersection = getIntersection(point, anotherPoint);
        if(intersection) {
            const shortestOne = point[0].length < point[1].length ? point[0] : point[1];
            const shortestAnother = anotherPoint[0].length < anotherPoint[1].length ? anotherPoint[0] : anotherPoint[1];

            intersections.push({
                intersection,
                oneWire: shortestOne,
                anotherWire: shortestAnother
            });
        }
    }
}

const distances = intersections
    .map(({ intersection, oneWire, anotherWire }) => {
        const oneDistance = oneWire.length + (intersection.y !== oneWire.y ? Math.abs(intersection.y - oneWire.y) : Math.abs(intersection.x - oneWire.x));
        const anotherDistance = anotherWire.length + (intersection.y !== anotherWire.y ? Math.abs(intersection.y - anotherWire.y) : Math.abs(intersection.x - anotherWire.x));
        return oneDistance + anotherDistance;
    })

console.log(Math.min(...distances))