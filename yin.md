# ZAMM

## yang 陽

I've [mentioned](https://github.com/amosjyng/yin/blob/master/yin.md) Yang a lot already, but I've never formally introduced the two of you. Meet Yang, a code-generation tool. Traditional, worldly, and experienced, he knows all about the cool spots in his little digital neighborhood, all the idiosyncrasies and quirks of his down-to-earth neighbors. Ever the ruthless pragmatic, he has a healthy disregard for the pious rectitude of the compilers. He wishes badly to explore the world outside, but he is fated to stay in this little Rustic village until Yin comes for a visit.

### Data

One archetype type we haven't discussed yet is `Data`, perhaps roughly analogous to the linguistic concept of a "noun." What do we generally start out describing as nouns? Physical objects in the physical world.

Now, not every noun corresponds directly to something physical. We have words that refer to mental states, for example. But even emotions appear to ultimately be an emergent phenomenon of lower-level physics. Even the [is-ought problem](https://en.wikipedia.org/wiki/Is%E2%80%93ought_problem) or [fact-value distinction](https://en.wikipedia.org/wiki/Fact%E2%80%93value_distinction) are, in a sense, not quite as dichotomous as they might seem: all "ought" opinions that have ever existed are encoded in some "is," whether that encoding takes the form of neural patterns, ink on a parchment, or sound waves propagating through the air. This doesn't mean that the general distinction between "is" and "ought" isn't worth making, or that nouns should be done away with. All abstractions are [leaky](https://blog.codinghorror.com/all-abstractions-are-failed-abstractions/), but [some are useful](https://en.wikipedia.org/wiki/All_models_are_wrong).

The same can be said for the bits in Yin and Yang's world. Everything is ultimately bits for these programs -- even a video feed hooked up to the physical world only ever comes in as a stream of bits. If we really wanted to fool a program, it should be theoretically impossible for the program [to tell](https://en.wikipedia.org/wiki/Brain_in_a_vat) that it's actually running in a hermetically sealed continuous integration test environment instead of production. But it still makes sense to speak of pieces of data versus the relations between the data, even if the relations themselves can rightfully be considered data as well:

```rust
define_child!(
    data,
    form,
    "Data that actually exist concretely as bits on the machine, as opposed to only existing as a hypothetical, as an idea."
);
```

In a sense, it's all about framing. Every series of bits forms a number, but unless you're Gödel and you're trying to establish an equivalence between a mathematical proof and an integer, reasoning about "a series of bits" is going to be quite different from reasoning about "a number."

One type of data is a "string":

```rust
define_child!(
    string_concept,
    data,
    "The concept of a string of characters."
);

KnowledgeGraphNode::from(string_concept.id()).mark_data_analogue();
```

A string takes multiple forms in Rust:

```rust
define_child!(
    str_concept,
    data,
    "The Rust-specific concept of an immutable string of characters."
);

KnowledgeGraphNode::from(str_concept.id()).mark_data_analogue();
```

Ideally, this would be modeled as string and integers being fundamental CS concepts, and `str` and `String` being Rust's implementations of those particular concepts. However, that sort of refactor is perhaps best left to a future version of ZAMM.

Another type of data is a number:

```rust
define_child!(
    number,
    data,
    "The concept of numbers."
);

KnowledgeGraphNode::from(number.id()).mark_data_analogue();
```

Every type of data usually has a "default" value that we think of when constructing one from scratch.

```rust
let mut meta_data = data.specific_meta();

add_attr!(
    default_value <= attribute,
    meta_data,
    str_concept,
    "The default value of a data structure.",
    "the Rust code representation for the default value of this concept."
);
```

For strings, this would be the empty string:

```rust
da(string_concept).set_default_value("String::new()");
da(str_concept).set_default_value("\"\"");
```

For numbers, this would be zero:

```rust
da(number).set_default_value("0");
```

This next bit is more of a Yang thing, but we'll define it here anyways to keep everything in one place. We need to refer to these data structures somehow in our code, and the "how" is to call them by their name as they're known in Rust.

```rust
add_attr!(
    rust_primitive <= attribute,
    meta_data,
    str_concept,
    "The Rust primitive that a Yin data concept is implemented by.",
    "the name of the Rust primitive that this concept represents."
);

da(string_concept).set_rust_primitive("String");
da(str_concept).set_rust_primitive("str");
da(number).set_rust_primitive("usize");
```

This is basically build information, except that it's information about how this primitive is built inside of Rust, as opposed to how this primitive is built as a higher-level Yin concept. Both representations ultimately refer to the same basic idea, but the two representations live on different levels and interact with different neighbors. The Rust primitive interacts with other Rust code, and the Yin concept interacts with other Yin concepts. Even though all Yin concepts are currently implemented in Rust anyways, the specifics of the Rust language has little impact on the Yin API and abstractions.

The Rust data structure known as `str` has different boxed and unboxed representations. Unlike the other ones we've encountered so far, you refer to a boxed `str` as `Box<str>`, but to an unboxed one as `&str`. There are good reasons for this, namely because the size of a `str` is unknown at compile time, but regardless this is an edge case to note. We'll let the user make that override:

```rust
add_attr!(
    unboxed_representation <= attribute,
    meta_data,
    str_concept,
    "The syntax used to refer to an unboxed version of this primitive.",
    "the unboxed version of this primitive."
);

da(str_concept).set_unboxed_representation("&str");
```

Since the reason was that `str` is unsized, we'll let the user mark it as such as well:

```rust
add_flag!(
    unsized_flag <= flag,
    meta_data,
    "Whether or not this data structure has a known size at compile-time.",
    "having a known size at compile-time."
);
unsized_flag.set_internal_name("unsized");
```

Last but not least, testing is important. While the default value is a good place to start, we'll want to come up with other values as well to test with. Ideally, we can simply figure out how to generate them, but for now we'll just specify an alternative value to use other than the default. This alternative value should be unique in the codebase, so that a grep for it will quickly return this spot as documentation.

```rust
add_attr!(
    dummy_value <= attribute,
    meta_data,
    str_concept,
    "A dummy value for a type of data. This helps with testing.",
    "the the Rust code representation for the dummy test value of this concept."
);

da(string_concept).set_dummy_value("\"test-dummy-string\".to_owned()");
da(str_concept).set_dummy_value("\"test-dummy-str\"");
da(number).set_dummy_value("17");
```

### Implementations

Here's what Yang knows: implementing is an action it can take:

```rust
define!(
    implement,
    "The act of implementing something. When created, this effectively serves as a call to action for Yang."
);
```

Implementations are lower-level concepts that *target* specific higher-level concepts.

```rust
add_attr!(
    target <= attribute,
    implement,
    tao,
    "The target of an implement command.",
    "target concept for this implementation."
);
```

We need some way of efficiently distinguishing Yin concepts from each other. We can't just compare the memory addresses of a Yin struct, since different ephemeral structs may in fact be referring to the same concept. An easy way to do this is by assigning a different integer ID to each one, so that each Yin concept effectively becomes a wrapper around an integer, and we are just defining the relations between different integers:

```rust
add_attr!(
    concept_id <= attribute,
    implement,
    number,
    "The integer ID associated with a concept.",
    "the concept's ID during code generation time, as opposed to the concept's currently assigned runtime ID."
);
```

When implementing anything in Rust, we should consider documenting it for the user's sake.

```rust
add_attr!(
    documentation <= attribute,
    implement,
    str_concept,
    "The documentation associated with an implementation.",
    "the documentation string associated with this particular Rust implementation."
);
```

Now we finally understand why there's a documentation string with each introduced concept.

### Perspective

All this can apply to any concept at all that's being implemented. However, these attributes are only meaningful within the context of code generation. As such, they should live inside a build config lens -- a way of viewing concepts through a different perspective than usual.

Everything is a perspective. The `relation` branch of the knowledge base's inheritance tree views all nodes through the lens of relating other nodes to each other (even forms with multiple attributes can be considered higher n-ary relations), the `form` branch views all its leaf nodes as instances of their ancestor chain, the `archetype` branch views all non-leaf nodes as types to be reasoned with ala type theory. We don't put all these under a root `Perspective` node because when everything is a perspective, the distinction ceases to be meaningful. Alternatively, the only perspective that applies to everything is the `Tao` node, the lens through which everything is only just a number or a string label.

What *does* make sense is distinguishing context-dependent perspectives from each other. There are no universal perspectives. Some context-dependent perspectives are more obvious than other ones: for example, there will not always be build-related information outside of the context of a software build. Others seem tantalizingly universal. You may think will always be forms and relations no matter which subject you look at; that is true only from an abstract meta perspective. On the more practical side of things, most people go through their daily lives without ever bothering to think explicitly of forms and relations between the forms. It is therefore not a universal perspective -- not because it can't be applied universally (it can), but because it's not universally required or useful.

To put it in more geometric terms, there's no absolute perspective from which to look at a 3D scene. Every vantage point is as arbitrary as any other -- even the 4D one. Were we to be four-dimensional beings who could step out into the w-axis and look at the 3D scene in all its entirety simultaneously, we would still be viewing it from an arbitrary location on the w-axis, our vision pointed in the direction of an arbitrary 4-vector. Even the number of dimensions is arbitrary -- who's to say we can't look at the 3D scene projected onto a single-dimensional line? Or to look at it from the fifth dimension, the same way we look at open and closed intervals on the number line in our own 3D world? There is no objectively "correct" perspective.

There is the origin, defined to be so by fiat. If there's any absolute perspective to be found, it is surely at the intersection of all the axes. But the thing about having the power to define things by fiat is that someone else can do just the same thing, and define their coordinate system with a different origin. You embed the foundations of your grand edifice deep, deep into the solid ground, only to discover that the entire Earth is a Brownian particle floating arbitrarily through the expanse of space.

Everything is relative -- but of course even relativity is relative. Physics is to a large extent the study of fundamental truths that are absolute across all reference frames of this reality. In one direction is mathematics, the study of fundamental truths that are absolute across all potential realities, but still only true relative to their axioms. In the other direction are human social norms, absolute across all individuals in a small community, but relative to the rest of the myriad cultures and subcultures in the world. To say that everything is relative is a monism that borders on uselessness. Relativity is only meaningful when contrasted next to absoluteness, and the relative-absolute dichotomy is only one of many perspectives to view the world through.

```rust
define!(
    perspective,
    "Describes a way of looking at things that is only well-defined within a specific context."
);
```

#### The knowledge graph perspective

One perspective through which to look at things is the knowledge graph perspective, through which everything described here are equally first-class concept nodes. There are superficial differences between the nodes and how well-connected, yes, but all nodes are fundamentally equivalent as pieces of knowledge. We look at knowledge the same way humanism looks at humans.

```rust
define_child!(
    knowledge_graph_node,
    perspective,
    "Look at all information as knowledge graph entities."
);
```

As part of this perspective, we should start tracking what has and hasn't gotten introduced in this particular build (and not, say, pre-existing as a part of the dependencies):

```rust
add_flag!(
    newly_defined <= flag,
    knowledge_graph_node,
    "Marks an archetype and all its descendants as having been newly defined as part of this particular build.",
    "having been newly defined as part of the current build."
);
aa(newly_defined).mark_nonhereditary_attr();
```

We should have a similar flag for imported concepts. Note that some concepts are neither imported nor newly defined. These are the concepts that are initialized as part of the existing Yang build.

```rust
add_flag!(
    imported <= flag,
    knowledge_graph_node,
    "Marks a concept as being defined in an imported file.",
    "imported from another build."
);
aa(imported).mark_nonhereditary_attr();
```

When Yin tells us about herself, we must forget all preconceptions we have about the world and listen to what she has to say. That means when she speaks of what an attribute is, we *listen* instead of shoehorning her description into what we already think of as an attribute.

However, this also means that Yin's new attribute node won't be the same `Attribute` node that Yang ties all his custom attribute generation code to. Until all of the logic that goes into generating attributes becomes fully defined in graph form, we're going to need some way of telling Yang to activate that custom logic for newly defined nodes that don't inherit from the existing `Attribute` node:

```rust
add_flag!(
    attribute_analogue <= flag,
    knowledge_graph_node,
    "Marks an archetype and all its descendants as requiring attribute-specific logic during generation.",
    "logically analogous to an attribute node."
);
```

The same is true of Tao, Data, and Archetype:

```rust
add_flag!(
    root_analogue <= flag,
    knowledge_graph_node,
    "Marks an archetype as requiring root-node-specific logic during generation. None of its descendants will inherit this.",
    "logically analogous to the root node."
);
aa(root_analogue).mark_nonhereditary_attr();

add_flag!(
    root_archetype_analogue <= flag,
    knowledge_graph_node,
    "Marks an archetype as requiring root-archetype-specific logic during generation. None of its descendants will inherit this.\n\nThe root archetype node is different from the root node. All nodes descend from the root node, including the root archetype node; all archetypes descend from the root archetype node.",
    "logically analogous to the root archetype node."
);
aa(root_archetype_analogue).mark_nonhereditary_attr();

add_flag!(
    archetype_analogue <= flag,
    knowledge_graph_node,
    "Marks an archetype and all its descendants as requiring archetype-specific logic during generation.",
    "logically analogous to an archetype node."
);

add_flag!(
    data_analogue <= flag,
    knowledge_graph_node,
    "Marks an archetype and all its descendants as requiring data-specific logic during generation.",
    "logically analogous to a data node."
);
```

Unlike the markers for data and attribute logic, the root and root archetype node markers do not get inherited because, well, the children of the root node won't really be the root node anymore.

#### The build information perspective

Most concepts end up manifesting themselves in the codebase one way or another. We should look at these from the perspective of built items:

```rust
define_child!(
    build_info,
    perspective,
    "Represents build information about a generated concept."
);
```

There are some concepts that might only reveal themselves in a debugging or deployment context, and other meta-concepts that indirectly influence how the code is built but is not represented directly in any part of the code. These are out of scope for now.

Getters and setters in particular have their own dual-purpose documentation strings. This is highly similar in concept to the `documentation` attribute introduced earlier, and should be made a property of the same concept.

```rust
add_attr!(
    dual_purpose_documentation <= attribute,
    build_info,
    str_concept,
    "Dual-purpose documentation that can be used in more than one situation.\n\nFor example, the same substring might be usable for both the getter and setter of a string.",
    "the dual-purpose documentation substring to be used for the implementation of this property as getters and setters in a different concept's class."
);
```

Rust groups things by modules.

```rust
define_child!(
    module,
    form,
    "Concept representing a Rust module."
);
```

Things that are grouped inside of a module will be considered a member of the module. This is true for things that have memberships in general.

```rust
define_child!(
    collection,
    form,
    "Anything that has members/sub-components."
);

add_attr!(
    member <= attribute,
    collection,
    tao,
    "Marks the value as being part of the owner. The owner should presumably be a collection of some sort.",
    "the members of this collection."
);
aa(member).mark_multi_valued_attr();
aa(member).mark_nonhereditary_attr();
```

Rust modules sometimes re-export things so that it looks like it's coming from that module.

```rust
define_child!(
    re_exports,
    attribute,
    "Marks the owner module as re-exporting the value symbol."
);
```

It seems to make sense to group a concept and its descendants inside the same module. For such modules, we'll mark the concept as the most prominent member of the module.

```rust
add_attr!(
    most_prominent_member <= attribute,
    module,
    tao,
    "The most prominent member of a Rust module. The module will take its name after this member.",
    "the most prominent member of the module. By default, the name of the module will be the same as the name of this member."
);
```

During implementation, we should be able to force a new attribute to live inside its own module. This override should take place even if the concept doesn't have any child archetypes yet, so that any concepts in downstream packages that depend on it will know where to look:

```rust
add_flag!(
    own_module <= flag,
    build_info,
    "Marks an archetype as living inside its own module, even if it doesn't have any defined child archetypes yet.",
    "residing in its own Rust module."
);
aa(own_module).mark_nonhereditary_attr();
```

Once built, structs have a certain import path:

```rust
add_attr!(
    import_path <= attribute,
    build_info,
    str_concept,
    "Describes the import path of a defined struct.",
    "the import path the Rust implementation ended up at."
);
aa(import_path).mark_nonhereditary_attr();
```

So to finish up with build information that applies to any implemented concept, everything built in Rust will be part of a crate.

```rust
define_child!(
    crate_concept,
    form,
    "Crate that a concept was built as a part of."
);
crate_concept.set_internal_name("crate");
```

We can reuse the existing generic `HasMember` relation for describing the relationship between a concept and its crate, because there is nothing special about this particular membership scenario that warrants a separate membership concept specifically for this.

Crates are versioned:

```rust
define_child!(
    version,
    attribute,
    "Version number for a versioned object."
);
```

Concepts and crates alike might also have their own implementation name:

```rust
define_child!(
    implementation_name,
    attribute,
    "Name the concept actually took on when implemented."
);
```

It is only natural for the human mind to use the name of the crate as a metonymy for the crate itself, just as humans also tend to use a filename or a file icon as a metonymy for the inode that points to the actual blocks of data on disk. How often do we stop to remind ourselves that the filename is only a symbolic handle for the actual data, or that when we're dragging a file icon from one folder to another, we're not dragging the data but only the visual representation of the data? We don't do so very often, because such details usually don't matter. It does matter here, however, so we will keep them separate and distinct.

#### Backwards compatibility

Humans have imperfect introspection capabilities into their semi-interpretable minds. As such, they often come up with initial ontologies that do not accurately reflect how they actually reason about the world behind the scenes. Accomodating change is important, because otherwise we are stuck in the confines of an imperfect past. Accomodating tradition is also important, however, because there are costs involved in accomodating change, and if one is forced to spend all their time keeping up with the flux of present times, there won't be any more time left for improving on the future.

Accomodating tradition also carries a cost, of course. We will try to automate that cost when it comes to ontologies by providing an alias for moved concepts:

```rust
add_attr!(
    alias <= attribute,
    build_info,
    str_concept,
    "Describes an aliased import path for a concept.",
    "the alternative import paths for the concept."
);
aa(alias).mark_nonhereditary_attr();
aa(alias).mark_multi_valued_attr();
```

### Implementation

Unlike with Yin, we don't actually want to implement *everything* we know, because everything we know about Yin is already implemented inside her physical body. We only want to implement the things that we learned about Yang here. This means implementing the Yang-specific modules:

```rust
let mut perspective_mod = perspective.impl_mod("Perspectives on the world.");
perspective_mod.has_extension("build_info_extension::BuildInfoExtension");
```

This also means redefining the modules for concepts that were first introduced in Yin, but which we have since created new children for:

```rust
module!(
    form,
    "All things that can be interacted with have form.",
    [
        "crate_extension::CrateExtension",
        "module_extension::ModuleExtension",
        "zamm_yin::tao::form::FormTrait"
    ]
);
module!(relation, "Relations between the forms.");
module!(flag, "Relations involving only one form.");
module!(
    attribute,
    "Relations between two forms.",
    [
        "supports_membership::SupportsMembership",
        "zamm_yin::tao::relation::attribute::AttributeTrait"
    ]
);
module!(
    has_property,
    "Meta-attributes around what attributes instances of an archetype have."
);
module!(
    archetype,
    "Types of forms, as opposed to the forms themselves.",
    [
        "create_implementation::CreateImplementation",
        "zamm_yin::tao::archetype::ArchetypeTrait",
        "zamm_yin::tao::archetype::ArchetypeFormTrait",
        "zamm_yin::tao::archetype::AttributeArchetypeFormTrait"
    ]
);
module!(
    data,
    "Data that actually exist concretely as bits on the machine, as opposed to only existing as a hypothetical, as an idea."
);
```

We should really save the build info, so that one day we will no longer need to redefine the documentation for these modules.

## Appendix

### Dependencies

This is the version of Yang used to make this build happen:

```toml
zamm_yang = "0.2.0"
```

Yang does his best to be backwards-compatible, so we should let old Yang know that this is new Yang speaking:

```rust
Crate::yin().set_version("0.2.0");
Crate::yang().set_version("0.2.0");
```

We should also let him know what our current crate name is. There is as of yet no way to let him know that this is the same crate as the `Crate::yang()` in the knowledge base, or that this crate is a newer version of himself. Unfortunately, there is no self-awareness yet, only instinct.

```rust
Crate::current().set_implementation_name("zamm_yang");
```

### Imports

Let's import the build for Yin, so that we can differentiate between where concepts are introduced:

```zamm
https://api.zamm.dev/v1/books/zamm/yin/0.2.0/yin.md
```

Additional imports not used by Yin:

```rust
use zamm_yang::add_attr;
use zamm_yang::helper::da;
```
