defmodule Aoc2023 do
  use Application
  def start(_type, args) do
    day = "01"
    solutions = %{
      "01" => Aoc2023.Day01,
    }
    output = load_input("01")
      |> solutions[day].p1
    clean_day_name = day |> String.to_integer
    IO.puts "Solution for day #{clean_day_name}:\n#{output}"
    {:ok, self()}
  end

  defp load_input(day) do
    {:ok, content} = File.read("inputs/#{day}/input.txt")
    content
  end

end
