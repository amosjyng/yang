# yang 陽

[![Build Status](https://travis-ci.com/amosjyng/yang.svg?branch=main)](https://travis-ci.com/amosjyng/yang)

Yang is a rudimentary, experimental code generator. Yang is both used by, and depends on, [Yin](https://crates.io/crates/zamm_yin) for code generation. See [`yin.md`](yin.md) for an in-depth description of its internals.

This crate is named `zamm_yang` for consistency with `zamm_yin`. However, the name for this tool is just `yang` for all intents and purposes within the ZAMM universe.

## Usage

First, add `yin` and `yang` as build dependencies. Then, define your [`build.rs`](examples/build.rs) for concept generation, and your [`main.rs`](examples/result/main.rs) to use the generated concepts (examples linked). Make sure to initialize the newly generated concepts on KB startup.

### Literate programming

You can use [ZAMM](https://crates.io/crates/zamm) to automatically use Yang to generate code out of READMEs.

## Development

Note that due to Yang using its autogeneration capabilities on itself, it requires a pre-existing binary of itself in order to build its current version. The `build.rs` script will attempt to download one from Bintray if it isn't found locally. If none of the versions of Bintray are available for your platform, you can bootstrap Yang by building each version consecutively and setting the `YANG_BINARY` environment variable to the location of the newly built `yang` executable.
