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
