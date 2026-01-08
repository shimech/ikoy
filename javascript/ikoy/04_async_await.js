async function asyncFunc() {
  console.log("1");
  await Promise.resolve("2").then((value) => {
    console.log(`${value}`);
  });
  console.log("3");
}
asyncFunc();

/**
 * Expected:
 * 1
 * 2
 * 3
 */
