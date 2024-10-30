from day04 import Card, day_a, day_b


def test_calculate_day_a_score():
    assert Card(12, [1, 2, 3], [4, 5, 6]).calculate_day_a_score() == 0
    assert Card(12, [1, 2, 3], [3, 4, 5]).calculate_day_a_score() == 1
    assert Card(12, [1, 2, 3], [2, 3, 4]).calculate_day_a_score() == 2
    assert Card(12, [1, 2, 3], [1, 2, 3]).calculate_day_a_score() == 4


def test_day_a():
    assert day_a("data/test_data.txt") == 13


def test_day_b():
    assert day_b("data/test_data.txt") == 30


def test_parse_cards():
    assert Card.from_line("Card 12: 2 3 4 | 4 5 6 7") == Card(
        12, [2, 3, 4], [4, 5, 6, 7]
    )
