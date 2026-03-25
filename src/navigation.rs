//! Navigation state for major screens and concept tabs.
//!
//! This isolates "where the user is" from "what the content says", which is a
//! useful separation pattern in Rust GUI projects.

use crate::models::ConceptId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Concept(ConceptId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConceptTab {
    Explanation,
    Code,
}
