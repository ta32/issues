console.log("test");
// load iframe
var iframe = document.createElement('iframe');
// using remote src works
// iframe.src = 'https://connect.trezor.io/8/iframe.html';
iframe.src = browser.extension.getURL('html/iframe.html');
iframe.width = '0px';
iframe.height = '0px';
iframe.style.position = 'absolute';
iframe.style.display = 'none';
iframe.style.border = '0px';
iframe.style.width = '0px';
iframe.style.height = '0px';
iframe.id = 'trezorconnect';
document.body.appendChild(iframe);
console.log("src: " + iframe.src);