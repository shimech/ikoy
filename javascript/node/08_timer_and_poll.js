// Example of https://nodejs.org/ja/learn/asynchronous-work/event-loop-timers-and-nexttick#phases-in-detail
import timers from "node:timers/promises";

function someAsyncOperation(callback) {
  // Assume this takes 950ms to complete
  timers.setTimeout(950).then(callback);
}

const timeoutScheduled = Date.now();

setTimeout(() => {
  const delay = Date.now() - timeoutScheduled;

  console.log(`${delay}ms have passed since I was scheduled`);
}, 1000);

// do someAsyncOperation which takes 950 ms to complete
someAsyncOperation(() => {
  const startCallback = Date.now();

  // do something that will take 100ms...
  while (Date.now() - startCallback < 100) {
    // do nothing
  }
});
