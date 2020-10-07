# yang 陽

[![Build Status](https://travis-ci.com/amosjyng/yang.svg?branch=main)](https://travis-ci.com/amosjyng/yang)

Yang is a rudimentary, experimental code generator. It both depends on [Yin](https://github.com/amosjyng/yin) to manage its internal state, and currently only generates code for Yin.

This crate is named `zamm_yang` for consistency with `zamm_yin`. However, the name for this tool is just `yang` for all intents and purposes within the ZAMM universe.

## Install

```sh
cargo install yang
```

## Usage

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
    -d, --documentation <DOC>    Documentation string for concept.
    -i, --id <ID>                ID of concept.

ARGS:
    <CONCEPT>    Name of concept to generate code for.
```

### Example

To generate code for the `Target` attribute with an ID offset of 2 from Yin's max id, run:

```sh
yang Target --id 2 -d "The target of an implement command."
```
