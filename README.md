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
    -d, --documentation <DOC>    Documentation string for concept.
    -i, --id <ID>                ID of concept.

ARGS:
    <CONCEPT>    Name of concept to generate code for.
```

#### Example

To generate code for the `Target` attribute with an ID offset of 2 from Yin's max id, run:

```sh
yang Target --id 2 -d "The target of an implement command."
```

After that, you can use the newly generated `src/concepts/attributes/target.rs` as such:

```rust
mod concepts {
    pub mod attributes {
        pub mod target;
    }
}

use concepts::attributes::target::Target;
use zamm_yin::concepts::attributes::Inherits;
use zamm_yin::concepts::ArchetypeTrait;
use zamm_yin::graph::{bind_in_memory_graph, Graph, InjectionGraph};
use zamm_yin::wrappers::CommonNodeTrait;

fn main() {
    // Initialize Yin KB with new type
    bind_in_memory_graph();
    let mut ig = InjectionGraph::new();
    zamm_yin::initialize_type!(ig, (Target));

    let mut target = Target::individuate();
    target.set_internal_name("Hello, world.".to_string());
    println!("{}", target.internal_name().unwrap());
}
```

Make sure to add `src/concepts/attributes/target.rs` to your `.gitignore` afterwards.

### As a library

If you're using a custom build script, chances are you will want to generate Yin concept files for your project directly rather than passing comamndline options to the `yang` binary. For example, to automatically generate the example file above at build time, first add `yin` and `yang` as build dependencies:

```toml
[build-dependencies]
zamm_yin = "0.0.2"
zamm_yang = "0.0.3"
```

Then, define your `build.rs` as such:

```rust
use zamm_yang::codegen::handle_implementation;
use zamm_yang::concepts::{initialize_kb, set_documentation, Implement};
use zamm_yin::concepts::{ArchetypeTrait, Tao};
use zamm_yin::wrappers::CommonNodeTrait;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    initialize_kb();

    let mut target = Tao::archetype().individuate_as_archetype();
    target.set_internal_name("Target".to_string());
    set_documentation(Box::new(&mut target), "The target of an implement command.");

    let mut implement_command = Implement::individuate();
    implement_command.set_target(target);

    handle_implementation(implement_command, 1);
}
```

Now you can use the newly generated `src/concepts/attributes/target.rs` in the same way as above.
