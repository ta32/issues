const sum = require('./sum');

//const { subtle } = require('node:crypto').webcrypto;
// window.crypto.subtle = subtle;

//const { subtle } = require('node:crypto').webcrypto;
// Object.defineProperty(window, 'crypto', {
//   get(){
//     return {
//       subtle: subtle,
//     }
//   }
// })

const crypto = require('node:crypto');
Object.defineProperty(global.self, "crypto", {
  value: {
    subtle: crypto.webcrypto.subtle,
  },
});

test('adds 1 + 2 to equal 5 should fail', () => {
  sum(1, 2).then((result) => {
    expect(result).toBe(5);
  });
});
