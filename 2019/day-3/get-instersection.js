const getSlope = (onePoint, anotherPoint) => (anotherPoint.y - onePoint.y) / (anotherPoint.x - onePoint.x);
const getB = (onePoint, anotherPoint) => {
    const slope = getSlope(onePoint, anotherPoint);
    return onePoint.y - slope * onePoint.x;
}

const isVertical = (onePoint, anotherPoint) => onePoint.x === anotherPoint.x && onePoint.y !== anotherPoint.y

// const getInstersection = (points1, points2) => {
//     let [ one1, one2 ] = points1;
//     let [ another1, another2 ] = points2;

//     if(one1 && one2 && another1 && another2) {
//         const oneSlope = getSlope(one1, one2);
//         const oneB = getB(one1, one2);
    
//         const anotherSlope = getSlope(another1, another2);
//         const anotherB = getB(another1, another2);
    
//         if(oneSlope === anotherSlope) return null;
    
//         const x = (oneB - anotherB) / (anotherSlope - oneSlope);
//         const y = (oneSlope * x) + oneB;
    
//         return {
//             x,
//             y
//         }
//     }
//     return null;
// }

const getInstersection = (points1, points2) => {
    let [ one1, one2 ] = points1;
    let [ another1, another2 ] = points2;

    if(one1 && one2 && another1 && another2) {
        const oneIsVertical = isVertical(one1, one2);
        const anotherIsVertical = isVertical(another1, another2);

        if(oneIsVertical === anotherIsVertical) return null; // If both is same direction.

        if(oneIsVertical) {
            if(
                ((one1.x < another1.x && one1.x > another2.x) || (one1.x > another1.x && one1.x < another2.x)) &&
                ((one1.y > another1.y && one2.y < another1.y) || (one1.y < another1.y && one2.y > another1.y))
            ) {
                return {
                    x: one1.x,
                    y: another1.y
                }
            }
        } else {
            if(
                ((another1.x < one1.x && another1.x > one2.x) || (another1.x > one1.x && another1.x < one2.x)) &&
                ((another1.y > one1.y && another2.y < one1.y) || (another1.y < one1.y && another2.y > one1.y))
            ) {
                return {
                    x: another1.x,
                    y: one1.y
                }
            }
        }
    }
    return null;
}

module.exports = getInstersection;