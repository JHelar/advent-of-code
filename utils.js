const fs = require('fs');
const path = require('path');

const sumReducer = (acc, amount) => acc + amount;

const getStringFromFile = inputFileName => fs.readFileSync(path.join(__dirname, inputFileName)).toString()

const readModules = inputFileName => getStringFromFile(inputFileName).split('\n');
const readArray = inputFileName => getStringFromFile(inputFileName).split(',')

module.exports = {
    sumReducer,
    readModules,
    readArray
}