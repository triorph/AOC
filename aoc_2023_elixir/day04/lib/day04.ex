defmodule Day04 do
  @moduledoc """
  Documentation for `Day04`.
  """

  def get_numbers_as_map_set(number_str) do
    number_str
    |> String.split(" ")
    |> Enum.map(&Integer.parse/1)
    |> Enum.filter(fn x -> match?({_, _}, x) end)
    |> Enum.map(fn {x, _} -> x end)
    |> MapSet.new()
  end

  def parse_line(line) do
    [_, numbers | _] = String.split(line, ":")
    [left, right | _] = String.split(numbers, "|")
    {get_numbers_as_map_set(left), get_numbers_as_map_set(right)}
  end

  def day_a_score({%MapSet{} = left, %MapSet{} = right}) do
    case MapSet.size(MapSet.intersection(left, right)) do
      0 -> 0
      size -> Integer.pow(2, size - 1)
    end
  end

  def day_a_lines(lines) do
    lines
    |> Enum.filter(fn line -> String.trim(line) != "" end)
    |> Enum.map(&parse_line/1)
    |> Enum.map(&day_a_score/1)
    |> Enum.reduce(fn x, acc -> x + acc end)
  end

  def day_b_lines(lines) do
    cards =
      lines
      |> Enum.filter(fn line -> String.trim(line) != "" end)
      |> Enum.map(&parse_line/1)
      |> Enum.map(fn {left, right} -> MapSet.size(MapSet.intersection(left, right)) end)
      |> Enum.with_index(1)

    start_quantities =
      cards
      |> Enum.reduce(%{}, fn {_, index}, acc -> Map.put(acc, index, 1) end)

    cards
    |> Enum.reduce(start_quantities, fn {size, index}, acc ->
      1..size//1
      |> Enum.reduce(acc, fn x, acc2 ->
        Map.put(acc2, x + index, Map.get(acc2, x + index) + Map.get(acc2, index))
      end)
    end)
    |> Enum.reduce(0, fn {_, value}, acc -> value + acc end)
  end

  def day_a(filepath) do
    case File.read(filepath) do
      {:ok, data} -> day_a_lines(String.split(data, "\n"))
      _ -> 0
    end
  end

  def day_b(filepath) do
    case File.read(filepath) do
      {:ok, data} -> day_b_lines(String.split(data, "\n"))
      _ -> 0
    end
  end
end
