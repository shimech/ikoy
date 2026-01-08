{
  /*
   * Test if ikoy can execute JavaScript code from a file.
   */
  console.log("Hello, this is a JavaScript file!");
}

{
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
}

{
  /*
   * Test promises
   */
  console.log("Promise: 1");
  Promise.resolve("2").then((value) => {
    console.log(`Promise: ${value}`);
  });
  console.log("Promise: 3");
}

{
  /*
   * Test async/await
   */
  async function asyncFunc() {
    console.log("Async: 1");
    await Promise.resolve("2").then((value) => {
      console.log(`Async: ${value}`);
    });
    console.log("Async: 3");
  }
  asyncFunc();
}

{
  /*
   * Test fs.readFile
   */
  const filePath = "./javascript/assets/fs.readFile.txt";
  fs.readFile(filePath, (_, data) => {
    console.log(`[${filePath}] ${data}`);
  });
  console.log(`reading ${filePath}...`);

  const notExistedFilePath = "./javascript/assets/fs.readFile.notExisted.txt";
  fs.readFile(notExistedFilePath, (err, _) => {
    console.error(err);
  });
  console.log(`reading ${notExistedFilePath}...`);
}

{
  /*
   * Test fsPromises.readFile
   */
  const filePath = "./javascript/assets/fsPromises.readFile.txt";
  fsPromises.readFile(filePath).then((data) => {
    console.log(`[${filePath}] ${data}`);
  });
  console.log(`reading ${filePath}...`);
}
