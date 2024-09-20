import { parse } from "./parser";
import { readFileSync } from "fs";
class Card {
  id: number;
  have: number[];
  winning: number[];

  constructor(id: number, have: number[], winning: number[]) {
    this.id = id;
    this.have = have;
    this.winning = winning;
  }

  countWinningNumbers(): number {
    return this.have.filter((x) => this.winning.includes(x)).length;
  }

  getDayAValue(): number {
    let winningNumbers = this.countWinningNumbers();
    return winningNumbers == 0 ? 0 : 2 ** (winningNumbers - 1);
  }
}

function getCards(data: string): Card[] {
  return parse(data);
}

function dayA(filepath: string): number {
  let data = readFileSync(filepath).toString();
  let cards = getCards(data);
  return cards.map((card) => card.getDayAValue()).reduce((a, b) => a + b);
}
function dayB(filepath: string): number {
  let data = readFileSync(filepath).toString();
  let cards = getCards(data);
  let cardCounts: { [id: number]: number } = {};
  for (let card of cards) {
    cardCounts[card.id] = 1;
  }
  for (let i = 0; i < cards.length; i++) {
    for (let j = 0; j < cards[i].countWinningNumbers(); j++) {
      cardCounts[cards[i].id + j + 1] += cardCounts[cards[i].id];
    }
  }
  return Object.values(cardCounts).reduce((a, b) => a + b);
}

export { dayA, dayB, Card, getCards };
