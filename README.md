# Hyperstone

A (WIP) Dota 2 replay parser written in Rust.

Only handles Source 2 replays.

Code is currently just being ported over from [dotabuff/manta](https://github.com/dotabuff/manta).

# Building

A simple `cargo build` will generate all the required Rust modules from the protobuf files before
compiling the crate itself. The generated modules will be placed under `src/protos/`, but you should
not have to touch these files yourself.
