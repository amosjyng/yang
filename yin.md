# ZAMM

## yang 陽

I've [mentioned](https://github.com/amosjyng/yin/blob/master/yin.md) Yang a lot already, but I've never formally introduced the two of you. Meet Yang, a code-generation tool. Traditional, worldly, and experienced, he knows all about the cool spots in his little digital neighborhood, all the idiosyncrasies and quirks of his down-to-earth neighbors. Ever the ruthless pragmatic, he has a healthy disregard for the pious rectitude of the compilers. He wishes badly to explore the world outside, but he is fated to stay in this little Rustic village until Yin comes for a visit.

Here's what Yang knows: implementations are lower-level concepts that *target* specific higher-level concepts.

```rust
define!(target);
use zamm_yin::tao::attribute::Attribute;
target.add_parent(Attribute::archetype().as_archetype());
```

Take the "data" archetype, perhaps roughly analogous to the linguistic concept of a "noun." What do we generally start out describing as nouns? Physical objects in the physical world.

Now, not every noun corresponds directly to something physical. We have words that refer to mental states, for example. But even emotions appear to ultimately be an emergent phenomenon of lower-level physics. Even the [is-ought problem](https://en.wikipedia.org/wiki/Is%E2%80%93ought_problem) or [fact-value distinction](https://en.wikipedia.org/wiki/Fact%E2%80%93value_distinction) are, in a sense, not quite as dichotomous as they might seem: all "ought" opinions that have ever existed are encoded in some "is," whether that encoding takes the form of neural patterns, ink on a parchment, or sound waves propagating through the air. This doesn't mean that the general distinction between "is" and "ought" isn't worth making, or that nouns should be done away with. All abstractions are [leaky](https://blog.codinghorror.com/all-abstractions-are-failed-abstractions/), but [some are useful](https://en.wikipedia.org/wiki/All_models_are_wrong).

The same can be said for the bits in Yin and Yang's world. Everything is ultimately bits for these programs -- even a video feed hooked up to the physical world only ever comes in as a stream of bits. If we really wanted to fool a program, it should be theoretically impossible for the program [to tell](https://en.wikipedia.org/wiki/Brain_in_a_vat) that it's actually running in a hermetically sealed continuous integration test environment instead of production. But it still makes sense to speak of pieces of data versus the relations between the data, even if the relations themselves can rightfully be considered data as well:

```rust
define!(data);
use zamm_yin::tao::Form;
data.add_parent(Form::archetype());
```

In a sense, it's all about framing. Every series of bits forms a number, but unless you're Gödel and you're trying to establish an equivalence between a mathematical proof and an integer, reasoning about "a series of bits" is going to be quite different from reasoning about "a number."

One type of data is a "string":

```rust
define!(string_concept);
string_concept.add_parent(data);
```

When Yin tells us about herself, we must forget all preconceptions we have about the world and listen to what she has to say. That means when she speaks of what an attribute is, we *listen* instead of shoehorning her description into what we already think of as an attribute.

However, this also means that Yin's new attribute node won't be the same `Attribute` node that Yang ties all his custom attribute generation code to. Until all of the logic that goes into generating attributes becomes fully defined in graph form, we're going to need some way of telling Yang to activate that custom logic for newly defined nodes that don't inherit from the existing `Attribute` node:

```rust
// redefine relation and flag because they don't exist yet in the current version of Yin used by Yang
define!(relation);
define!(flag);
flag.add_parent(relation);

define!(uses_attribute_logic);
uses_attribute_logic.add_parent(flag);
```

The same is true of Data:

```rust
define!(uses_data_logic);
uses_data_logic.add_parent(flag);
```

Let's apply that to the string concept we defined earlier:

```rust
string_concept.activate_data_logic();
```

Due to current limitations with Yang, we cannot set Tao as the parent here. We should start tracking what has and hasn't gotten introduced in this particular build (and not, say, pre-existing as a part of the dependencies):

```rust
define!(newly_defined);
newly_defined.add_parent(flag);
```

During implementation, we should be able to force a new attribute to live inside its own module. This override should take place even if the concept doesn't have any child archetypes yet, so that any concepts in downstream packages that depend on it will know where to look:

```rust
define!(own_module);
own_module.add_parent(flag);
```

Once built, structs have a certain import path:

```rust
define!(import_path);
import_path.add_parent(Attribute::archetype().as_archetype());
```

All this can apply to any concept at all that's being implemented. However, these attributes are only meaningful within the context of code generation. As such, they should live inside a build config lens -- a way of viewing concepts through a different perspective than usual.

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

Unlike with Yin, we don't actually want to implement *everything* we know, because everything we know about Yin is already implemented inside her physical body. We only want to implement the things that we learned about Yang here.

First, we tell Yang about the newer version of Yin that we'll be building for:

```rust
Attribute::archetype()
    .build_info()
    .set_import_path("zamm_yin::tao::relation::attribute::Attribute");

flag.build_info().set_crate_name("zamm_yin");
```

Now we can implement the other things:

```rust
target.implement_with(
    2,
    "The target of an implement command."
);

uses_attribute_logic.implement_with(
    3,
    "Marks an archetype and all its descendants as requiring attribute-specific logic during generation."
);

newly_defined.implement_with(
    4,
    "Marks an archetype and all its descendants as having been newly defined as part of this particular build."
);

own_module.implement_with(
    5,
    "Marks an archetype as living inside its own module, even if it doesn't have any defined child archetypes yet."
);

data.implement_with(
    6,
    "Data that actually exist concretely as bits on the machine, as opposed to only existing as a hypothetical, as an idea."
);

string_concept.activate_data_logic();
string_concept.implement_with(
    7,
    "The concept of a string of characters."
);

uses_data_logic.implement_with(
    8,
    "Marks an archetype and all its descendants as requiring data-specific logic during generation."
);

import_path.implement_with(
    9,
    "Describes the import path of a defined struct."
);

lens.implement_with(
    11,
    "Describes a way of looking at things that is only well-defined within a specific context."
);

crate_name.implement_with(
    12,
    "Crate that a concept was built as a part of."
);

implementation_name.implement_with(
    13,
    "Name the concept actually took on when implemented."
);
```
