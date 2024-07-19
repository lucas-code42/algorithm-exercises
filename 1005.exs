defmodule Media.Ex do
  def media() do
    value_a = IO.gets("")
    value_a = String.replace(value_a, "\n", "")
    value_a = String.to_float(value_a)

    value_b = IO.gets("")
    value_b = String.replace(value_b, "\n", "")
    value_b = String.to_float(value_b)

    result = :erlang.float_to_binary((value_a + value_b) / 2, decimals: 5)
    "MEDIA = #{result}"
  end
end

Media.Ex.media()
