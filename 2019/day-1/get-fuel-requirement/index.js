// Advent of code day 1
const utils = require('../../utils')

const getFuelRequirement = mass => Math.floor(mass / 3) - 2;

const getFuelRequirementFromModules = modules => modules.map(getFuelRequirement).reduce(utils.sumReducer, 0)

module.exports = {
    getFuelRequirement,
    getFuelRequirementFromModules
}