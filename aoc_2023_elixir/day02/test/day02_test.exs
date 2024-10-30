defmodule Day02Test do
  use ExUnit.Case
  doctest Day02

  test "Test parsing the test_data" do
    {:ok, data} = File.read("data/test_data.txt")

    games =
      data
      |> String.split("\n")
      |> Enum.filter(fn x -> String.trim(x) != "" end)
      |> Day02.parse_data()

    expected = [
      %Day02.Game{
        id: 1,
        rounds: [
          %{"blue" => 3, "red" => 4},
          %{"red" => 1, "green" => 2, "blue" => 6},
          %{"green" => 2}
        ]
      },
      %Day02.Game{
        id: 2,
        rounds: [
          %{"blue" => 1, "green" => 2},
          %{"green" => 3, "blue" => 4, "red" => 1},
          %{"green" => 1, "blue" => 1}
        ]
      },
      %Day02.Game{
        id: 3,
        rounds: [
          %{"green" => 8, "blue" => 6, "red" => 20},
          %{"blue" => 5, "red" => 4, "green" => 13},
          %{"green" => 5, "red" => 1}
        ]
      },
      %Day02.Game{
        id: 4,
        rounds: [
          %{"green" => 1, "red" => 3, "blue" => 6},
          %{"green" => 3, "red" => 6},
          %{"green" => 3, "blue" => 15, "red" => 14}
        ]
      },
      %Day02.Game{
        id: 5,
        rounds: [
          %{"red" => 6, "blue" => 1, "green" => 3},
          %{"blue" => 2, "red" => 1, "green" => 2}
        ]
      }
    ]

    assert games == expected
  end

  test "Test part A result on test data" do
    assert Day02.day_a("data/test_data.txt") == 8
  end

  test "Test part A result on real data" do
    assert Day02.day_a("data/input_data.txt") == 2256
  end

  test "Test part B result on test data" do
    assert Day02.day_b("data/test_data.txt") == 2286
  end

  test "Test part B result on real data" do
    assert Day02.day_b("data/input_data.txt") == 74229
  end
end
