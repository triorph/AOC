defmodule Day03 do
  @moduledoc """
  Take note of locations of numbers and compare them to symbols
  """

  defmodule Location do
    defstruct [:start, :end]
  end

  defmodule Symbol do
    defstruct [:symbol]
  end

  defmodule Number do
    defstruct [:number]
  end

  defmodule Point do
    defstruct [:x, :y]
  end

  defmodule GridObject do
    defstruct [:location, :value]
  end

  def get_digit_or_null(char) do
    ret = Integer.parse(char)

    case ret do
      {number, _} -> number
      :error -> nil
    end
  end

  def assemble_grid_objects(
        {%Point{} = this_location, char},
        %{
          :current_value => current_value,
          :start_location => start_location,
          :previous_location => previous_location,
          :grid_objects => grid_objects
        }
      ) do
    char_num = get_digit_or_null(char)

    {grid_objects, current_value, start_location} =
      if current_value != nil && (char_num == nil || this_location.y != start_location.y) do
        grid_objects =
          List.insert_at(grid_objects, -1, %GridObject{
            location: %Location{
              start: start_location,
              end: %Point{x: previous_location.x, y: previous_location.y}
            },
            value: %Number{number: current_value}
          })

        current_value = nil
        start_location = nil
        {grid_objects, current_value, start_location}
      else
        {grid_objects, current_value, start_location}
      end

    {grid_objects, current_value, start_location} =
      if !is_nil(char_num) do
        {current_value, start_location} =
          if current_value == nil do
            current_value = char_num
            start_location = this_location
            {current_value, start_location}
          else
            current_value = current_value * 10 + char_num
            {current_value, start_location}
          end

        {grid_objects, current_value, start_location}
      else
        grid_objects =
          if char != "." do
            List.insert_at(grid_objects, -1, %GridObject{
              location: %Location{start: this_location, end: this_location},
              value: %Symbol{symbol: char}
            })
          else
            grid_objects
          end

        {grid_objects, current_value, start_location}
      end

    %{
      :current_value => current_value,
      :start_location => start_location,
      :grid_objects => grid_objects,
      :previous_location => this_location
    }
  end

  def parse_lines(lines) do
    initial_state = %{
      :current_value => nil,
      :start_location => nil,
      :grid_objects => [],
      :previous_location => nil
    }

    data_points =
      Enum.with_index(lines)
      |> Enum.map(fn {line, y} ->
        Enum.with_index(String.graphemes(line))
        |> Enum.map(fn {char, x} -> {%Point{x: x, y: y}, char} end)
      end)
      |> List.flatten()

    %{:grid_objects => grid_objects} =
      Enum.reduce(data_points, initial_state, &assemble_grid_objects/2)

    grid_objects
  end

  def is_adjacent?(%GridObject{location: %Location{start: num_start, end: num_end}}, %GridObject{
        location: %Location{start: symbol_pos}
      }) do
    symbol_pos.x >= num_start.x - 1 && symbol_pos.x <= num_end.x + 1 &&
      symbol_pos.y >= num_start.y - 1 && symbol_pos.y <= num_start.y + 1
  end

  def day_a_lines(lines) do
    grid_objects = lines |> Enum.filter(fn x -> String.trim(x) != "" end) |> parse_lines()
    grid_symbols = Enum.filter(grid_objects, fn x -> match?(%{value: %Symbol{}}, x) end)
    grid_numbers = Enum.filter(grid_objects, fn x -> match?(%{value: %Number{}}, x) end)

    grid_numbers
    |> Enum.filter(fn grid_number ->
      Enum.any?(grid_symbols, fn symbol -> is_adjacent?(grid_number, symbol) end)
    end)
    |> Enum.map(fn grid_number -> grid_number.value.number end)
    |> Enum.reduce(0, fn x, acc -> x + acc end)
  end

  def day_a(filepath) do
    case File.read(filepath) do
      {:ok, data} -> day_a_lines(String.split(data, "\n"))
      {:error, _} -> 0
    end
  end

  def day_b_lines(lines) do
    grid_objects = lines |> Enum.filter(fn x -> String.trim(x) != "" end) |> parse_lines()
    grid_numbers = Enum.filter(grid_objects, fn x -> match?(%{value: %Number{}}, x) end)

    gear_candidates =
      Enum.filter(grid_objects, fn x -> match?(%{value: %Symbol{symbol: "*"}}, x) end)

    gear_candidates
    |> Enum.map(fn gear_candidate ->
      Enum.filter(grid_numbers, fn number_candidate ->
        is_adjacent?(number_candidate, gear_candidate)
      end)
    end)
    |> Enum.filter(fn adjacent_numbers -> Enum.count(adjacent_numbers) == 2 end)
    |> Enum.map(fn adjacent_numbers ->
      Enum.reduce(adjacent_numbers, 1, fn x, acc -> x.value.number * acc end)
    end)
    |> Enum.reduce(0, fn x, acc -> x + acc end)
  end

  def day_b(filepath) do
    case File.read(filepath) do
      {:ok, data} -> day_b_lines(String.split(data, "\n"))
      {:error, _} -> 0
    end
  end
end
