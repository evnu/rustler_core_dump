defmodule RustlerCoreDump do
  use Rustler, otp_app: :rustler_core_dump, crate: "rustler_core_dump"

  def roundtrip(_term), do: throw(:nif_not_loaded)

  def test do
    reference = make_ref()
    IO.inspect(reference)
    {1, ^reference} = roundtrip(reference)
  end
end
