from day01 import calculate_day_a_line, calculate_day_b_line, day_a, day_b


def test_calculate_day_a_line():
    assert calculate_day_a_line("12") == 12
    assert calculate_day_a_line("abc12def") == 12
    assert calculate_day_a_line("abc1def") == 11
    assert calculate_day_a_line("abc1def") == 11
    assert calculate_day_a_line("abc123def") == 13


def test_calculate_day_b_line():
    assert calculate_day_b_line("onetwo") == 12
    assert calculate_day_b_line("1two") == 12
    assert calculate_day_b_line("one2") == 12
    assert calculate_day_b_line("abconeightef") == 18
    assert calculate_day_b_line("onetwothree") == 13
    assert calculate_day_b_line("two") == 22


def test_day_a_test_data():
    assert day_a("data/test_data_a.txt") == 142


def test_day_b_test_data():
    assert day_b("data/test_data_b.txt") == 281
