const clonedeep = require('lodash.clonedeep');

const positionsString = require('../utils').getStringFromFile('./day-12/input.txt');

const parsePosition = positionString => {
    const { x, y, z } = positionString.match(/x=(?<x>[-\d]*), y=(?<y>[-\d]*), z=(?<z>[-\d]*)/).groups
    return {
        x: x | 0,
        y: y | 0,
        z: z | 0
    }
}
const velocityChange = (a, b) => a === b ? 0 : a > b ? -1 : 1;
const add = (v1, v2) => ({ x: v1.x + v2.x, y: v1.y + v2.y, z: v1.z + v2.z });

const applyGravityTo = a1 => a2 => {
    const a1Change = {
        x: velocityChange(a1.position.x, a2.position.x),
        y: velocityChange(a1.position.y, a2.position.y),
        z: velocityChange(a1.position.z, a2.position.z)
    }

    Object.assign(a1, {
        velocity: add(a1.velocity, a1Change)
    })
}

const applyVelocityTo = a1 => () => {
    Object.assign(a1, {
        position: add(a1.position, a1.velocity)
    })
}

const isVelocityZero = ({ velocity }) => velocity.x === 0 && velocity.y === 0 && velocity.z === 0;
const getKineticEnergy = ({ velocity }) => Math.abs(velocity.x) + Math.abs(velocity.y) + Math.abs(velocity.z);
const getPotentialEnergy = ({ position }) => Math.abs(position.x) + Math.abs(position.y) + Math.abs(position.z);

const getMoonEnergy = moon => {
    const potential = getPotentialEnergy(moon);
    const kinetic = getKineticEnergy(moon);

    return potential * kinetic;
}

const positions = positionsString.split('\n').map(parsePosition).map((position, index) => ({
    position,
    velocity: {
        x: 0,
        y: 0,
        z: 0
    },
    energy: 0,
    name: `moon-${index}`
}));

const getLcm = (x, y) => {
    function gcd_two_numbers(x, y) {
        x = Math.abs(x);
        y = Math.abs(y);
        while (y) {
            var t = y;
            y = x % y;
            x = t;
        }
        return x;
    }

    if ((typeof x !== 'number') || (typeof y !== 'number'))
        return false;
    return (!x || !y) ? 0 : Math.abs((x * y) / gcd_two_numbers(x, y));
}



const makeTimeStep = (moon, rest) => {
    const applyGravity = applyGravityTo(moon);
    const applyVelocity = applyVelocityTo(moon);
    const startPosition = {...moon.position}
    return {
        applyGravity: () => {
            rest.forEach(other => {
                applyGravity(other);
            })
        },
        applyVelocity,
        startPosition,
        get velocity() {
            return moon.velocity
        },
        get position() {
            return moon.position
        }
    }
}

const getCycles = (positions, checkFor) => {
    const pos = [...positions].map(pos => clonedeep(pos));

    const moons = pos.map((moon, index) => {
        return makeTimeStep(moon, pos.filter(pm => pm.name !== moon.name))
    });
    
    let stepCount = 1;
    moons.forEach(timeStep => timeStep.applyGravity());
    moons.forEach(timeStep => timeStep.applyVelocity());
    while (!moons.every(moon => moon.velocity[checkFor] === 0)) {
        moons.forEach(timeStep => timeStep.applyGravity());
        moons.forEach(timeStep => timeStep.applyVelocity());
        stepCount++;
    }

    return stepCount * 2;
}

// let [io, europa, ganymede, callisto] = positions;

// const ioTimeStep = makeTimeStep(io, [europa, ganymede, callisto]);
// const europaTimeStep = makeTimeStep(europa, [io, ganymede, callisto]);
// const ganymedeTimeStep = makeTimeStep(ganymede, [io, europa, callisto]);
// const callistoTimeStep = makeTimeStep(callisto, [io, ganymede, europa]);
// const moons = [ioTimeStep, europaTimeStep, ganymedeTimeStep, callistoTimeStep];

const cycles = ['x', 'y', 'z'].map(checkFor => getCycles(positions, checkFor));
const sum = cycles.reduce((acc, x) => getLcm(acc, x), 1);

console.log({
    sum
})
// console.log(`Resulting steps: ${ioTimeStep.orbitLength}.`)