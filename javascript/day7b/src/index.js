// @ts-check
import * as fs from "node:fs";

const input = fs.readFileSync("./src/input.txt", "utf-8");

const lines = input
  .trim()
  .split("\n")
  .map((x) => {
    const part = x.split(":");
    const key = Number(part[0]);
    const value = part[1]
      .trim()
      .split(" ")
      .map((num) => Number(num));
    return [key, value];
  });

let sum = 0;

for (const num of lines) {
  if (Array.isArray(num[1]) && !Array.isArray(num[0])) {
    const numbers = calculate_number(num[1][0], num[1].slice(1));
    if (numbers.includes(num[0])) {
      sum += num[0];
    }
  }
}

console.log(sum);

/**
 * Recursively calculates all possible results by adding or multiplying
 * the current number with elements from the remaining array.
 *
 * @param {number} curr - The current value to be used in calculations.
 * @param {number[]} rest - The remaining array of numbers to process.
 * @returns {number[]} An array of results from all recursive calculations.
 */
function calculate_number(curr, rest) {
  if (rest.length === 0) {
    return [curr];
  }

  const head = rest[0];
  const tail = rest.slice(1);
  const result = [];

  result.push(...calculate_number(curr + head, tail));
  result.push(...calculate_number(curr * head, tail));
  result.push(...calculate_number(Number(curr.toString() + head), tail));
  return result;
}
