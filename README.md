# yang 陽

[![Build Status](https://travis-ci.com/amosjyng/yang.svg?branch=main)](https://travis-ci.com/amosjyng/yang)

Yang is a rudimentary, experimental code generator. It both depends on [Yin](https://github.com/amosjyng/yin) to manage its internal state, and currently only generates code for Yin.

This crate is named `zamm_yang` for consistency with `zamm_yin`. However, the name for this tool is just `yang` for all intents and purposes within the ZAMM universe.

## Install

```sh
cargo install yang
```

## Usage

### As a binary

```text
yang 0.0.3
Amos Ng <me@amos.ng>
Code generator for Yin.

USAGE:
    yang [OPTIONS] <CONCEPT> --id <ID>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --comment_autogen <COMMENT_AUTOGEN>
            Whether or not to add an autogeneration comment to each generated    
            line of code. Defaults to true.
    -d, --documentation <DOC>                  Documentation string for concept. 
    -i, --id <ID>                              ID offset from Yin's max id.      

ARGS:
    <CONCEPT>    Name of concept to generate code for.
```

#### Example

To generate code for the `Target` attribute with an ID offset of 1 from Yin's max id, run:

```sh
yang Target --id 1 -d "The target of an implement command."
```

After that, you can use the newly generated `src/concepts/attributes/target.rs` as in the [example](examples/result/main.rs). Make sure to add `src/concepts/attributes/target.rs` to your `.gitignore` afterwards.

### As a library

If you're using a custom build script, chances are you will want to generate Yin concept files for your project directly rather than passing comamndline options to the `yang` binary. For example, to automatically generate the example file above at build time, first add `yin` and `yang` as build dependencies:

```toml
[build-dependencies]
zamm_yin = "0.0.2"
zamm_yang = "0.0.3"
```

Then, define your `build.rs` as in the [example](examples/build.rs), and your `main.rs` in the same way as above.
