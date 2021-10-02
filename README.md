[![CI](https://github.com/AlexandruIca/verg/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexandruIca/verg/actions/workflows/ci.yml)

# Verg
This is a WIP vector graphics rendering library. Its goal is to be simple to use and easy to embed in existing projects. It doesn't necessarily want to implement a lot of features:
- Rendering cubic and quadratic Bézier curves
- Gradients: linear, radial, conic
- Dashing and stroking
- Clipping and masking
- Support for color spaces
- Correct alpha compositing, including supporting Porter Duff's operators

Any other features will be implemented only if they're not too complicated. The focus right now is on _correct_ rendering rather than having lots of features.

# Setting up a development environment
You can check `shell.nix`(even if you don't use nix) to see the dependencies that are needed(all of them can be gathered with `rustup`).

To run the project's tests:
```sh
cargo test --all-features
```