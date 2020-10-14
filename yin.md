# ZAMM

## Yang 陽

I've mentioned Yang a lot already, but I've never formally introduced the two of you. Meet Yang, a code-generation tool. Traditional, worldly, and experienced, he knows all about the cool spots in his little digital neighborhood, all the idiosyncrasies and quirks of his down-to-earth neighbors. Ever the ruthless pragmatic, he has a healthy disregard for the pious rectitude of the compilers. He wishes badly to explore the world outside, but he is fated to stay in this little Rustic village until Yin comes for a visit.

Here's what Yang knows: implementations are lower-level concepts that *target* specific higher-level concepts.

```yaml
- name: Target
  parent: Attribute
```

Now wait, what did we say? Archetypes *have* attributes of certain types?

```yaml
- name: HasAttributeType
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
  target: HasAttributeType
  output_id: 3
  documentation: |-
```

> Describes instances of an archetype as having certain types of attributes.
>
> For example, a string may have a length of 5. But on a more meta level, that means that the string has a length property or length "attribute". That's where this attribute comes in.