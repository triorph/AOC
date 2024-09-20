import { readFileSync } from "fs";

class Point2D {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}

class NumberPosition {
  value: number;
  start: Point2D;
  end: Point2D;

  constructor(value: number, start: Point2D, end: Point2D) {
    this.value = value;
    this.start = start;
    this.end = end;
  }

  isAdjacent(symbol: SymbolPosition): boolean {
    return (
      symbol.position.x >= this.start.x - 1 &&
      symbol.position.x < this.end.x + 1 &&
      symbol.position.y >= this.start.y - 1 &&
      symbol.position.y <= this.end.y + 1
    );
  }
}
class SymbolPosition {
  symbol: string;
  position: Point2D;

  constructor(symbol: string, position: Point2D) {
    this.symbol = symbol;
    this.position = position;
  }
}

function isDigit(c: string): boolean {
  return c in ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
}

function parseNumbersAndSymbols(
  data: string,
): [NumberPosition[], SymbolPosition[]] {
  let lines: string[] = data.split("\n");
  let numberPositions: NumberPosition[] = [];
  let symbolPositions: SymbolPosition[] = [];
  let currentNumber: number = 0;
  let numberStart: Point2D | null = null;
  for (let y = 0; y < lines.length; y++) {
    for (let x = 0; x < lines[y].length; x++) {
      if (isDigit(lines[y][x])) {
        if (numberStart == null) {
          numberStart = new Point2D(x, y);
        }
        currentNumber = currentNumber * 10 + parseInt(lines[y][x]);
      } else {
        if (currentNumber != 0 && numberStart != null) {
          numberPositions.push(
            new NumberPosition(currentNumber, numberStart, new Point2D(x, y)),
          );
          currentNumber = 0;
          numberStart = null;
        }

        if (lines[y][x] != ".") {
          symbolPositions.push(
            new SymbolPosition(lines[y][x], new Point2D(x, y)),
          );
        }
      }
    }
    if (currentNumber != 0 && numberStart != null) {
      numberPositions.push(
        new NumberPosition(
          currentNumber,
          numberStart,
          new Point2D(lines[y].length, y),
        ),
      );
      currentNumber = 0;
      numberStart = null;
    }
  }

  return [numberPositions, symbolPositions];
}

function dayA(filepath: string): number {
  let data = readFileSync(filepath);
  let [numberPositions, symbolPositions] = parseNumbersAndSymbols(
    data.toString(),
  );
  return numberPositions
    .filter((numberPosition) =>
      symbolPositions.some((symbol) => numberPosition.isAdjacent(symbol)),
    )
    .map((numberPosition) => numberPosition.value)
    .reduce((a, b) => a + b);
}

function dayB(filepath: string): number {
  let data = readFileSync(filepath);
  let [numberPositions, symbolPositions] = parseNumbersAndSymbols(
    data.toString(),
  );
  return symbolPositions
    .filter((symbolPosition) => symbolPosition.symbol == "*")
    .map((symbolPosition) =>
      numberPositions.filter((numberPosition) =>
        numberPosition.isAdjacent(symbolPosition),
      ),
    )
    .filter((numberPositions) => numberPositions.length == 2)
    .map((numberPositions) =>
      numberPositions
        .map((numberPosition) => numberPosition.value)
        .reduce((a, b) => a * b),
    )
    .reduce((a, b) => a + b);
}

export {
  dayA,
  dayB,
  parseNumbersAndSymbols,
  Point2D,
  NumberPosition,
  SymbolPosition,
};
