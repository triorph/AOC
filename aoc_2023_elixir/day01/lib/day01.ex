defmodule Day01 do
  @moduledoc """
  Find the calibration value of each string, where its the first and last number are the 2 digits, and sum them.

  Day a: only numerical digits
  Day b: also words for numbers (e.g. "eight" => 8)
  """

  @doc """

  Gets the digit value of the first character, or nil if None

  ## Examples

      iex> Day01.get_digit_of_first_char("1est")
      1
      iex> Day01.get_digit_of_first_char("hello")
      nil

  """
  def get_digit_of_first_char(candidate) do
    ret = Integer.parse(String.slice(candidate, 0..0))

    case ret do
      {number, _} -> number
      :error -> nil
    end
  end

  @doc """
  Gets the digit value of the first word, or nil if None.

  ## Examples

    iex> Day01.get_number_from_start_word("eightars")
    8
    iex> Day01.get_number_from_start_word("hello")
    nil
  """

  def get_number_from_start_word(candidate) do
    numbers = %{one: 1, two: 2, three: 3, four: 4, five: 5, six: 6, seven: 7, eight: 8, nine: 9}

    {ret, _} =
      numbers
      |> Enum.map(fn {key, val} ->
        if String.starts_with?(candidate, Atom.to_string(key)) do
          val
        else
          nil
        end
      end)
      |> Enum.filter(fn x -> x != nil end)
      |> List.pop_at(0, nil)

    ret
  end

  @doc """
  Works out the calibration value for a line for part A
  """
  def day_a_calibration_value(input) do
    set_value = fn candidate, {first, last} ->
      candidate_number = get_digit_of_first_char(candidate)

      case {first, last, candidate_number} do
        {_, _, nil} -> {first, last}
        {nil, _, _} -> {candidate_number, candidate_number}
        _ -> {first, candidate_number}
      end
    end

    {first, last} =
      0..String.length(input)
      |> Enum.map(fn x -> String.slice(input, x..String.length(input)) end)
      |> Enum.reduce({nil, nil}, set_value)

    first * 10 + last
  end

  @doc """
  Works out if the string's start is either a number word or a digit, and return the appropriate integer (or nil if neither)
  """
  def day_b_digit(candidate) do
    case {get_digit_of_first_char(candidate), get_number_from_start_word(candidate)} do
      {nil, nil} -> nil
      {digit, nil} -> digit
      {_, value} -> value
    end
  end

  @doc """
  Works out the calibration value for a line in part b
  """
  def day_b_calibration_value(input) do
    set_value = fn candidate, {first, last} ->
      candidate_number = day_b_digit(candidate)

      case {first, last, candidate_number} do
        {_, _, nil} -> {first, last}
        {nil, _, _} -> {candidate_number, candidate_number}
        _ -> {first, candidate_number}
      end
    end

    {first, last} =
      0..String.length(input)
      |> Enum.map(fn x -> String.slice(input, x..String.length(input)) end)
      |> Enum.reduce({nil, nil}, set_value)

    first * 10 + last
  end

  @doc """
  Works out the sum of all calibration values for part A on the lines of data
  """
  def day_a_lines(lines) do
    lines
    |> Enum.filter(fn x -> String.trim(x) != "" end)
    |> Enum.map(fn x -> day_a_calibration_value(x) end)
    |> Enum.reduce(0, fn x, acc -> x + acc end)
  end

  @doc """
  Reads the file and does the part A calculation on its contents
  """
  def day_a(filepath) do
    case File.read(filepath) do
      {:ok, data} -> day_a_lines(String.split(data, "\n"))
      {:error, _} -> 0
    end
  end

  @doc """
  Works out the sum of all calibration values for part B on the lines of data
  """
  def day_b_lines(lines) do
    lines
    |> Enum.filter(fn x -> String.trim(x) != "" end)
    |> Enum.map(fn x -> day_b_calibration_value(x) end)
    |> Enum.reduce(0, fn x, acc -> x + acc end)
  end

  @doc """
  Reads the file and does the part B calculation on its contents
  """
  def day_b(filepath) do
    case File.read(filepath) do
      {:ok, data} -> day_b_lines(String.split(data, "\n"))
      {:error, _} -> 0
    end
  end
end
