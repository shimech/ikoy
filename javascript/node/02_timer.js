let timeoutId1 = null;
timeoutId1 = setTimeout(() => {
  console.log(`This is a setTimeout callback of ${timeoutId1}.`);
}, 500);
if (timeoutId1) {
  console.log(
    `Timer [ID: ${timeoutId1}] created by setTimeout has been created.`
  );
}
const timeoutId2 = setTimeout(() => {
  console.log("This will never be shown because this timer will be cleared.");
}, 500);
clearTimeout(timeoutId2);

let count = 0;
const intervalId = setInterval(() => {
  count++;
  console.log(
    `setInterval callback of ${intervalId} has been called ${count} times.`
  );
}, 1000);
setTimeout(() => {
  clearInterval(intervalId);
  console.log(
    `Timer [ID: ${intervalId}] created by setInterval has been cleared.`
  );
}, 5000);
console.log(
  `Timer [ID: ${intervalId}] created by setInterval has been created.`
);
