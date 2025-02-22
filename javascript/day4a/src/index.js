// @ts-check
import * as fs from "node:fs";

const input = fs.readFileSync("./src/input.txt", "utf-8");

const lines = input.trim().split("\n");

const input_arr = [];
for (const line of lines) {
  input_arr.push(line.trim().split(""));
}

const posibilities = [
  [0, 1],
  [0, -1],
  [1, 0],
  [-1, 0],
  [1, 1],
  [-1, -1],
  [1, -1],
  [-1, 1],
];

const word = "XMAS";
let count = 0;

for (const [row_idx, row] of input_arr.entries()) {
  for (const [col_idx, cell] of row.entries()) {
    if (cell === "X") {
      for (const dir of posibilities) {
        let found = true;
        for (const [i, expected_char] of word.split("").entries()) {
          const new_row = row_idx + i * dir[0];
          const new_col = col_idx + i * dir[1];
          if (
            new_row < 0 ||
            new_col < 0 ||
            new_col >= row.length ||
            new_row >= input_arr.length ||
            input_arr[new_row][new_col] !== expected_char
          ) {
            found = false;
            break;
          }

          // console.log(input_arr[new_row][new_col]);
        }
        if (found) {
          count += 1;
        }
      }
    }
  }
}

console.log(count);
