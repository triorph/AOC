import {
  daya,
  parseRange,
  RangeConverter,
  parseData,
  Almanac,
  Map,
} from "./day05";
import { readFileSync } from "fs";

test("Test the sample data", () => {
  expect(daya("data/test_data.txt")).toBe(35);
});

test("Test that we can parse a range", () => {
  expect(parseRange("10 20 30")).toEqual(new RangeConverter(10, 20, 30));
});

test("Test that we can parse the full data map", () => {
  let data = readFileSync("data/test_data.txt").toString();
  expect(parseData(data)).toEqual(
    new Almanac(
      [79, 14, 55, 13],
      [
        new Map([
          new RangeConverter(50, 98, 2),
          new RangeConverter(52, 50, 48),
        ]),
        new Map([
          new RangeConverter(0, 15, 37),
          new RangeConverter(37, 52, 2),
          new RangeConverter(39, 0, 15),
        ]),
        new Map([
          new RangeConverter(49, 53, 8),
          new RangeConverter(0, 11, 42),
          new RangeConverter(42, 0, 7),
          new RangeConverter(57, 7, 4),
        ]),
        new Map([
          new RangeConverter(88, 18, 7),
          new RangeConverter(18, 25, 70),
        ]),
        new Map([
          new RangeConverter(45, 77, 23),
          new RangeConverter(81, 45, 19),
          new RangeConverter(68, 64, 13),
        ]),
        new Map([new RangeConverter(0, 69, 1), new RangeConverter(1, 0, 69)]),
        new Map([
          new RangeConverter(60, 56, 37),
          new RangeConverter(56, 93, 4),
        ]),
      ]
    )
  );
});
