queueMicrotask(() => {
  console.log("Microtask is executed");
});

console.log("Synchronous task is executed");
