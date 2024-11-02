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

  defmodule GridObjectListBuilder do
    defstruct current_value: nil, start_location: nil, grid_objects: [], previous_location: nil

    def get_digit_or_null(char) do
      ret = Integer.parse(char)

      case ret do
        {number, _} -> number
        :error -> nil
      end
    end

    def finish_number(%GridObjectListBuilder{} = state) do
      grid_objects =
        List.insert_at(state.grid_objects, -1, %GridObject{
          location: %Location{
            start: state.start_location,
            end: state.previous_location
          },
          value: %Number{number: state.current_value}
        })

      Map.merge(state, %{
        :grid_objects => grid_objects,
        :current_value => nil,
        :start_location => nil
      })
    end

    def maybe_finish_number(
          {%Point{} = this_location, _},
          %GridObjectListBuilder{} = state,
          char_num
        ) do
      if state.current_value != nil &&
           (char_num == nil || this_location.y != state.start_location.y) do
        finish_number(state)
      else
        state
      end
    end

    def create_number(
          {%Point{} = this_location, _},
          %GridObjectListBuilder{} = state,
          char_num
        ) do
      Map.merge(state, %{:current_value => char_num, :start_location => this_location})
    end

    def increase_number(%GridObjectListBuilder{} = state, char_num) do
      Map.merge(state, %{:current_value => state.current_value * 10 + char_num})
    end

    def increase_or_create_number(
          {%Point{}, _} = data_point,
          %GridObjectListBuilder{} = state,
          char_num
        ) do
      if state.current_value == nil do
        create_number(data_point, state, char_num)
      else
        increase_number(state, char_num)
      end
    end

    def maybe_add_symbol(
          {%Point{} = this_location, char},
          %GridObjectListBuilder{} = state
        ) do
      grid_objects =
        if char != "." do
          List.insert_at(state.grid_objects, -1, %GridObject{
            location: %Location{start: this_location, end: this_location},
            value: %Symbol{symbol: char}
          })
        else
          state.grid_objects
        end

      Map.put(state, :grid_objects, grid_objects)
    end

    def increase_number_or_add_symbol(
          {%Point{}, _} = data_point,
          %GridObjectListBuilder{} = state,
          char_num
        ) do
      if !is_nil(char_num) do
        increase_or_create_number(data_point, state, char_num)
      else
        maybe_add_symbol(data_point, state)
      end
    end

    def assemble_grid_objects(
          {%Point{} = this_location, char} = data_point,
          %GridObjectListBuilder{} = state
        ) do
      char_num = get_digit_or_null(char)
      state = maybe_finish_number(data_point, state, char_num)
      state = increase_number_or_add_symbol(data_point, state, char_num)

      Map.merge(state, %{:previous_location => this_location})
    end
  end

  def get_characters_and_locations(lines) do
    Enum.with_index(lines)
    |> Enum.map(fn {line, y} ->
      Enum.with_index(String.graphemes(line))
      |> Enum.map(fn {char, x} -> {%Point{x: x, y: y}, char} end)
    end)
    |> List.flatten()
  end

  def parse_lines(lines) do
    data_points = get_characters_and_locations(lines)

    %GridObjectListBuilder{} =
      state =
      Enum.reduce(
        data_points,
        %GridObjectListBuilder{},
        &GridObjectListBuilder.assemble_grid_objects/2
      )

    state.grid_objects
  end

  def is_adjacent?(
        %GridObject{location: %Location{start: num_start, end: num_end}, value: %Number{}},
        %GridObject{
          location: %Location{start: symbol_pos},
          value: %Symbol{}
        }
      ) do
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
