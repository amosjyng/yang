# ZAMM

## yang 陽

I've [mentioned](https://github.com/amosjyng/yin/blob/master/yin.md) Yang a lot already, but I've never formally introduced the two of you. Meet Yang, a code-generation tool. Traditional, worldly, and experienced, he knows all about the cool spots in his little digital neighborhood, all the idiosyncrasies and quirks of his down-to-earth neighbors. Ever the ruthless pragmatic, he has a healthy disregard for the pious rectitude of the compilers. He wishes badly to explore the world outside, but he is fated to stay in this little Rustic village until Yin comes for a visit.

Here's what Yang knows: implementations are lower-level concepts that *target* specific higher-level concepts.

```yaml
- name: Target
  parent: Attribute
```

Take the "data" archetype, perhaps roughly analogous to the linguistic concept of a "noun." What do we generally start out describing as nouns? Physical objects in the physical world.

Now, not every noun corresponds directly to something physical. We have words that refer to mental states, for example. But even emotions appear to ultimately be an emergent phenomenon of lower-level physics. Even the [is-ought problem](https://en.wikipedia.org/wiki/Is%E2%80%93ought_problem) or [fact-value distinction](https://en.wikipedia.org/wiki/Fact%E2%80%93value_distinction) are, in a sense, not quite as dichotomous as they might seem: all "ought" opinions that have ever existed are encoded in some "is," whether that encoding takes the form of neural patterns, ink on a parchment, or sound waves propagating through the air. This doesn't mean that the general distinction between "is" and "ought" isn't worth making, or that nouns should be done away with. All abstractions are [leaky](https://blog.codinghorror.com/all-abstractions-are-failed-abstractions/), but [some are useful](https://en.wikipedia.org/wiki/All_models_are_wrong).

The same can be said for the bits in Yin and Yang's world. Everything is ultimately bits for these programs -- even a video feed hooked up to the physical world only ever comes in as a stream of bits. If we really wanted to fool a program, it should be theoretically impossible for the program [to tell](https://en.wikipedia.org/wiki/Brain_in_a_vat) that it's actually running in a hermetically sealed continuous integration test environment instead of production. But it still makes sense to speak of pieces of data versus the relations between the data, even if the relations themselves can rightfully be considered data as well:

```yaml
- name: Data
  parent: Tao
```

In a sense, it's all about framing. Every series of bits forms a number, but unless you're Gödel and you're trying to establish an equivalence between a mathematical proof and an integer, reasoning about "a series of bits" is going to be quite different from reasoning about "a number."

One type of data is a "string":

```yaml
- name: StringConcept
  parent: Data
```

When Yin tells us about herself, we must forget all preconceptions we have about the world and listen to what she has to say. That means when she speaks of what an attribute is, we *listen* instead of shoehorning her description into what we already think of as an attribute.

However, this also means that Yin's new attribute node won't be the same `Attribute` node that Yang ties all his custom attribute generation code to. Until all of the logic that goes into generating attributes becomes fully defined in graph form, we're going to need some way of telling Yang to activate that custom logic for newly defined nodes that don't inherit from the existing `Attribute` node:

```yaml
- name: Flag
  parent: Tao
- name: UsesAttributeLogic
  parent: Tao
```

Due to current limitations with Yang, we cannot set Tao as the parent here. We should start tracking what has and hasn't gotten introduced in this particular build (and not, say, pre-existing as a part of the dependencies):

```yaml
- name: NewlyDefined
  parent: Tao
```

During implementation, we should be able to force a new attribute to live inside its own module. This override should take place even if the concept doesn't have any child archetypes yet, so that any concepts in downstream packages that depend on it will know where to look:

```yaml
- name: OwnModule
  parent: Tao
```



### Implementation

Unlike with Yin, we don't actually want to implement *everything* we know, because everything we know about Yin is already implemented inside her physical body. We only want to implement the things that we learned about Yang here:

```yaml
- parent: Implement
  target: Target
  output_id: 2
  documentation: |-
```

> The target of an implement command.

```yaml
- parent: Implement
  target: Flag
  output_id: 3
  documentation: |-
```

> Unary relations.

```yaml
- parent: Implement
  target: UsesAttributeLogic
  output_id: 4
  documentation: |-
```

> Marks an archetype and all its descendants as requiring attribute-specific logic during generation.

```yaml
- parent: Implement
  target: NewlyDefined
  output_id: 5
  documentation: |-
```

> Marks an archetype and all its descendants as having been newly defined as part of this particular build.

```yaml
- parent: Implement
  target: OwnModule
  output_id: 6
  documentation: |-
```

> Marks an archetype as living inside its own module, even if it doesn't have any defined child archetypes yet.

```yaml
- parent: Implement
  target: Data
  output_id: 7
  documentation: |-
```

> Data that actually exist concretely as bits on the machine, as opposed to only existing as a hypothetical, as an idea.

```yaml
- parent: Implement
  target: StringConcept
  output_id: 8
  documentation: |-
```

> The concept of a string of characters.