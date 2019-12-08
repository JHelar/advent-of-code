// Advent of code day 1 part 2

const getFuelRequirement = require('../get-fuel-requirement').getFuelRequirement;

const utils = require('../../utils');

const getAddedFuelRequirement = mass => {
    let sum = 0;
    let new_mass = getFuelRequirement(mass);

    while(new_mass > 0) {
        sum += new_mass;
        new_mass = getFuelRequirement(new_mass);
    }

    return sum;
}

const getAddedFuelRequirementFromModules = modules => modules.map(getAddedFuelRequirement).reduce(utils.sumReducer, 0)

module.exports = {
    getAddedFuelRequirement,
    getAddedFuelRequirementFromModules
}