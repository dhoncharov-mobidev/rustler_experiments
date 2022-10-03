defmodule RustlerExperiment do
  @moduledoc """
  Documentation for `RustlerExperiment`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> RustlerExperiment.hello()
      :world

  """
  def hello do
    :world
  end
end

defmodule RustlerExperiment.Native do
  use Rustler, otp_app: :rustler_experiment, crate: :rustlerexperiment

  def add(_, _), do: :erlang.nif_error(:nif_not_loaded)

end
