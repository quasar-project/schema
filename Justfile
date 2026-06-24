alias v := version
alias fmt := format

default:
    @echo "Usage: just <command> [options]"
    @just --list

graph:
    conan graph info . --format=html > graph.html
    mkdir -p docs
    mv graph.html docs

version *ARGS='--show':
    @wucc version {{ ARGS }}

[doc("Formats Rust and C++ code")]
format:
    cargo +nightly fmt --all
    find . -name '*.c' -o -name '*.cc' -o -name '*.cpp' -o -name '*.h' -o -name '*.hh' -o -name '*.hpp' -exec clang-format -i {} \;
