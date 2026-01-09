let id = 1;

function sleep(id) {
  if (id % 2 === 0) {
    return 4000;
  } else {
    return 500;
  }
}

http.createServer((res) => {
  console.log(`ID: ${id} is requested!`);

  const body = `${id}`;
  setTimeout(() => {
    res(body);
  }, sleep(id));

  id += 1;
});
