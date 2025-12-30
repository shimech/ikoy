/*
 * Test if ikoy can execute JavaScript code from a file.
 */
console.log("Hello, this is a JavaScript file!");

/*
 * Test timers
 */
let timeoutId1 = null;
timeoutId1 = setTimeout(() => {
  console.log(`This is a setTimeout callback of ${timeoutId1}.`);
}, 0);
if (timeoutId1) {
  console.log(
    `Timer [ID: ${timeoutId1}] created by setTimeout has been created.`
  );
}

const timeoutId2 = setTimeout(() => {
  console.log("This will never be shown because this timer will be cleared.");
}, 1000);
clearTimeout(timeoutId2);

let count = 0;
const intervalId = setInterval(() => {
  count++;
  console.log(`setInterval callback has been called ${count} times.`);
}, 500);
setTimeout(() => {
  clearInterval(intervalId);
}, 2000);

/*
 * Test promises
 */
console.log("Promise: 1");
Promise.resolve("2").then((value) => {
  console.log(`Promise: ${value}`);
});
console.log("Promise: 3");
