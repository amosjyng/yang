use crate::tao::form::{BuildInfo, Crate, Module};
use crate::tao::perspective::{KnowledgeGraph, Perspective};
#[rustfmt::skip]
use crate::tao::relation::attribute::{ConceptId, Documentation, HasMember, ImplementationName, ImportPath, MostProminentMember, ReExports, RustPrimitive, Target, Version};
#[rustfmt::skip]
use crate::tao::relation::flag::{NewlyDefined, Nonhereditary, OwnModule, UsesAttributeLogic, UsesDataLogic, UsesRootNodeLogic};
use crate::tao::Implement;
use zamm_yin::graph::{Graph, InjectionGraph};
use zamm_yin::initialize_type;
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::relation::attribute::Inherits;

/// The maximum concept ID inside the types distributed by Yin itself. App-
/// specific type concepts should continue their numbering on top of this.
pub const YIN_MAX_ID: usize = 21;

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
            NewlyDefined,
            Module,
            HasMember,
            ReExports,
            MostProminentMember,
            OwnModule,
            RustPrimitive,
            UsesAttributeLogic,
            UsesDataLogic,
            UsesRootNodeLogic,
            Nonhereditary,
            ImportPath,
            BuildInfo,
            Perspective,
            KnowledgeGraph,
            Crate,
            Version,
            ImplementationName
        )
    );
}
