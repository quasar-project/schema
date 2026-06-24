alias v := version

default:
    @echo "Usage: just <command> [options]"
    @just --list

graph:
    conan graph info . --format=html > graph.html
    mkdir -p docs
    mv graph.html docs

version *ARGS='--show':
    @wucc version {{ ARGS }}

generate-ts:
    @cargo test --features=serde,grpc,client,server,typescript