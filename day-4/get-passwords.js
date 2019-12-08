const isIncreasing = num => {
    const numbers = num.toString().split('');
    for (let i = 0; i < numbers.length; i++) {
        const prevNum = numbers[i - 1] | 0;
        const num = numbers[i] | 0;

        if(prevNum && prevNum > num) {
            return false;
        }
    }
    return true;
}
const hasDouble = num => {
    const doubleMatches = num.toString().match(/(\d)\1+/g);
    if(doubleMatches) {
        return doubleMatches.some(m => m.length === 2)
    }
    return false;
}
const isSixDigits = num => num <= 999999;

const isValidPassword = password => {
    const checkers = [
        isSixDigits,
        isIncreasing,
        hasDouble
    ];

    return checkers.every(c => c(password))
}

const getPasswords = (from, to) => {
    const passwords = [];

    for(let password = from; password <= to; password++) {
        if(isValidPassword(password)) {
            passwords.push(password);
        }
    }
    return passwords;
}

module.exports = {
    passwords: getPasswords,
    isIncreasing,
    hasDouble,
    isValidPassword
};