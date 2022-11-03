defmodule RustlerExperimentTest do
  use ExUnit.Case
  doctest RustlerExperiment

  test "greets the world" do
    assert RustlerExperiment.hello() == :world
  end

  test "adds two numbers" do
    assert RustlerExperiment.add(1, 2) == 3
  end

  describe "mrt_parser can" do
    setup do
      [
        path: "http://archive.routeviews.org/bgpdata/2021.10/UPDATES/updates.20211001.0000.bz2",
        filters: %{"prefix" => "211.98.251.0/24"}
      ]
    end

    test "return list of maps without filters", %{path: path} do
      result = RustlerExperiment.mrt_parser(path, %{})
      refute Enum.empty?(result)
      result |> Enum.count() |> IO.inspect()
    end

    test "return list of maps with applied filters", %{path: path, filters: filters} do
      result = RustlerExperiment.mrt_parser(path, filters)
      refute Enum.empty?(result)
      result |> Enum.count() |> IO.inspect()
    end
  end
end
