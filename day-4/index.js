const getPasswords = require('./get-passwords');

const passwords = getPasswords.passwords(152085, 670283);

const isValid = getPasswords.isValidPassword(112233)

console.log({
    isValid,
    len: passwords.length
})