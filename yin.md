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

Getters and setters in particular have their own dual-purpose documentation strings:

```rust
define!(dual_purpose_documentation);
dual_purpose_documentation.add_parent(Attribute::archetype().as_archetype());
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
define!(attribute_analogue);
attribute_analogue.add_parent(Flag::archetype());
```

The same is true of Tao and Data:

```rust
define!(uses_root_node_logic);
uses_root_node_logic.add_parent(Flag::archetype());

define!(uses_data_logic);
uses_data_logic.add_parent(Flag::archetype());
```

Unlike the markers for data and attribute logic, the root node marker does not get inherited because, well, the children of the root node won't really be the root node anymore.

### Perspective

All this can apply to any concept at all that's being implemented. However, these attributes are only meaningful within the context of code generation. As such, they should live inside a build config lens -- a way of viewing concepts through a different perspective than usual.

Everything is a perspective. The `relation` branch of the knowledge base's inheritance tree views all nodes through the lens of relating other nodes to each other (even forms with multiple attributes can be considered higher n-ary relations), the `form` branch views all its leaf nodes as instances of their ancestor chain, the `archetype` branch views all non-leaf nodes as types to be reasoned with ala type theory. We don't put all these under a root `Perspective` node because when everything is a perspective, the distinction ceases to be meaningful. Alternatively, the only perspective that applies to everything is the `Tao` node, the lens through which everything is only just a number or a string label.

What *does* make sense is distinguishing context-dependent perspectives from each other. There are no universal perspectives. Some context-dependent perspectives are more obvious than other ones: for example, there will not always be build-related information outside of the context of a software build. Others seem tantalizingly universal. You may think will always be forms and relations no matter which subject you look at; that is true only from an abstract meta perspective. On the more practical side of things, most people go through their daily lives without ever bothering to think explicitly of forms and relations between the forms. It is therefore not a universal perspective -- not because it can't be applied universally (it can), but because it's not universally required or useful.

To put it in more geometric terms, there's no absolute perspective from which to look at a 3D scene. Every vantage point is as arbitrary as any other -- even the 4D one. Were we to be four-dimensional beings who could step out into the w-axis and look at the 3D scene in all its entirety simultaneously, we would still be viewing it from an arbitrary location on the w-axis, our vision pointed in the direction of an arbitrary 4-vector. Even the number of dimensions is arbitrary -- who's to say we can't look at the 3D scene projected onto a single-dimensional line? Or to look at it from the fifth dimension, the same way we look at open and closed intervals on the number line in our own 3D world? There is no objectively "correct" perspective.

There is the origin, defined to be so by fiat. If there's any absolute perspective to be found, it is surely at the intersection of all the axes. But the thing about having the power to define things by fiat is that someone else can do just the same thing, and define their coordinate system with a different origin. You embed the foundations of your grand edifice deep, deep into the solid ground, only to discover that the entire Earth is a Brownian particle floating arbitrarily through the expanse of space.

Everything is relative -- but of course even relativity is relative. Physics is to a large extent the study of fundamental truths that are absolute across all reference frames of this reality. In one direction is mathematics, the study of fundamental truths that are absolute across all potential realities, but still only true relative to their axioms. In the other direction are human social norms, absolute across all individuals in a small community, but relative to the rest of the myriad cultures and subcultures in the world. To say that everything is relative is a monism that borders on uselessness. Relativity is only meaningful when contrasted next to absoluteness, and the relative-absolute dichotomy is only one of many perspectives to view the world through.

```rust
define!(perspective);
```

One perspective through which to look at things is the knowledge graph perspective, through which everything described here are equally first-class concept nodes. There are superficial differences between the nodes and how well-connected, yes, but all nodes are fundamentally equivalent as pieces of knowledge. We look at knowledge the same way humanism looks at humans.

```rust
define!(knowledge_graph_node);
knowledge_graph_node.add_parent(perspective);
```

As part of this perspective, We should start tracking what has and hasn't gotten introduced in this particular build (and not, say, pre-existing as a part of the dependencies):

```rust
define!(newly_defined);
newly_defined.add_parent(Flag::archetype());
aa(newly_defined).set_owner_archetype(knowledge_graph_node);
knowledge_graph_node.add_flag(newly_defined);
```

