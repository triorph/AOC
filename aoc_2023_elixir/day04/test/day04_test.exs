defmodule Day04Test do
  use ExUnit.Case
  doctest Day04

  test "Parses a line correctly" do
    actual = Day04.parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
    expected = {MapSet.new([41, 48, 83, 86, 17]), MapSet.new([83, 86, 6, 31, 17, 9, 48, 53])}
    assert actual == expected
  end

  test "Test part A on the test data" do
    actual = Day04.day_a("data/test_data.txt")
    expected = 13
    assert actual == expected
  end

  test "Test part A on the real data" do
    actual = Day04.day_a("data/input_data.txt")
    expected = 21213
    assert actual == expected
  end

  test "Test part B on the test data" do
    actual = Day04.day_b("data/test_data.txt")
    expected = 30
    assert actual == expected
  end

  test "Test part B on the real data" do
    actual = Day04.day_b("data/input_data.txt")
    expected = 8_549_735
    assert actual == expected
  end
end
