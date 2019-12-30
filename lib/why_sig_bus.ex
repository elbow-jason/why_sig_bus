defmodule WhySigBus do
  @moduledoc """
  Anything other than an empty tuple causes SIGBUS.
  """
  use Rustler, otp_app: :why_sig_bus, crate: "whysigbus_native"

  def decode_args(_args), do: :erlang.nif_error(:nif_not_loaded)
end
