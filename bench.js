import { setTimeout } from "node:timers/promises";

while (true) {
  const requestedAt = Date.now();
  fetch("http://127.0.0.1:7878")
    .then((res) => res.text())
    .then((body) => {
      const responsedAt = Date.now();
      console.log(`ID: ${body}, Response time: ${responsedAt - requestedAt}ms`);
    })
    .catch((err) => console.error(err));
  await setTimeout(1000);
}
