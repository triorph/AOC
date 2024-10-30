class Card:
    def __init__(self, id: int, have: list[int], winning: list[int]):
        self.id = id
        self.have = have
        self.winning = winning

    @staticmethod
    def from_line(line: str) -> "Card":
        card_and_id, numbers = line.split(":")
        id = int(card_and_id[5:])
        have_str, winning_str = numbers.split("|")
        have = [int(number) for number in have_str.strip().split(" ") if number != ""]
        winning = [
            int(number) for number in winning_str.strip().split(" ") if number != ""
        ]
        return Card(id, have, winning)

    def __eq__(self, other: object) -> bool:
        return (
            isinstance(other, Card)
            and self.id == other.id
            and self.have == other.have
            and self.winning == other.winning
        )

    def __repr__(self):
        return (
            f"Card {self.id}: "
            f"{' '.join([str(number) for number in self.have])} "
            f"| {' '.join([str(number) for number in self.winning])}"
        )

    def calculate_matching(self) -> int:
        return len(set(self.have).intersection(set(self.winning)))

    def calculate_day_a_score(self) -> int:
        matching: int = self.calculate_matching()
        return 0 if matching == 0 else 2 ** (matching - 1)


def parse_lines(lines: list[str]) -> list[Card]:
    return [Card.from_line(line) for line in lines if line != ""]


def day_a(filepath: str) -> int:
    with open(filepath, "r") as f:
        lines: list[str] = f.readlines()
    return sum(card.calculate_day_a_score() for card in parse_lines(lines))


def day_b(filepath: str) -> int:
    with open(filepath, "r") as f:
        lines: list[str] = f.readlines()
    ticket_counts: dict[int, int] = {}
    cards: list[Card] = parse_lines(lines)
    for card in cards:
        ticket_counts.setdefault(card.id, 1)
        for i in range(card.calculate_matching()):
            ticket_counts.setdefault(card.id + i + 1, 1)
            ticket_counts[card.id + i + 1] += ticket_counts[card.id]
    return sum(ticket_counts.values())


def main():
    print(f"Day04 part a result is {day_a('data/input_data.txt')}")
    print(f"Day04 part b result is {day_b('data/input_data.txt')}")


if __name__ == "__main__":
    main()
