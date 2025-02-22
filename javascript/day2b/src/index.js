// @ts-check
import * as fs from "node:fs";

const input = fs.readFileSync("./src/input.txt", "utf-8");

const lines = input.trim().split("\n");

const input_arr = [];
for (const line of lines) {
  input_arr.push(line.trim().split(/\s+/).map(Number));
}

const safe_reports = input_arr.filter((lined) => {
  if (isValidLine(lined)) {
    return true;
  }
  return lined.some((_, idx) => {
    const modified = [...lined];
    modified.splice(idx, 1);
    return isValidLine(modified);
  });
}).length;

console.log("reports: ", safe_reports);

/**
 * Checks if a line of numbers is valid based on specific rules.
 *
 * @param {number[]} lined - The array of numbers to check.
 * @returns {boolean} - Returns true if the line is valid, otherwise false.
 */
function isValidLine(lined) {
  let trend = 0;
  for (let i = 0; i < lined.length - 1; i++) {
    const pair = [lined[i], lined[i + 1]];

    const diff = Math.abs(pair[0] - pair[1]);

    if (diff > 3 || diff < 1) {
      return false;
    }

    if (pair[0] < pair[1]) {
      if (trend === 0) {
        trend = 1;
      }
      if (trend !== 1) {
        return false;
      }
    } else {
      if (trend === 0) {
        trend = 2;
      }
      if (trend !== 2) {
        return false;
      }
    }
  }
  return true;
}
