defmodule Day03Test do
  use ExUnit.Case
  doctest Day03

  test "Test grid object parseing basic number" do
    actual = Day03.parse_lines(["...123.."])

    expected = [
      %Day03.GridObject{
        value: %Day03.Number{number: 123},
        location: %Day03.Location{
          start: %Day03.Point{x: 3, y: 0},
          end: %Day03.Point{x: 5, y: 0}
        }
      }
    ]

    assert actual == expected
  end

  test "Test grid object symbol" do
    actual = Day03.parse_lines(["..#."])

    expected = [
      %Day03.GridObject{
        value: %Day03.Symbol{symbol: "#"},
        location: %Day03.Location{
          start: %Day03.Point{x: 2, y: 0},
          end: %Day03.Point{x: 2, y: 0}
        }
      }
    ]

    assert actual == expected
  end

  test "Test multiline number" do
    actual = Day03.parse_lines(["...12", "34..."])

    expected = [
      %Day03.GridObject{
        value: %Day03.Number{number: 12},
        location: %Day03.Location{start: %Day03.Point{x: 3, y: 0}, end: %Day03.Point{x: 4, y: 0}}
      },
      %Day03.GridObject{
        value: %Day03.Number{number: 34},
        location: %Day03.Location{start: %Day03.Point{x: 0, y: 1}, end: %Day03.Point{x: 1, y: 1}}
      }
    ]

    assert actual == expected
  end

  test "Test the part A calculation on the test data" do
    actual = Day03.day_a("data/test_data.txt")
    expected = 4361
    assert actual == expected
  end

  test "Test the part A calculation on the real data" do
    actual = Day03.day_a("data/input_data.txt")
    expected = 540_025
    assert actual == expected
  end

  test "Test the part B calculation on the test data" do
    actual = Day03.day_b("data/test_data.txt")
    expected = 467_835
    assert actual == expected
  end

  test "Test the part B calculation on the real data" do
    actual = Day03.day_b("data/input_data.txt")
    expected = 84_584_891
    assert actual == expected
  end
end
