import { readFileSync } from "fs";
var isEqual = require("lodash.isequal");
var zip = require("lodash.zip");
type Hand = string;

enum HandRanks {
  HIGH_CARD = 0,
  ONE_PAIR = 1,
  TWO_PAIR = 2,
  THREE_OF_A_KIND = 3,
  FULL_HOUSE = 4,
  FOUR_OF_A_KIND = 5,
  FIVE_OF_A_KIND = 6,
}

function getDayBRank(hand: Hand): number {
  let handMap = new Map<string, number>();
  for (let c of [...hand]) {
    handMap.set(c, (handMap.get(c) ?? 0) + 1);
  }
  let jokerSize = handMap.get("J") ?? 0;
  handMap.delete("J");
  // Get the max key/value from the remaining entries
  let [maxKey, maxValue] = [...handMap.entries()].reduce(
    ([key, value]: [string, number], acc: [string, number]) => {
      if (acc[1] > value) {
        return acc;
      } else {
        return [key, value];
      }
    },
    ["J", 0]
  );
  handMap.set(maxKey, maxValue + jokerSize);
  let values = Array.from(handMap.values());
  return valuesToRank(values);
}

function valuesToRank(values: number[]): number {
  if (isEqual(values, [5])) {
    return HandRanks.FIVE_OF_A_KIND;
  } else if (isEqual(values.sort(), [1, 4])) {
    return HandRanks.FOUR_OF_A_KIND;
  } else if (isEqual(values.sort(), [2, 3])) {
    return HandRanks.FULL_HOUSE;
  } else if (isEqual(values.sort(), [1, 1, 3])) {
    return HandRanks.THREE_OF_A_KIND;
  } else if (isEqual(values.sort(), [1, 2, 2])) {
    return HandRanks.TWO_PAIR;
  } else if (isEqual(values.sort(), [1, 1, 1, 2])) {
    return HandRanks.ONE_PAIR;
  } else if (isEqual(values.sort(), [1, 1, 1, 1, 1])) {
    return HandRanks.HIGH_CARD;
  } else {
    throw Error("Invalid values to convert to a hand rank");
  }
}

function getDayARank(hand: Hand): number {
  let handMap = new Map<string, number>();
  for (let c of [...hand]) {
    handMap.set(c, (handMap.get(c) ?? 0) + 1);
  }
  let values = Array.from(handMap.values());
  return valuesToRank(values);
}

function parseLine(line: string): [Hand, number] {
  let [handStr, numberStr] = line.split(" ");
  let number = parseInt(numberStr);
  return [handStr, number];
}

function parseData(data: string): [Hand, number][] {
  return data.trim().split("\n").map(parseLine);
}

function compareCardsDayA(left: Hand, right: Hand): number {
  const mappings = {
    "2": 2,
    "3": 3,
    "4": 4,
    "5": 5,
    "6": 6,
    "7": 7,
    "8": 8,
    "9": 9,
    T: 10,
    J: 11,
    Q: 12,
    K: 13,
    A: 14,
  };
  return zip([...left], [...right])
    .map(([charLeft, charRight]: [string, string]) => {
      return Object(mappings)[charLeft] - Object(mappings)[charRight];
    })
    .reduce((value: number, thisValue: number) => {
      if (value != 0) {
        return value;
      } else {
        return thisValue;
      }
    }, 0);
}

function compareCardsDayB(left: Hand, right: Hand): number {
  const mappings = {
    J: 1,
    "2": 2,
    "3": 3,
    "4": 4,
    "5": 5,
    "6": 6,
    "7": 7,
    "8": 8,
    "9": 9,
    T: 10,
    Q: 12,
    K: 13,
    A: 14,
  };
  return zip([...left], [...right])
    .map(([charLeft, charRight]: [string, string]) => {
      return Object(mappings)[charLeft] - Object(mappings)[charRight];
    })
    .reduce((value: number, thisValue: number) => {
      if (value != 0) {
        return value;
      } else {
        return thisValue;
      }
    }, 0);
}

function dayASorter(left: Hand, right: Hand): number {
  let leftRank = getDayARank(left);
  let rightRank = getDayARank(right);
  if (leftRank == rightRank) {
    return compareCardsDayA(left, right);
  }
  return leftRank - rightRank;
}

function dayBSorter(left: Hand, right: Hand): number {
  let leftRank = getDayBRank(left);
  let rightRank = getDayBRank(right);
  if (leftRank == rightRank) {
    return compareCardsDayB(left, right);
  }
  return leftRank - rightRank;
}

function dayA(filePath: string): number {
  let data: string = readFileSync(filePath).toString();
  let games = parseData(data);
  return games
    .sort((l, r) => dayASorter(l[0], r[0]))
    .map(([_, value], index) => {
      return value * (index + 1);
    })
    .reduce((x: number, acc: number) => {
      return x + acc;
    });
}

function dayB(filePath: string): number {
  let data: string = readFileSync(filePath).toString();
  let games = parseData(data);
  return games
    .sort((l, r) => dayBSorter(l[0], r[0]))
    .map(([_, value], index) => {
      return value * (index + 1);
    })
    .reduce((x: number, acc: number) => {
      return x + acc;
    });
}

export {
  dayA,
  parseLine,
  HandRanks,
  getDayARank,
  dayASorter,
  getDayBRank,
  dayBSorter,
  dayB,
};
