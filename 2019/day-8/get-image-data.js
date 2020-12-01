const chunk = (array, size) => {
    const chunked_arr = [];
    let index = 0;
    while (index < array.length) {
        chunked_arr.push(array.slice(index, size + index));
        index += size;
    }
    return chunked_arr;
}

const getImageChunks = (input, width, height) => {
    const chunkSize = width * height;
    const inputArray = input.split('');
    return chunk(inputArray, chunkSize)
}

const makeAsLayers = (width, height) => layerArray => chunk(layerArray, width)
const getImageData = (input, width, height) => getImageChunks(input, width, height).map(makeAsLayers(width, height))

module.exports = {
    getImageData,
    getImageChunks
}