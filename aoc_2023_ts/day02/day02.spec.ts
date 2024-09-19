import { dayA, dayB } from "./day02";

test("Day A test data is correct", () => {
  expect(dayA("data/test_data.txt")).toBe(8);
});

test("Day B test data is correct", () => {
  expect(dayB("data/test_data.txt")).toBe(2286);
});
