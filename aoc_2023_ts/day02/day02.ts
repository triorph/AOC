import { parse } from "./parser";
import { readFileSync } from "fs";

function dayA(filePath: string): number {
  let data: string = readFileSync(filePath).toString();
  let games = parseData(data);
  return games
    .filter((game) => game.isDayACompatible())
    .map((game) => game.id)
    .reduce((a, b) => a + b);
}

function dayB(filePath: string): number {
  let data: string = readFileSync(filePath).toString();
  let games = parseData(data);
  return games.map((game) => game.dayBValue()).reduce((a, b) => a + b);
}

class Round {
  entries: [number, string][];

  constructor(entries: [number, string][]) {
    this.entries = entries;
  }

  isDayACompatible(): boolean {
    for (let [quantity, colour] of this.entries) {
      if (colour == "red" && quantity > 12) {
        return false;
      }
      if (colour == "green" && quantity > 13) {
        return false;
      }
      if (colour == "blue" && quantity > 14) {
        return false;
      }
    }
    return true;
  }
}

class Game {
  id: number;
  rounds: Round[];

  constructor(id: number, rounds: Round[]) {
    this.id = id;
    this.rounds = rounds;
  }

  isDayACompatible(): boolean {
    return this.rounds.every((round) => round.isDayACompatible());
  }

  dayBValue(): number {
    let maxDict: { [id: string]: number } = {};
    this.rounds.forEach((round) => {
      for (let [quantity, colour] of round.entries) {
        if (!(colour in maxDict) || maxDict[colour] < quantity) {
          maxDict[colour] = quantity;
        }
      }
    });
    return Object.values(maxDict).reduce((a, b) => a * b);
  }
}

function parseData(data: string): Game[] {
  const sampleOutput = parse(data);
  return sampleOutput;
}

export { dayA, dayB, Game, Round };
