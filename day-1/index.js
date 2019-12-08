const getAddedFuelRequirementFromModules = require('./get-added-fuel-requirement').getAddedFuelRequirementFromModules

const modules = require('../utils').readModules('./day-1/input.txt');

const fuelAmount = getAddedFuelRequirementFromModules(modules);

console.log('Santa needs ')
console.log({
    fuelAmount
})