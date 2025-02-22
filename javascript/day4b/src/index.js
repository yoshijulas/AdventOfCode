// @ts-check
import * as fs from "node:fs";

const input = fs.readFileSync("./src/input.txt", "utf-8");

const lines = input.trim().split("\n");

const input_arr = [];
for (const line of lines) {
  input_arr.push(line.trim().split(""));
}

const DIRECTIONS = [
  [-1, -1],
  [-1, 1],
  [1, 1],
  [1, -1],
];

let count = 0;

for (const [row_idx, row] of input_arr.entries()) {
  for (const [col_idx, cell] of row.entries()) {
    if (cell === "A") {
      let found_letters = "";
      for (const dir of DIRECTIONS) {
        const new_row = row_idx + dir[0];
        const new_col = col_idx + dir[1];
        if (
          new_row < 0 ||
          new_col < 0 ||
          new_col >= row.length ||
          new_row >= input_arr.length
        ) {
          break;
        }

        found_letters += input_arr[new_row][new_col];
      }

      if (
        found_letters === "MMSS" ||
        found_letters === "SMMS" ||
        found_letters === "SSMM" ||
        found_letters === "MSSM"
      ) {
        count += 1;
      }
    }
  }
}

console.log(count);
