use crate::tao::form::{Crate, Module};
use crate::tao::perspective::{BuildInfo, KnowledgeGraphNode, Perspective};
use crate::tao::relation::attribute::{
    ConceptId, Documentation, DualPurposeDocumentation, DummyValue, HasMember, ImplementationName,
    ImportPath, MostProminentMember, ReExports, RustPrimitive, Target, UnboxedRepresentation,
    Version,
};
use crate::tao::relation::flag::{
    ArchetypeAnalogue, AttributeAnalogue, DataAnalogue, Imported, NewlyDefined, OwnModule,
    RootAnalogue, RootArchetypeAnalogue,
};
use crate::tao::Implement;
use zamm_yin::graph::{Graph, InjectionGraph};
use zamm_yin::initialize_type;
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::form::data::{Number, StringConcept};
use zamm_yin::tao::relation::attribute::has_property::{HasAttribute, HasFlag};
use zamm_yin::tao::relation::attribute::{Inherits, OwnerArchetype, ValueArchetype};
use zamm_yin::tao::Tao;

/// The maximum concept ID inside the types distributed by Yin itself. App-
/// specific type concepts should continue their numbering on top of this.
pub const YIN_MAX_ID: usize = zamm_yin::tao::YIN_MAX_ID + 27;

/// Adds all concepts to knowledge graph.
pub fn initialize_types() {
    let mut ig = InjectionGraph::new();
    #[rustfmt::skip]
    initialize_type!(
        ig,
        (
            Implement,
            Target,
            ConceptId,
            Documentation,
            RustPrimitive,
            UnboxedRepresentation,
            Perspective,
            KnowledgeGraphNode,
            NewlyDefined,
            Imported,
            AttributeAnalogue,
            RootAnalogue,
            RootArchetypeAnalogue,
            ArchetypeAnalogue,
            DataAnalogue,
            BuildInfo,
            DualPurposeDocumentation,
            Module,
            HasMember,
            ReExports,
            MostProminentMember,
            OwnModule,
            ImportPath,
            DummyValue,
            Crate,
            Version,
            ImplementationName
        )
    );
    ig.add_edge(Implement::TYPE_ID, HasAttribute::TYPE_ID, Target::TYPE_ID);
    ig.add_edge(
        Implement::TYPE_ID,
        HasAttribute::TYPE_ID,
        ConceptId::TYPE_ID,
    );
    ig.add_edge(
        Implement::TYPE_ID,
        HasAttribute::TYPE_ID,
        Documentation::TYPE_ID,
    );
    ig.add_edge(Target::TYPE_ID, OwnerArchetype::TYPE_ID, Implement::TYPE_ID);
    ig.add_edge(Target::TYPE_ID, ValueArchetype::TYPE_ID, Tao::TYPE_ID);
    ig.add_edge(
        ConceptId::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        Implement::TYPE_ID,
    );
    ig.add_edge(ConceptId::TYPE_ID, ValueArchetype::TYPE_ID, Number::TYPE_ID);
    ig.add_edge(
        Documentation::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        Implement::TYPE_ID,
    );
    ig.add_edge(
        Documentation::TYPE_ID,
        ValueArchetype::TYPE_ID,
        StringConcept::TYPE_ID,
    );
    ig.add_edge(
        KnowledgeGraphNode::TYPE_ID,
        HasFlag::TYPE_ID,
        NewlyDefined::TYPE_ID,
    );
    ig.add_edge(
        KnowledgeGraphNode::TYPE_ID,
        HasFlag::TYPE_ID,
        Imported::TYPE_ID,
    );
    ig.add_edge(
        KnowledgeGraphNode::TYPE_ID,
        HasFlag::TYPE_ID,
        AttributeAnalogue::TYPE_ID,
    );
    ig.add_edge(
        KnowledgeGraphNode::TYPE_ID,
        HasFlag::TYPE_ID,
        RootAnalogue::TYPE_ID,
    );
    ig.add_edge(
        KnowledgeGraphNode::TYPE_ID,
        HasFlag::TYPE_ID,
        RootArchetypeAnalogue::TYPE_ID,
    );
    ig.add_edge(
        KnowledgeGraphNode::TYPE_ID,
        HasFlag::TYPE_ID,
        ArchetypeAnalogue::TYPE_ID,
    );
    ig.add_edge(
        KnowledgeGraphNode::TYPE_ID,
        HasFlag::TYPE_ID,
        DataAnalogue::TYPE_ID,
    );
    ig.add_edge(
        NewlyDefined::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        KnowledgeGraphNode::TYPE_ID,
    );
    ig.add_edge(
        Imported::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        KnowledgeGraphNode::TYPE_ID,
    );
    ig.add_edge(
        AttributeAnalogue::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        KnowledgeGraphNode::TYPE_ID,
    );
    ig.add_edge(
        RootAnalogue::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        KnowledgeGraphNode::TYPE_ID,
    );
    ig.add_edge(
        RootArchetypeAnalogue::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        KnowledgeGraphNode::TYPE_ID,
    );
    ig.add_edge(
        ArchetypeAnalogue::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        KnowledgeGraphNode::TYPE_ID,
    );
    ig.add_edge(
        DataAnalogue::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        KnowledgeGraphNode::TYPE_ID,
    );
    ig.add_edge(BuildInfo::TYPE_ID, HasFlag::TYPE_ID, OwnModule::TYPE_ID);
    ig.add_edge(
        BuildInfo::TYPE_ID,
        HasAttribute::TYPE_ID,
        DualPurposeDocumentation::TYPE_ID,
    );
    ig.add_edge(
        BuildInfo::TYPE_ID,
        HasAttribute::TYPE_ID,
        ImportPath::TYPE_ID,
    );
    ig.add_edge(
        DualPurposeDocumentation::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        BuildInfo::TYPE_ID,
    );
    ig.add_edge(
        DualPurposeDocumentation::TYPE_ID,
        ValueArchetype::TYPE_ID,
        StringConcept::TYPE_ID,
    );
    ig.add_edge(
        Module::TYPE_ID,
        HasAttribute::TYPE_ID,
        MostProminentMember::TYPE_ID,
    );
    ig.add_edge(
        MostProminentMember::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        Module::TYPE_ID,
    );
    ig.add_edge(
        MostProminentMember::TYPE_ID,
        ValueArchetype::TYPE_ID,
        Tao::TYPE_ID,
    );
    ig.add_edge(
        OwnModule::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        BuildInfo::TYPE_ID,
    );
    ig.add_edge(
        ImportPath::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        BuildInfo::TYPE_ID,
    );
    ig.add_edge(
        ImportPath::TYPE_ID,
        ValueArchetype::TYPE_ID,
        StringConcept::TYPE_ID,
    );
}