We'll do the same for other knowledge-graph-related nodes:

```rust
aa(attribute_analogue).set_owner_archetype(knowledge_graph_node);
knowledge_graph_node.add_flag(attribute_analogue);
```

Rust groups things by modules.

```rust
define!(module);
module.add_parent(Form::archetype());
```

Things that are grouped inside of a module will be considered a member of the module:

```rust
define!(has_member);
has_member.add_parent(Attribute::archetype().as_archetype());
```

Rust modules sometimes re-export things so that it looks like it's coming from that module.

```rust
define!(re_exports);
re_exports.add_parent(Attribute::archetype().as_archetype());
```

It seems to make sense to group a concept and its descendants inside the same module. For such modules, we'll mark the concept as the most prominent member of the module.

```rust
define!(most_prominent_member);
most_prominent_member.add_parent(Attribute::archetype().as_archetype());
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

```rust
define!(build_info);
build_info.add_parent(Form::archetype());
```

So to finish up with build information that applies to any implemented concept, everything built in Rust will be part of a crate.

```rust
define!(crate_concept);
crate_concept.set_internal_name_str("crate");
crate_concept.add_parent(Form::archetype());
```

We can reuse the existing generic `HasMember` relation for describing the relationship between a concept and its crate, because there is nothing special about this particular membership scenario that warrants a separate membership concept specifically for this.

Crates are versioned:

```rust
define!(version);
version.add_parent(Attribute::archetype().as_archetype());
```

Concepts and crates alike might also have their own implementation name:

```rust
define!(implementation_name);
implementation_name.add_parent(Attribute::archetype().as_archetype());
```

It is only natural for the human mind to use the name of the crate as a metonymy for the crate itself, just as humans also tend to use a filename or a file icon as a metonymy for the inode that points to the actual blocks of data on disk. How often do we stop to remind ourselves that the filename is only a symbolic handle for the actual data, or that when we're dragging a file icon from one folder to another, we're not dragging the data but only the visual representation of the data? We don't do so very often, because such details usually don't matter. It does matter here, however, so we will keep them separate and distinct.

### Implementation

Unlike with Yin, we don't actually want to implement *everything* we know, because everything we know about Yin is already implemented inside her physical body. We only want to implement the things that we learned about Yang here:

```rust
implement.implement_with_doc(
    "The act of implementing something. When created, this effectively serves as a call to action for Yang."
);

target.implement_with_doc("The target of an implement command.");
concept_id.implement_with_doc("The integer ID associated with a concept.");
documentation.implement_with_doc("The documentation associated with an implementation.");
dual_purpose_documentation.implement_with_doc(
    "Dual-purpose documentation that can be used in more than one situation.\n\nFor example, the same substring might be usable for both the getter and setter of a string."
);

let mut nd_impl = newly_defined.implement_with_doc(
    "Marks an archetype and all its descendants as having been newly defined as part of this particular build."
);
nd_impl.dual_document("having been newly defined as part of the current build.");

module.implement_with_doc("Concept representing a Rust module.");

has_member.implement_with_doc(
    "Marks the value as being part of the owner. The owner should presumably be a collection of some sort."
);

re_exports.implement_with_doc("Marks the owner module as re-exporting the value symbol.");

most_prominent_member.implement_with_doc(
    "The most prominent member of a Rust module. The module will take its name after this member."
);

own_module.implement_with_doc(
    "Marks an archetype as living inside its own module, even if it doesn't have any defined child archetypes yet."
);

rust_primitive.implement_with_doc(
    "The Rust primitive that a Yin data concept is implemented by."
);

let mut aa_impl = attribute_analogue.implement_with_doc(
    "Marks an archetype and all its descendants as requiring attribute-specific logic during generation."
);
aa_impl.dual_document("as logically analogous to an attribute node.");

uses_data_logic.implement_with_doc(
    "Marks an archetype and all its descendants as requiring data-specific logic during generation."
);

uses_root_node_logic.implement_with_doc(
    "Marks an archetype as requiring root-node-specific logic during generation. None of its descendants will inherit this."
);
import_path.implement_with_doc("Describes the import path of a defined struct.");
build_info.implement_with_doc("Represents build information about a generated concept.");

