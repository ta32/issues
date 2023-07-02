
function sum2(a, b) {
  return a + b;
}

async function sum(a, b) {
  let passKey = await crypto.subtle.generateKey(
    {
      name: 'AES-GCM',
      length: 256,
    },
    true,
    ['encrypt', 'decrypt']
  );
  return a + b;
}

module.exports = {sum, sum2}
