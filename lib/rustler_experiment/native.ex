defmodule RustlerExperiment.Native do
  use Rustler, otp_app: :rustler_experiment, crate: :rustlerexperiment

  def add(_, _), do: :erlang.nif_error(:nif_not_loaded)
  def mrt_parser(_), do: :erlang.nif_error(:nif_not_loaded)
  def sample_list(_filters \\ [default: "default"]), do: :erlang.nif_error(:nif_not_loaded)
end
