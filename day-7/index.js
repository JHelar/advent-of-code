const { spawn } = require('child_process');
const Readable = require('stream').Readable;
const getPermutations = require('./get-permutations');

const spawnProgram = (phaseSetting, inputStream) => {
    const program = spawn('node', ['./day-7/child_process.js', phaseSetting], {
        cwd: process.cwd()
    });
    const outputStream = new Readable();
    outputStream._read = () => {}; // redundant? see update below

    const test = new Readable();
    test._read = () => {}; // redundant? see update below

    test.pipe(program.stdin)

    inputStream.on('data', data => {
        const sanitized = data.toString().replace('\n', '');
        console.log(`${phaseSetting} got data: ${sanitized}`);
        console.log(`${phaseSetting} writes ${sanitized}`);
        // test.push(sanitized + '\n');
        // test.emit('readable', sanitized)
    })

    program.on('exit', code => {
        // process.stdout.write(`Exit with: ${code}\n`)
        console.log(`${phaseSetting} outputs ${code.toString()} and exits.`);
        outputStream.push(code.toString());
        outputStream.push(null);
    })

    program.stderr.on('data', (data) => {

    });

    program.stdout.on('data', (data) => {
        console.log(`${phaseSetting} stdout: ${data}`);
        if(data.indexOf('Input') > -1){
        } else {
            console.log(`${phaseSetting} outputs ${data}`);
            outputStream.push(data);
        }
    });

    return outputStream;
}

const getOutputPromise = phaseSettings => new Promise(res => {
    let currentOutput = 0;

    const firstInputStream = new Readable();
    firstInputStream._read = () => {}; // redundant? see update below

    const lastOutput = phaseSettings.reduce((prevOutput, phaseSetting) => spawnProgram(phaseSetting, prevOutput), firstInputStream);

    lastOutput.on('data', data => {
        const sanitizedData = data.toString().replace('\n', '');
        currentOutput = sanitizedData;
        console.log({
            currentOutput
        })
        firstInputStream.push(currentOutput);
    })

    lastOutput.on('end', () => {
        res(currentOutput);
    })

    setTimeout(() => {
        // Start the mayhem
        firstInputStream.push(currentOutput.toString());
    }, 1000);
})

const run = async () => {
    const permutations = getPermutations([5,6,7,8,9]);
    let bestPhaseSettings;
    let bestOutput = 0;

    for (let i = 0; i < 1; i++) {
        const output = await getOutputPromise(permutations[i]);
        if(output > bestOutput) {
            bestPhaseSettings = permutations[i];
            bestOutput = output;
            // console.log('New best: ' + bestOutput)
        }
    }
    // const outputs = await Promise.all(outputPromises);
    console.log({
        bestOutput,
        bestPhaseSettings
    })
}

run();