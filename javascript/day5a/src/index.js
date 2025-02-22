// @ts-check
import * as fs from "node:fs";

const input = fs.readFileSync("./src/input.txt", "utf-8");

const lines = input.trim().split("\n");

const input_arr = [];
for (const line of lines) {
  input_arr.push(line.trim().split(""));
}
