import {
  dayA,
  dayASorter,
  dayB,
  dayBSorter,
  getDayARank,
  getDayBRank,
  HandRanks,
  parseLine,
} from "./day07";

it("Test parseing some lines", () => {
  expect(parseLine("32T3K 765")).toEqual(["32T3K", 765]);
});

it("Test rankings day A", () => {
  expect(getDayARank("32T3K")).toBe(HandRanks.ONE_PAIR);
  expect(getDayARank("T55J5")).toBe(HandRanks.THREE_OF_A_KIND);
  expect(getDayARank("KK677")).toBe(HandRanks.TWO_PAIR);
  expect(getDayARank("KTJJT")).toBe(HandRanks.TWO_PAIR);
  expect(getDayARank("QQQJA")).toBe(HandRanks.THREE_OF_A_KIND);
});

it("Test rankings day B", () => {
  expect(getDayBRank("32T3K")).toBe(HandRanks.ONE_PAIR);
  expect(getDayBRank("T55J5")).toBe(HandRanks.FOUR_OF_A_KIND);
  expect(getDayBRank("KK677")).toBe(HandRanks.TWO_PAIR);
  expect(getDayBRank("KTJJT")).toBe(HandRanks.FOUR_OF_A_KIND);
  expect(getDayBRank("QQQJA")).toBe(HandRanks.FOUR_OF_A_KIND);
  expect(getDayBRank("JJJJJ")).toBe(HandRanks.FIVE_OF_A_KIND);
});

it("Test sorting day A", () => {
  expect(["AAAJ8", "23456"].sort(dayASorter)).toEqual(["23456", "AAAJ8"]);
  expect(["23459", "23456"].sort(dayASorter)).toEqual(["23456", "23459"]);
  expect(
    ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"].sort(dayASorter)
  ).toEqual(["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"]);
});

it("Test sorting day B", () => {
  expect(
    ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"].sort(dayBSorter)
  ).toEqual(["32T3K", "KK677", "T55J5", "QQQJA", "KTJJT"]);
});

it("Test day A on the test data", () => {
  expect(dayA("data/test_data.txt")).toBe(6440);
});

it("Test day A on the real data", () => {
  expect(dayA("data/input_data.txt")).toBe(255048101);
});

it("Test day B on the test data", () => {
  expect(dayB("data/test_data.txt")).toBe(5905);
});

it("Test day B on the real data", () => {
  expect(dayB("data/input_data.txt")).toBe(253718286);
});
