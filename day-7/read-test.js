const Readable = require('stream').Readable;

var s = new Readable();
s.push('beep');

s.on('data', data => {
    console.log(data.toString());
})
s.on('end', () => {
    console.log('All done')
})
s.push('beep');
s.push(null);