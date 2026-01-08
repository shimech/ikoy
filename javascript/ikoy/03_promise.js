console.log("1");
Promise.resolve("2").then((value) => {
  console.log(`${value}`);
});
console.log("3");

/**
 * Expected:
 * 1
 * 3
 * 2
 */
