console.log("Hello, this is a JavaScript file!");

setTimeout(() => {
  console.log("Hello, this is a setTimeout!");
}, 1000);
const timeoutId = setTimeout(() => {
  console.log("This will never be shown because this timer will be cleared.");
}, 2000);
clearTimeout(timeoutId);
setTimeout(() => {
  console.log("Hello, this is a setTimeout!");
}, 3000);

console.log(`1 + 2 = ${1 + 2}`);

("Done!");
