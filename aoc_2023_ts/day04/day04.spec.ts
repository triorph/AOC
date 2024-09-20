import { Card, dayA, dayB, getCards } from "./day04";

test("Test we can parse the card data", () => {
  let cards = getCards("Card 12: 2 3 4 5 | 4 3 2");
  expect(cards.length).toBe(1);
  expect(cards[0]).toEqual(new Card(12, [2, 3, 4, 5], [4, 3, 2]));
});

test("Test overlap count", () => {
  let card = new Card(12, [2, 3, 4, 5], [4, 3, 8]);
  expect(card.countWinningNumbers()).toBe(2);
});

test("Test dayA score no match", () => {
  let card = new Card(12, [1, 2, 3, 4], [5, 6, 7, 8]);
  expect(card.getDayAValue()).toBe(0);
});

test("Test dayA score with matches", () => {
  let card = new Card(12, [1, 2, 3, 4], [3, 4, 5]);
  expect(card.getDayAValue()).toBe(2);
});

test("Test dayA test data gets right result", () => {
  expect(dayA("data/test_data.txt")).toBe(13);
});

test("Test dayB test data gets right result", () => {
  expect(dayB("data/test_data.txt")).toBe(30);
});
