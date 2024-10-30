import { readFileSync } from "fs";
function daya(filename: string): number {
  let data = readFileSync(filename).toString();
  let almanac = parseData(data);
  return almanac.dayA();
}

function dayb(filename: string): number {
  let data = readFileSync(filename).toString();
  let almanac = parseData(data);
  return almanac.dayB();
}

class Range {
  start: number;
  end: number;
  length: number;
  constructor(start: number, length: number) {
    this.start = start;
    this.end = start + length;
    this.length = length;
  }
}

class RangeConverter {
  sourceStart: number;
  destStart: number;
  length: number;
  constructor(destStart: number, sourceStart: number, len: number) {
    this.sourceStart = sourceStart;
    this.destStart = destStart;
    this.length = len;
  }

  convertNumber(input: number): number | null {
    if (this.withinSource(input)) {
      return input - this.sourceStart + this.destStart;
    }
    return null;
  }

  withinSource(value: number): boolean {
    return value >= this.sourceStart && value < this.sourceStart + this.length;
  }

  convertRanges(input: Range): Range[] {
    if (this.withinSource(input.start) && this.withinSource(input.end)) {
      return [
        new Range(
          input.start - this.sourceStart + this.destStart,
          input.length
        ),
      ];
    } else if (this.withinSource(input.start)) {
      return [
        new Range(input.start - this.sourceStart + this.destStart),
        new Range(),
      ];
    }
    return [input];
  }
}

function parseRange(line: string): RangeConverter {
  let values = line.split(" ");
  if (values.length != 3) {
    throw new Error("Invalid input line");
  }
  return new RangeConverter(
    parseInt(values[0]),
    parseInt(values[1]),
    parseInt(values[2])
  );
}

class Almanac {
  seeds: number[];
  maps: Map[];
  constructor(seeds: number[], maps: Map[]) {
    this.seeds = seeds;
    this.maps = maps;
  }

  dayA(): number {
    return Math.min(...this.seeds.map((seed) => this.convertNumber(seed)));
  }

  dayB(): number {
    return 0;
  }

  convertNumber(input: number): number {
    let next = input;
    for (let map of this.maps) {
      next = map.convertNumber(next);
    }
    return next;
  }

  convertRanges(input: Range[]): Range[] {
    let next = input;
    for (let map of this.maps) {
      next = map.convertRanges(next);
    }
    return next;
  }
}

class Map {
  ranges: RangeConverter[];

  constructor(ranges: RangeConverter[]) {
    this.ranges = ranges;
  }

  convertNumber(input: number): number {
    for (let range of this.ranges) {
      let output = range.convertNumber(input);
      if (output != null) {
        return output;
      }
    }
    return input;
  }

  convertRanges(input: Range[]): Range[] {
    return [];
  }
}

function parseData(data: string): Almanac {
  let lines = data.split("\n");
  let seeds = lines[0]
    .slice(7)
    .split(" ")
    .map((x) => parseInt(x));
  let map = new Array<RangeConverter[]>();
  for (let line of lines.slice(1)) {
    if (line.endsWith("map:")) {
      map.push(new Array<RangeConverter>());
    } else if (line.trim() != "") {
      map[map.length - 1].push(parseRange(line));
    }
  }
  return new Almanac(
    seeds,
    map.map((mapData) => new Map(mapData))
  );
}

export { daya, parseRange, RangeConverter, parseData, Almanac, Map, dayb };