perspective.implement_with_doc(
    "Describes a way of looking at things that is only well-defined within a specific context."
);
perspective.impl_mod("Perspectives on the world.");
knowledge_graph_node.implement_with_doc("Look at all information as knowledge graph entities.");

crate_concept.implement_with_doc("Crate that a concept was built as a part of.");
version.implement_with_doc("Version number for a versioned object.");
implementation_name.implement_with_doc("Name the concept actually took on when implemented.");
```

Last but not least, let's make sure to also define the modules for concepts that were first introduced in Yin, but which we have since created new children for:

```rust
let mut form_mod = Form::archetype().impl_mod("All things that can be interacted with have form.");
form_mod.has_extension("build_info_extension::BuildInfoExtension");
form_mod.has_extension("crate_extension::CrateExtension");
form_mod.has_extension("defined_marker::DefinedMarker");
form_mod.has_extension("module_extension::ModuleExtension");

let mut archetype_mod = Archetype::archetype().impl_mod("Types of forms, as opposed to the forms themselves.");
archetype_mod.has_extension("attribute_activation::CodegenFlags");
archetype_mod.has_extension("create_implementation::CreateImplementation");

let mut data_mod = Data::archetype().impl_mod(
    "Data that actually exist concretely as bits on the machine, as opposed to only existing as a hypothetical, as an idea."
);
data_mod.has_extension("data_extension::DataExtension");

Relation::archetype().impl_mod("Relations between the forms.");
Flag::archetype().impl_mod("Relations involving only one form.");

let mut attr_mod = Attribute::archetype().impl_mod("Relations between two forms.");
attr_mod.has_extension("supports_membership::SupportsMembership");

HasProperty::archetype().impl_mod(
    "Meta-attributes around what attributes instances of an archetype have."
);
```

We should really save the build info, so that one day we will no longer need to redefine the documentation for these modules.

## Appendix

### Dependencies

This is the version of Yang used to make this build happen:

```toml
zamm_yang = "0.1.6"
```

Yang does his best to be backwards-compatible, so we should let old Yang know that this is new Yang speaking:

```rust
Crate::yin().set_version("0.1.4");
```

We should also let him know what our current crate name is. There is as of yet no way to let him know that this is the same crate as the `Crate::yang()` in the knowledge base, or that this crate is a newer version of himself. Unfortunately, there is no self-awareness yet, only instinct.

```rust
Crate::current().set_implementation_name("zamm_yang");
```

### Imports

These are the generic imports for general Yang generation:

```rust
use zamm_yang::define;
use zamm_yang::tao::initialize_kb;
use zamm_yang::tao::Tao;
use zamm_yang::tao::ImplementExtension;
use zamm_yang::tao::archetype::ArchetypeTrait;
use zamm_yang::tao::archetype::ArchetypeFormTrait;
use zamm_yang::tao::archetype::AttributeArchetypeFormTrait;
use zamm_yang::tao::archetype::ArchetypeFormExtensionTrait;
use zamm_yang::tao::archetype::CreateImplementation;
use zamm_yang::tao::archetype::CodegenFlags;
use zamm_yang::tao::form::Crate;
use zamm_yang::tao::form::CrateExtension;
use zamm_yang::tao::form::Form;
use zamm_yang::tao::form::FormTrait;
use zamm_yang::tao::form::ModuleExtension;
use zamm_yang::tao::callbacks::handle_all_implementations;
use zamm_yang::codegen::CodegenConfig;
use zamm_yang::node_wrappers::CommonNodeTrait;
use zamm_yang::helper::aa;
```

These are the imports specific to building on top of Yin:

```rust
use zamm_yang::tao::form::data::Data;
use zamm_yang::tao::archetype::Archetype;
use zamm_yang::tao::relation::Relation;
use zamm_yang::tao::relation::attribute::Attribute;
use zamm_yang::tao::relation::attribute::has_property::HasProperty;
use zamm_yang::tao::relation::flag::Flag;
```

### Backwards compatibility

There is some data in the latest Yin that won't be generated by the current Yang, because the current Yang relies on the previous Yin.

```rust
use zamm_yang::tao::relation::attribute::Owner;
Relation::archetype().add_attribute_type(Owner::archetype());
```
