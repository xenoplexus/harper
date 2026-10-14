//! FussBucket fork: no trained models. Harper's Brill tagger and Burn chunker
//! were trained on non-commercial corpora, so this crate only re-exports the
//! shared types and a rule-based chunker. Tags come from the dictionary
//! (see `harper-core/src/document.rs`).

pub use harper_pos_utils::{Chunker, RuleChunker, Tagger, UPOS};

/// The model-free noun-phrase chunker.
pub fn rule_chunker() -> RuleChunker {
    RuleChunker
}
