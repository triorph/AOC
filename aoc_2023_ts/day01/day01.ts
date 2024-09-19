import { readFileSync } from "fs";

function findCalibrationValueDayA(line: string): number {
  let first: null | string = null;
  let last: null | string = null;
  for (let i = 0; i < line.length; i++) {
    if (line[i] in ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]) {
      if (first == null) {
        first = line[i];
      }
      last = line[i];
    }
  }
  if (first == null || last == null) {
    return 0;
  }
  return parseInt(first + last);
}

const mappings = {
  one: 1,
  two: 2,
  three: 3,
  four: 4,
  five: 5,
  six: 6,
  seven: 7,
  eight: 8,
  nine: 9,
};
function dayBDigit(line: string): number | null {
  for (let [key, value] of Object.entries(mappings)) {
    if (line.startsWith(key)) {
      return value;
    } else if (line.startsWith(value.toString())) {
      return value;
    }
  }
  return null;
}

function findCalibrationValueDayB(line: string): number {
  let first: null | number = null;
  let last: null | number = null;
  for (let i = 0; i < line.length; i++) {
    let value = dayBDigit(line.slice(i));
    if (value != null) {
      if (first == null) {
        first = value;
      }
      last = value;
    }
  }
  return (first ?? 0) * 10 + (last ?? 0);
}

function dayA(filePath: string): number {
  let data = readFileSync(filePath, "utf8");
  return data
    .split("\n")
    .map(findCalibrationValueDayA)
    .reduce((a, b) => a + b);
}

function dayB(filePath: string): number {
  let data = readFileSync(filePath, "utf8");
  return data
    .split("\n")
    .map(findCalibrationValueDayB)
    .reduce((a, b) => a + b);
}

export { findCalibrationValueDayA, findCalibrationValueDayB, dayA, dayB };
