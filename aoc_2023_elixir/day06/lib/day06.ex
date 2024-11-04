defmodule Day06 do
  @moduledoc """
  """

  def ways_to_beat(time, distance) when is_integer(time) and is_integer(distance) do
    # Best to use quadratic formula to solve this:
    # - (time - x) * (x) > - distance
    # x^2 - time * x + distance > 0
    # (- b +- sqrt(b^2 - 4ac)) / 2a > 0

    # common is the -b / 2a component
    common = time / 2
    # sides is the sqrt(b^2  -4ac) / 2a component, which is both + and -
    sides = :math.sqrt(Integer.pow(time, 2) - 4 * distance) / 2

    # since its a greater than check, any "exact" results need to be offset by 1
    upper =
      if floor(common + sides) == common + sides do
        floor(common + sides - 1)
      else
        floor(common + sides)
      end

    lower =
      if(ceil(common - sides) == common - sides) do
        ceil(common - sides + 1)
      else
        ceil(common - sides)
      end

    upper - lower + 1
  end

  def day_a_lines(lines) when is_list(lines) do
    races = parse_data_a(lines)

    races
    |> Enum.map(fn {time, distance} -> ways_to_beat(time, distance) end)
    |> Enum.reduce(1, fn x, acc -> x * acc end)
  end

  def day_a(filepath) when is_binary(filepath) do
    case File.read(filepath) do
      {:ok, data} -> day_a_lines(String.split(data, "\n"))
      _ -> 0
    end
  end

  def parse_data_a(lines) when is_list(lines) do
    [times, distances] =
      lines
      |> Enum.filter(fn line -> String.trim(line) != "" end)
      |> Enum.map(fn line ->
        Regex.scan(~r/\d+/, line)
        |> List.flatten()
        |> Enum.map(fn num_str ->
          case Integer.parse(num_str) do
            {number, _} -> number
            _ -> 0
          end
        end)
      end)

    Enum.zip(times, distances)
  end

  def day_b_lines(lines) when is_list(lines) do
    [time, distance] = parse_data_b(lines)
    ways_to_beat(time, distance)
  end

  def day_b(filepath) when is_binary(filepath) do
    case File.read(filepath) do
      {:ok, data} -> day_b_lines(String.split(data, "\n"))
      _ -> 0
    end
  end

  def parse_data_b(lines) when is_list(lines) do
    lines
    |> Enum.filter(fn line -> String.trim(line) != "" end)
    |> Enum.map(fn line ->
      Regex.scan(~r/\d+/, line)
      |> List.flatten()
      |> Enum.join("")
    end)
    |> Enum.map(fn num_str ->
      case Integer.parse(num_str) do
        {number, _} -> number
        _ -> 0
      end
    end)
  end
end
