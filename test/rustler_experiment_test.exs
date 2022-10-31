defmodule RustlerExperimentTest do
  use ExUnit.Case
  doctest RustlerExperiment

  test "greets the world" do
    assert RustlerExperiment.hello() == :world
  end

  test "adds two numbers" do
    assert RustlerExperiment.add(1, 2) == 3
  end

  test "sample parser" do
    result =
      RustlerExperiment.mrt_parser(
        "http://archive.routeviews.org/bgpdata/2021.10/UPDATES/updates.20211001.0000.bz2"
      )

    IO.inspect(result, label: "result")
  end

  test "sample list" do
    result = RustlerExperiment.Native.sample_list()
    IO.inspect(result)
  end
end
