defmodule Rusty.Mixfile do
  use Mix.Project

  def project do
    [
      app: :rustler_core_dump,
      version: "0.1.0",
      elixir: "~> 1.5",
      start_permanent: Mix.env == :prod,
      deps: deps(),
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, git: "https://github.com/hansihe/rustler", sparse: "rustler_mix"}
    ]
  end

  defp rustler_crates do
    [
      rustler_core_dump: [
        path: "native/rustler_core_dump",
        mode: :debug
      ]
    ]
  end
end
