//! Shared data models used across the crate.
//!
//! Keeping models in one file helps beginners see how enums/structs become
//! common language between modules like navigation, content, and UI.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConceptId {
    WhatIsRust,
    VariablesMutability,
    Functions,
    Types,
    ControlFlow,
    Ownership,
    BorrowingReferences,
    Slices,
    Structs,
    Enums,
    PatternMatching,
    OptionResult,
    VectorsStrings,
    ErrorHandling,
    ModulesOrganization,
    TraitsGenerics,
    LifetimesIntro,
    CargoBasics,
}

#[derive(Debug, Clone)]
pub struct Concept {
    pub id: ConceptId,
    pub title: &'static str,
    pub why_it_matters: &'static str,
    pub explanation: &'static str,
    pub code: &'static str,
    pub code_explanation: &'static str,
    pub beginner_mistake: &'static str,
    pub recap: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualMode {
    Idle,
    MutabilityEdit,
    BorrowShared,
    BorrowExclusive,
    MatchHappyPath,
    MatchFallback,
    EnumStateIdle,
    EnumStateRunning,
    StructFocus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipMode {
    Move,
    ImmutableBorrow,
    MutableBorrow,
}
