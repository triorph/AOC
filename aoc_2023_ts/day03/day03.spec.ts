import {
  dayA,
  dayB,
  NumberPosition,
  parseNumbersAndSymbols,
  Point2D,
  SymbolPosition,
} from "./day03";

test("Test data should give the right result for part A", () => {
  expect(dayA("data/test_data.txt")).toBe(4361);
});

test("Test data should give the right result for part B", () => {
  expect(dayB("data/test_data.txt")).toBe(467835);
});

test("Test parse numbers just 1", () => {
  let [numbers, symbols] = parseNumbersAndSymbols("123");
  expect(numbers.length).toBe(1);
  expect(symbols.length).toBe(0);
  expect(numbers[0]).toEqual(
    new NumberPosition(123, new Point2D(0, 0), new Point2D(3, 0)),
  );
});

test("Test parse symbols just 1", () => {
  let [numbers, symbols] = parseNumbersAndSymbols("+");
  expect(numbers.length).toBe(0);
  expect(symbols.length).toBe(1);
  expect(symbols[0]).toEqual(new SymbolPosition("+", new Point2D(0, 0)));
});

test("Test multiline numbers are split", () => {
  let [numbers, symbols] = parseNumbersAndSymbols("..12\n34..");
  expect(numbers.length).toBe(2);
  expect(symbols.length).toBe(0);
  expect(numbers[0]).toEqual(
    new NumberPosition(12, new Point2D(2, 0), new Point2D(4, 0)),
  );
  expect(numbers[1]).toEqual(
    new NumberPosition(34, new Point2D(0, 1), new Point2D(2, 1)),
  );
});

test("Test parse numbers and symbols multiple each", () => {
  let input = "..123..+..\n..*...2...\n....-....";
  let [numbers, symbols] = parseNumbersAndSymbols(input);
  expect(numbers.length).toBe(2);
  expect(symbols.length).toBe(3);
  expect(numbers[0]).toEqual(
    new NumberPosition(123, new Point2D(2, 0), new Point2D(5, 0)),
  );
  expect(numbers[1]).toEqual(
    new NumberPosition(2, new Point2D(6, 1), new Point2D(7, 1)),
  );
  expect(symbols[0]).toEqual(new SymbolPosition("+", new Point2D(7, 0)));
  expect(symbols[1]).toEqual(new SymbolPosition("*", new Point2D(2, 1)));
  expect(symbols[2]).toEqual(new SymbolPosition("-", new Point2D(4, 2)));
});

test("Test adjacency true", () => {
  let number = new NumberPosition(12, new Point2D(3, 4), new Point2D(5, 4));
  let symbol = new SymbolPosition("+", new Point2D(5, 5));
  expect(number.isAdjacent(symbol)).toBeTruthy();
});

test("Test adjacency false", () => {
  let number = new NumberPosition(12, new Point2D(3, 4), new Point2D(5, 4));
  let symbol = new SymbolPosition("+", new Point2D(6, 5));
  expect(number.isAdjacent(symbol)).toBeFalsy();
});
