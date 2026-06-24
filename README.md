<p align="center">
  <img src="logo.png" width="96" alt="Quasar Schema logo">
  <h1 align="center">QuaSAR Schema</h1>
  <h5 align="center"><b>QuaSAR</b> protobuf schema library for the QuaSAR project.</h5>
</p>

QuaSAR Schema defines protobuf schemas for SAR telemetry, SAR commands, track
management, network telemetry, and discovery messages used over ZeroMQ.

The project is published as:

- a Rust crate named `quasar_schema`;
- a C++ Conan package named `quasar_schema`, exported as a shared library.

The schema intentionally does not define ZeroMQ endpoints, topics, or runtime
configuration. It documents socket roles and data flow in comments and through
descriptor options from `quasar/zmq.proto`.

## Schema Layout

All schema files use `syntax = "proto3"`.
Shared domain types come from `mms_protocol` / `mms::protocol`:
- `mms.pb.Uuid`
- `mms.pb.LatLon`
- `mms.pb.Dim3`
- `mms.pb.EulerAngles`
- `mms.pb.Sockaddr`
- `mms.pb.SockaddrV4`
- `mms.pb.Version`

See [docs/zmq.md](docs/zmq.md) for socket roles and message flow.

## Rust Crate

```toml
[dependencies]
quasar_schema = { registry = "whs31", version = "1" }
```

The crate uses `prost` and requires `protoc` at build time unless the
`vendored-protobuf` feature is enabled:

```toml
quasar_schema = { registry = "whs31", version = "1", features = ["vendored-protobuf"] }
```

Feature flags:

| Feature | Effect |
| --- | --- |
| `serde` | Adds serde derives to generated messages/enums and enables `mms_protocol/serde` |
| `legacy` | Passes `--experimental_allow_proto3_optional` to `protoc` |
| `wkt` | Generates local Rust code for Google well-known types |
| `vendored-protobuf` | Uses the `protobuf-src` crate to provide `protoc` |

Rust re-exports generated modules from `quasar.pb`.

Downstream schema crates can import QuaSAR Schema files with:
```rust
use quasar_schema::{PROTO_FILES, PROTO_INCLUDE_DIR};
```

`PROTO_INCLUDE_DIR` points at the repository `schema/` directory, and
`PROTO_FILES` lists the `quasar/*.proto` files relative to that directory.

## C++ Conan Package

```python
def requirements(self):
    self.requires("quasar_schema/1.0.0@quasar/dev")
```

```cmake
find_package(QuaSARSchema REQUIRED)
target_link_libraries(my_target PRIVATE quasar::schema)
```

The C++ package:

- builds `quasar_schema` as a shared library;
- links shared `protobuf`, shared `abseil`, and `mms::protocol`;
- installs generated protobuf headers under `include/quasar/*.pb.h`;
- installs raw schema files under `schema/quasar/*.proto`;
- exposes the schema directory as a Conan resource directory.

Generated C++ classes live in namespaces matching the protobuf packages, for
example:

## Building

### Rust

```sh
cargo build
cargo test
```

### C++ with Conan

```sh
conan build . --build=missing
```

The Conan recipe passes the `mms::protocol` schema resource directory to CMake
as `QUASAR_SCHEMA_MMS_PROTO_DIR`. For standalone CMake builds, set that cache variable
to the directory containing `mms/*.proto`.

## Development

Format sources:
```sh
just format # or just fmt
```
