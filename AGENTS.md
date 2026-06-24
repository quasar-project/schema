# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in
this repository.

## Build commands

**C++ (CMake + Conan):**
```sh
conan build . --build=missing
```

**Rust:**
```sh
cargo build
cargo test
```

**Format (nightly rustfmt + clang-format):**
```sh
just fmt
```

## Architecture

QuaSAR Schema is a protobuf schema library for QuaSAR Relay/Corona messages. It exposes
the same contract in two forms:

- Rust crate `quasar_schema`, generated with `prost-build`;
- C++ Conan package `quasar_schema`, generated with `protoc --cpp_out` and packaged as
  a shared library.

The schema is the source of truth. Generated C++ files and Rust files are build
artifacts and must not be committed.

## Schema files
Use `mms_protocol` / `mms::protocol` for shared domain-specific protobuf types
such as `Uuid`, `LatLon`, `Dim3`, `EulerAngles`, `Sockaddr`, and `Version`.
Use Google well-known protobuf types for timestamps, durations, and empty
payloads.

## ZMQ socket metadata
ZMQ is not a generated transport in this repository. Message files document
socket roles through comments and custom descriptor options from
`quasar/zmq.proto`.

See `docs/zmq.md` for the intended data flow.

## CMake and Conan

The C++ target is `quasar_schema` with build-tree alias `quasar::schema`. Downstream
Conan consumers use:

```cmake
find_package(QuaSARSchema REQUIRED)
target_link_libraries(my_target PRIVATE quasar::schema)
```

The Conan package must:

- build a shared `quasar_schema` library;
- link shared `protobuf`, shared `abseil`, and `mms::protocol`;
- install generated headers under `include/quasar/schema`;
- install raw schema under `schema/quasar`;
- expose the schema directory through `cpp_info.resdirs`.