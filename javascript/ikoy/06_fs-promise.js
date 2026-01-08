const filePath = "./javascript/assets/fsPromises.readFile.txt";
fsPromises.readFile(filePath).then((data) => {
  console.log(`[${filePath}] ${data}`);
});
console.log(`reading ${filePath}...`);
