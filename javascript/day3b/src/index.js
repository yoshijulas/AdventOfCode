// @ts-check
import * as fs from "node:fs";

const input = fs.readFileSync("./src/input.txt", "utf-8");
const re = /don't\(\)|do\(\)|mul\(\d{1,3},\d{1,3}\)/g;
const re_separate = /\d{1,3}/g;

const matches = input.match(re) || [];

let enabled = true;
let sum = 0;

for (const detection of matches) {
  switch (detection) {
    case "do()":
      enabled = true;
      break;
    case "don't()":
      enabled = false;
      break;
    default:
      if (enabled) {
        const num = detection.match(re_separate) || [];
        if (num.length === 2) {
          sum += Number(num[0]) * Number(num[1]);
        }
      }
  }
}

console.log(sum);
