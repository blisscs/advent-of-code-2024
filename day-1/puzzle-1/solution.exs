{list1, list2} =
  File.read!("input.txt")
  |> String.split("\n")
  |> Enum.reduce({[], []}, fn line, {list1, list2} ->
    Regex.named_captures(~r/(?<left>\d+)\s+(?<right>\d+)/, line)
    |> case do
      %{"left" => left, "right" => right} ->
        {[String.to_integer(left) | list1], [String.to_integer(right) | list2]}

      _ ->
        {list1, list2}
    end
  end)

# Sort the list
list1 = Enum.sort(list1)
list2 = Enum.sort(list2)

# Pair them
pairs = Enum.zip([list1, list2])

total_distance =
  Enum.reduce(pairs, 0, fn {left, right}, total_distance ->
    total_distance + abs(right - left)
  end)

IO.puts("total distance = #{total_distance}")
