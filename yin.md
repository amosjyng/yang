# ZAMM

## yang 陽

I've [mentioned](https://github.com/amosjyng/yin/blob/master/yin.md) Yang a lot already, but I've never formally introduced the two of you. Meet Yang, a code-generation tool. Traditional, worldly, and experienced, he knows all about the cool spots in his little digital neighborhood, all the idiosyncrasies and quirks of his down-to-earth neighbors. Ever the ruthless pragmatic, he has a healthy disregard for the pious rectitude of the compilers. He wishes badly to explore the world outside, but he is fated to stay in this little Rustic village until Yin comes for a visit.

Here's what Yang knows: implementing is an action it can take:

```rust
define!(implement);
```

Implementations are lower-level concepts that *target* specific higher-level concepts.

```rust
define!(target);
target.add_parent(Attribute::archetype().as_archetype());
```

We need some way of efficiently distinguishing Yin concepts from each other. We can't just compare the memory addresses of a Yin struct, since different ephemeral structs may in fact be referring to the same concept. An easy way to do this is by assigning a different integer ID to each one, so that each Yin concept effectively becomes a wrapper around an integer, and we are just defining the relations between different integers:

```rust
define!(concept_id);
concept_id.add_parent(Attribute::archetype().as_archetype());
```

When implementing anything in Rust, we should consider documenting it for the user's sake:

```rust
define!(documentation);
documentation.add_parent(Attribute::archetype().as_archetype());
```

Each data primitive has an associated primitive type in Rust. We should define an attribute for this:

```rust
define!(rust_primitive);
rust_primitive.add_parent(Attribute::archetype().as_archetype());
```

This is basically build information, except that it's information about how this primitive is built inside of Rust, as opposed to how this primitive is built as a higher-level Yin concept. Both representations ultimately refer to the same basic idea, but the two representations live on different levels and interact with different neighbors. The Rust primitive interacts with other Rust code, and the Yin concept interacts with other Yin concepts. Even though all Yin concepts are currently implemented in Rust anyways, the specifics of the Rust language has little impact on the Yin API and abstractions.

When Yin tells us about herself, we must forget all preconceptions we have about the world and listen to what she has to say. That means when she speaks of what an attribute is, we *listen* instead of shoehorning her description into what we already think of as an attribute.

However, this also means that Yin's new attribute node won't be the same `Attribute` node that Yang ties all his custom attribute generation code to. Until all of the logic that goes into generating attributes becomes fully defined in graph form, we're going to need some way of telling Yang to activate that custom logic for newly defined nodes that don't inherit from the existing `Attribute` node:

```rust
define!(uses_attribute_logic);
uses_attribute_logic.add_parent(Flag::archetype());
```

The same is true of Tao and Data:

```rust
define!(uses_root_node_logic);
uses_root_node_logic.add_parent(Flag::archetype());

define!(uses_data_logic);
uses_data_logic.add_parent(Flag::archetype());
```

Unlike the markers for data and attribute logic, the root node marker does not get inherited because, well, the children of the root node won't really be the root node anymore.

Due to current limitations with Yang, we cannot set Tao as the parent here. We should start tracking what has and hasn't gotten introduced in this particular build (and not, say, pre-existing as a part of the dependencies):

```rust
define!(newly_defined);
newly_defined.add_parent(Flag::archetype());
```

During implementation, we should be able to force a new attribute to live inside its own module. This override should take place even if the concept doesn't have any child archetypes yet, so that any concepts in downstream packages that depend on it will know where to look:

```rust
define!(own_module);
own_module.add_parent(Flag::archetype());
```

Once built, structs have a certain import path:

```rust
define!(import_path);
import_path.add_parent(Attribute::archetype().as_archetype());
```

All this can apply to any concept at all that's being implemented. However, these attributes are only meaningful within the context of code generation. As such, they should live inside a build config lens -- a way of viewing concepts through a different perspective than usual.

```rust
define!(build_info);
build_info.add_parent(Form::archetype());
```

Everything is a lens. The `relation` branch of the knowledge base's inheritance tree views all nodes through the lens of relating other nodes to each other (even forms with multiple attributes can be considered higher n-ary relations), the `form` branch views all its leaf nodes as instances of their ancestor chain, the `archetype` branch views all non-leaf nodes as types to be reasoned with ala type theory. We don't put all these under a root `Lens` node because when everything is a lens, the distinction ceases to be meaningful. Alternatively, the only lens that applies to everything is the `Tao` node, the lens through which everything is only just a number or a string label.

