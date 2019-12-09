const readIntcode = require('./read-intcode');

const intcode = require('../utils').readArray('./day-7/test.txt');

(async () => {
    try {
        const result = await readIntcode(intcode);
        process.exit(result)
    } catch (e) {
        // Deal with the fact the chain failed
    }
})();
