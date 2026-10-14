use super::Chunker;
use crate::UPOS;

/// A model-free noun-phrase chunker for FussBucket.
///
/// A noun phrase is a run of modifiers (determiners, numerals, adjectives)
/// followed by one or more heads (nouns, proper nouns, pronouns). It follows
/// the same heuristic as Harper's `NominalPhrase` pattern, applied to tags.
#[derive(Debug, Default, Clone, Copy)]
pub struct RuleChunker;

fn is_head(tag: Option<UPOS>) -> bool {
    matches!(tag, Some(UPOS::NOUN | UPOS::PROPN | UPOS::PRON))
}

fn is_modifier(tag: Option<UPOS>) -> bool {
    matches!(tag, Some(UPOS::DET | UPOS::NUM | UPOS::ADJ))
}

impl Chunker for RuleChunker {
    fn chunk_sentence(&self, _sentence: &[String], tags: &[Option<UPOS>]) -> Vec<bool> {
        let mut flags = vec![false; tags.len()];
        let mut i = 0;
        while i < tags.len() {
            let start = i;
            while i < tags.len() && is_modifier(tags[i]) {
                i += 1;
            }
            let heads_start = i;
            while i < tags.len() && is_head(tags[i]) {
                i += 1;
            }
            if i > heads_start {
                flags[start..i].iter_mut().for_each(|f| *f = true);
            } else if i == start {
                i += 1;
            }
        }
        flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_modifiers_and_heads() {
        let tags = [
            Some(UPOS::DET),
            Some(UPOS::ADJ),
            Some(UPOS::NOUN),
            Some(UPOS::VERB),
            Some(UPOS::PRON),
        ];
        let flags = RuleChunker.chunk_sentence(&[], &tags);
        assert_eq!(flags, vec![true, true, true, false, true]);
    }

    #[test]
    fn modifiers_without_head_are_not_a_phrase() {
        let tags = [Some(UPOS::DET), Some(UPOS::ADJ), Some(UPOS::VERB)];
        let flags = RuleChunker.chunk_sentence(&[], &tags);
        assert_eq!(flags, vec![false, false, false]);
    }
}
