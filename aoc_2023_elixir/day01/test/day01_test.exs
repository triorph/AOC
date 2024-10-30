defmodule Day01Test do
  use ExUnit.Case
  doctest Day01

  test "Tests the day a calibration check" do
    assert Day01.day_a_calibration_value("1") == 11
    assert Day01.day_a_calibration_value("12") == 12
    assert Day01.day_a_calibration_value("123") == 13
    assert Day01.day_a_calibration_value("ab1cd2ef3gh") == 13
    assert Day01.day_a_calibration_value("ab1gh") == 11
  end

  test "Test get_digit_of_first_char" do
    assert Day01.get_digit_of_first_char("1") == 1
    assert Day01.get_digit_of_first_char("abc1") == nil
    assert Day01.get_digit_of_first_char("321") == 3
  end

  test "Test the test_data for day a" do
    assert Day01.day_a("data/test_data_a.txt") == 142
  end

  test "Test the real data for day a" do
    assert Day01.day_a("data/input_data.txt") == 53386
  end

  test "Test getting numbers to words" do
    assert Day01.get_number_from_start_word("eight") == 8
    assert Day01.get_number_from_start_word("bob") == nil
    assert Day01.get_number_from_start_word("8") == nil
    assert Day01.get_number_from_start_word("two") == 2
  end

  test "Test day b calibration check" do
    assert Day01.day_b_calibration_value("1") == 11
    assert Day01.day_b_calibration_value("two") == 22
    assert Day01.day_b_calibration_value("eightwo") == 82
    assert Day01.day_b_calibration_value("abc34twoneightars") == 38
  end

  test "Test the test_data for day b" do
    assert Day01.day_b("data/test_data_b.txt") == 281
  end

  test "Test the real data for day b" do
    assert Day01.day_b("data/input_data.txt") == 53312
  end
end
