defmodule Day06Test do
  use ExUnit.Case
  doctest Day06

  test "Test parsing the test data (part A)" do
    lines =
      case File.read("data/test_data.txt") do
        {:ok, data} -> String.split(data, "\n")
        _ -> []
      end

    actual = Day06.parse_data_a(lines)
    expected = [{7, 9}, {15, 40}, {30, 200}]
    assert actual == expected
  end

  test "Test parsing the test data (part B)" do
    lines =
      case File.read("data/test_data.txt") do
        {:ok, data} -> String.split(data, "\n")
        _ -> []
      end

    actual = Day06.parse_data_b(lines)
    expected = [71530, 940_200]
    assert actual == expected
  end

  test "Test the part A calculation on the test data" do
    assert Day06.day_a("data/test_data.txt") == 288
  end

  test "Test the part A calculation on the real data" do
    assert Day06.day_a("data/input_data.txt") == 138_915
  end

  test "Test ways_to_beat calculations" do
    assert Day06.ways_to_beat(7, 9) == 4
    assert Day06.ways_to_beat(15, 40) == 8
    assert Day06.ways_to_beat(30, 200) == 9
  end

  test "Test the part B calculation on the test data" do
    assert Day06.day_b("data/test_data.txt") == 71503
  end

  test "Test the part B calculation on the real data" do
    assert Day06.day_b("data/input_data.txt") == 27_340_847
  end
end
