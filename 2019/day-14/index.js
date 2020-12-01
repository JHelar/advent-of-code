const flattenDeep = require('lodash.flattendeep');

const recipe = (name, count, ingredients) => ({
    name,
    count: count | 0,
    ingredients
})
const recipes = require('../utils').getStringFromFile('./day-14/input.txt').split('\n').reduce((recipes, recipeString) => {
    const [inputsString, outputString] = recipeString.split('=>');
    const inputs = inputsString.split(',').map(i => {
        const [inputQuantity, inputName] = i.trim().split(' ');
        return {
            name: inputName,
            quantity: inputQuantity | 0
        }
    })

    const [ outputQuantity, outputName ] = outputString.trim().split(' ');
    recipes[outputName] = recipe(outputName, outputQuantity, inputs);

    return recipes;
}, {});

const leftOverMaterials = {};
const results = {ORE: 0};
let oreBank = 1000000000000;

const getOreCount = (recipe, quantity) => {
    if (!(recipe.name in leftOverMaterials)) {
        leftOverMaterials[recipe.name] = 0;
    }

    while (leftOverMaterials[recipe.name] >= 1) {
        leftOverMaterials[recipe.name] -= 1;
        quantity -= 1;
    }

    const ingredientMultiplier = Math.ceil(quantity / recipe.count);
    const amount = recipe.count * ingredientMultiplier;
    
    if( quantity === 0) {
        return;
    }

    const leftOvers = amount - quantity;
    leftOverMaterials[recipe.name] += leftOvers;

    recipe.ingredients.forEach(ingredient => {
        const ingredientNeeds = ingredient.quantity * ingredientMultiplier;
        if(ingredient.name === 'ORE') {
            oreBank -= ingredientNeeds;
            if(oreBank < 0) {
                throw new Error('Not enough ore!')
            }
            results['ORE'] += ingredientNeeds;
        } else {
            getOreCount(recipes[ingredient.name], ingredientNeeds);
        }
    })
}


let fuelCount = 0;
while(true) {
    try {
        getOreCount(recipes.FUEL, 1);
        fuelCount++;
    } catch(e){
        break;
    }
}

console.log({
    fuelCount
})