const {sum, sum2} = require('./sum');

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

test('sum adds 1 + 2 to equal 5 should fail', () => {
  sum(1, 2).then((result) => {
    expect(result).toBe(5);
  });
});

test('sum 2 adds 1 + 2 to equal 5 should fail', () => {
  expect(sum2(1, 2)).toBe(5);
});

