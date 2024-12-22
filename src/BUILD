load("@rules_rust//rust:defs.bzl", "rust_library", "rust_binary",)

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "led_lib",
    srcs = ["led.rs"],  # Ensure this matches the file's location
    # crate_root = "led.rs",
    crate_name = "led",
)
rust_library(
    name = "sys_calls_lib",
    srcs = ["syscalls.rs"],
    deps = [
        "@crates//:libc",
    ],
)


rust_binary(
    name = "main_bin",
    srcs = ["main.rs"],  # Entry point for the binary
    crate_root = "main.rs",
    deps = [
        ":led_lib",
        ":sys_calls_lib",
    ],
)
    