What *does* make sense is distinguishing context-dependent lens from universal ones. There will always be forms and relations no matter which subject you look at; there will not always be build-related information outside of the context of a software build. We should define the lens accordingly:

```rust
define!(lens);
```

So to finish up with build information that applies to any implemented concept, everything built in Rust will be part of a crate:

```rust
define!(crate_name);
crate_name.set_internal_name("crate".to_owned());
crate_name.add_parent(Attribute::archetype().as_archetype());
```

Technically, this should instead be a `part-of-crate` relation between the crate (as its own separate concept apart from the part-of-crate relationship) and the implementation of the concept (as its own separate concept apart from the original concept). Furthermore, as it stands right now this relation points only to the name of the crate, as opposed to the crate itself. However, these distinctions don't matter right now.

Besides, it is only natural for the human mind to use the name of the crate as a metonymy for the crate itself, just as humans also tend to use a filename or a file icon as a metonymy for the inode that points to the actual blocks of data on disk. How often do we stop to remind ourselves that the filename is only a symbolic handle for the actual data, or that when we're dragging a file icon from one folder to another, we're not dragging the data but only the visual representation of the data? We don't do so very often, because such details usually don't matter, and so we will also skip them here.

And might also have their own implementation name:

```rust
define!(implementation_name);
implementation_name.add_parent(Attribute::archetype().as_archetype());
```

### Implementation

Unlike with Yin, we don't actually want to implement *everything* we know, because everything we know about Yin is already implemented inside her physical body. We only want to implement the things that we learned about Yang here:

```rust
implement.implement_with_doc(
    "The act of implementing something. When created, this effectively serves as a call to action for Yang."
);

target.implement_with_doc("The target of an implement command.");
concept_id.implement_with_doc("The integer ID associated with a concept.");
documentation.implement_with_doc("The documentation associated with an implementation.");

newly_defined.implement_with_doc(
    "Marks an archetype and all its descendants as having been newly defined as part of this particular build."
);

own_module.implement_with_doc(
    "Marks an archetype as living inside its own module, even if it doesn't have any defined child archetypes yet."
);

rust_primitive.implement_with_doc(
    "The Rust primitive that a Yin data concept is implemented by."
);

uses_attribute_logic.implement_with_doc(
    "Marks an archetype and all its descendants as requiring attribute-specific logic during generation."
);

uses_data_logic.implement_with_doc(
    "Marks an archetype and all its descendants as requiring data-specific logic during generation."
);

uses_root_node_logic.implement_with_doc(
    "Marks an archetype as requiring root-node-specific logic during generation. None of its descendants will inherit this."
);

import_path.implement_with_doc("Describes the import path of a defined struct.");
build_info.implement_with_doc("Represents build information about a generated concept.");

lens.implement_with_doc(
    "Describes a way of looking at things that is only well-defined within a specific context."
);

crate_name.implement_with_doc("Crate that a concept was built as a part of.");

implementation_name.implement_with_doc("Name the concept actually took on when implemented.");
```

## Appendix

### Dependencies

These are the versions of Yin and Yang used to make this build happen:

```toml
zamm_yin = "=0.1.0"
zamm_yang = "=0.1.1"
```

### Imports

These are the generic imports for general Yang generation:

```rust
use zamm_yin::tao::Tao;
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::archetype::ArchetypeFormTrait;
use zamm_yin::tao::archetype::AttributeArchetype;
use zamm_yin::tao::form::FormTrait;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yang::codegen::CodegenConfig;
use zamm_yang::tao::callbacks::handle_all_implementations;
use zamm_yang::tao::initialize_kb;
use zamm_yang::tao::Implement;
use zamm_yang::tao::ImplementConfig;
use zamm_yang::tao::archetype::CodegenFlags;
use zamm_yang::tao::archetype::CreateImplementation;
use zamm_yang::tao::form::DefinedMarker;
use zamm_yang::tao::form::data::DataExtension;
use zamm_yang::define;
use zamm_yang::helper::aa;
```

These are the imports specific to building on top of Yin:

```rust
use zamm_yin::tao::relation::attribute::Attribute;
use zamm_yin::tao::relation::flag::Flag;
use zamm_yin::tao::form::Form;
```
