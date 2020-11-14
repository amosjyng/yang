# yang 陽

[![Build Status](https://travis-ci.com/amosjyng/yang.svg?branch=main)](https://travis-ci.com/amosjyng/yang)

Yang is a rudimentary, experimental code generator. Yang is both used by, and depends on, [Yin](https://crates.io/crates/zamm_yin) for code generation. See [`yin.md`](yin.md) for an in-depth description of its internals.

This crate is named `zamm_yang` for consistency with `zamm_yin`. However, the name for this tool is just `yang` for all intents and purposes within the ZAMM universe.

## Install

```sh
cargo install zamm_yang
```

## Usage

### As a binary

```text
yang 0.0.12
Amos Ng <me@amos.ng>
Code generator for Yin.

USAGE:
    yang <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build      Generate code from an input file
    clean      Clean up autogenerated files
    help       Prints this message or the help of the given subcommand(s)
    release    Prepare repo for a Cargo release
    test       Make sure the project will pass CI tests
```

#### Example

See this project's very own [`yin.md`](yin.md) for an example configuration file. Run

```sh
yang build
```

and you can use the newly generated `src/concepts/attributes/target.rs` as in the [example](examples/result/main.rs). Make sure to initialize the new `Target` concept on KB startup

### As a library

If you're using a custom build script, chances are you will want to generate Yin concept files for your project directly rather than passing comamndline options to the `yang` binary.

For example, to automatically generate the example file above at build time, first add `yin` and `yang` as build dependencies. Then, define your `build.rs` as in the [example](examples/build.rs), and your `main.rs` in the same way as above.

## Development

Note that due to Yang using its autogeneration capabilities on itself, it requires a pre-existing binary of itself in order to build its current version. The `build.rs` script will attempt to download one from Bintray if it isn't found locally. If none of the versions of Bintray are available for your platform, you can bootstrap Yang by building each version consecutively and setting the `YANG_BINARY` environment variable to the location of the newly built `yang` executable.
