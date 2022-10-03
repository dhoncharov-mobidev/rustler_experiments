defmodule RustlerExperimentTest do
  use ExUnit.Case
  doctest RustlerExperiment

  test "greets the world" do
    assert RustlerExperiment.hello() == :world
  end
end
