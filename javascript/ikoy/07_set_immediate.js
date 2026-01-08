console.log("Start");

ikoy.superHeavyProcess(() => {
  console.log("Super Heavy Process!");
}, 1000);

setTimeout(() => {
  setImmediate(() => {
    console.log("setImmediate");
  });
}, 2000);
