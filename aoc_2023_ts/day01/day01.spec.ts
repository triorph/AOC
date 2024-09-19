import {
  findCalibrationValueDayA,
  dayA,
  dayB,
  findCalibrationValueDayB,
} from "./day01";

test("Reads a string with multiple digits correctly", () => {
  expect(findCalibrationValueDayA("712324")).toBe(74);
});

test("Reads a string with multiple digits and non-digits correctly", () => {
  expect(findCalibrationValueDayA("ab7ce12de324defj")).toBe(74);
});

test("Reads a string with a single digit and multiple non-digits correctly", () => {
  expect(findCalibrationValueDayA("abec5defs")).toBe(55);
});

test("The day a test result is correct", () => {
  expect(dayA("data/test_data_a.txt")).toBe(142);
});

test("Reads a string with two named digits at each end", () => {
  expect(findCalibrationValueDayB("two1nine")).toBe(29);
});

test("The day b test result is correct", () => {
  expect(dayB("data/test_data_b.txt")).toBe(281);
});
