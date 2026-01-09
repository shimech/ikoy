import http from "node:http";

let id = 1;

function sleep(id) {
  if (id % 2 === 0) {
    return 4000;
  } else {
    return 500;
  }
}

const server = http.createServer((_, res) => {
  console.log(`ID: ${id} is requested!`);

  const body = `${id}`;
  setTimeout(() => {
    res.writeHead(200, { "Content-Type": "text/plain" });
    res.end(body);
  }, sleep(id));

  id += 1;
});

server.listen(7878);
