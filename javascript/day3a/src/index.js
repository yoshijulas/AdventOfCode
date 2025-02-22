// @ts-check
import * as fs from "node:fs";

const input = fs.readFileSync("./src/input.txt", "utf-8");
const re = /mul\((\d{1,3}),(\d{1,3})\)/g;
const sum = [...input.matchAll(re)].reduce(
  (acc, capture) => acc + Number(capture[1]) * Number(capture[2]),
  0,
);

console.log(sum);
