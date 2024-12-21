load("@rules_rust//rust:defs.bzl", "rust_library", "rust_binary")

rust_library(
    name = "led_lib",
    srcs = ["lib.rs"],  # Ensure this matches the file's location
    crate_root = "lib.rs",
    crate_name = "led",
)

rust_binary(
    name = "main_bin",
    srcs = ["main.rs"],  # Entry point for the binary
    crate_root = "main.rs",
    deps = [":led_lib"],  # Link the library as a dependency
)
    