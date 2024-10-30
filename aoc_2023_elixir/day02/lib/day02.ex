defmodule Day02 do
  @moduledoc """
  Do some set math on bags of numbers
  """

  defmodule Game do
    defstruct [:id, :rounds]
  end

  @doc """
  Run the part B calculation on the line data

  Find the power of the smallest possible bag that would allow all the rounds
  """
  def day_b_lines(lines) do
    lines
    |> Enum.filter(fn x -> String.trim(x) != "" end)
    |> parse_data
    |> Enum.map(&get_smallest_bag/1)
    |> Enum.map(&get_round_power/1)
    |> Enum.reduce(0, fn x, acc -> x + acc end)
  end

  @doc """
  Run the part B calculation on the contents of the file
  """
  def day_b(filepath) do
    case File.read(filepath) do
      {:ok, data} -> day_b_lines(String.split(data, "\n"))
      {:error, _} -> 0
    end
  end

  @doc """
  Find the smallest bag that covers both rounds,

  This is essentially the max of the 2 rounds values for each of its colours.
  """
  def smallest_coverage(round1, round2) do
    for x <- MapSet.union(MapSet.new(Map.keys(round1)), MapSet.new(Map.keys(round2))),
        into: %{},
        do: {x, max(Map.get(round1, x, 0), Map.get(round2, x, 0))}
  end

  @doc """
  Find the smallest bag that covers all of a games rounds
  """
  def get_smallest_bag(game) do
    game.rounds
    |> Enum.reduce(&smallest_coverage/2)
  end

  @doc """
  Get the power value of a round, which is the product of all of its quantities
  """
  def get_round_power(round) do
    round |> Map.values() |> Enum.reduce(1, fn x, acc -> x * acc end)
  end

  @doc """
  Run the part A calculation on the lines given.

  Check if all of a games rounds would be possible for 12 red balls, 13 green and 14 blue, then sum the IDs of possible games
  """
  def day_a_lines(lines) do
    lines
    |> Enum.filter(fn x -> String.trim(x) != "" end)
    |> parse_data
    |> Enum.filter(&game_is_possible_part_a?/1)
    |> Enum.map(fn game -> game.id end)
    |> Enum.reduce(0, fn x, acc -> x + acc end)
  end

  @doc """
  Run the part A calculation on the contents of the given file
  """
  def day_a(filepath) do
    case File.read(filepath) do
      {:ok, data} -> day_a_lines(String.split(data, "\n"))
      {:error, _} -> 0
    end
  end

  @doc """
  Checks if a round is possible for the part A check
  """
  def round_is_possible_part_a?(round) do
    %{"red" => 12, "green" => 13, "blue" => 14}
    |> Enum.all?(fn {key, val} -> Map.get(round, key, 0) <= val end)
  end

  def game_is_possible_part_a?(game) do
    game.rounds
    |> Enum.all?(&round_is_possible_part_a?/1)
  end

  @doc """
  Parses a string representing a round into its Round object (a dictionary of colours to quantity)
  """
  def parse_round(round_str) do
    round_str
    |> String.trim()
    |> String.split(",")
    |> Enum.map(fn x -> String.trim(x) end)
    |> Enum.reduce(%{}, fn x, acc ->
      [quantity_str, colour | _] = String.split(x, " ")
      {quantity, _} = Integer.parse(quantity_str)

      Map.put(acc, colour, quantity)
    end)
  end

  @doc """
  Parses a line representing a game into its Game object (id + list of rounds)
  """
  def parse_line(line) do
    [game_str, rounds | _] = String.split(line, ":")
    [_, id_str | _] = String.split(game_str, " ")
    rounds = rounds |> String.trim() |> String.split(";") |> Enum.map(&parse_round/1)

    {id_val, _} = Integer.parse(id_str)
    ret = %Game{id: id_val, rounds: rounds}
    ret
  end

  @doc """
  Parses a list of strings representing multiple games into a list of game objects
  """
  def parse_data(lines) do
    lines |> Enum.map(&parse_line/1)
  end
end
