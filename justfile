# threads := num_cpus() - 1

alias b := build
alias r := run
alias d := drive

default:
    @just --list

build:
    cargo build

run:
    cargo run

drive:
    #!/usr/bin/env fish
    set -l jobs (math (nproc) - 1) # leave one CPU core for interactivity
    set -lx RUST_LOG info
    set -lx RUST_BACKTRACE 0
    cargo.exe run --jobs=$jobs --bin drive

# TODO: use mold here
dev:
    cargo run --features bevy/dynamic_linking
