defmodule RustlerExperiment do
  @moduledoc """
  Documentation for `RustlerExperiment`.
  """
  alias RustlerExperiment.Native

  @doc """
  Hello world.

  ## Examples

      iex> RustlerExperiment.hello()
      :world

  """
  def hello do
    :world
  end

  def add(a, b), do: Native.add(a, b)

  def mrt_parser(path, _opts \\ []) do
    Native.mrt_parser(path)
  end
end
