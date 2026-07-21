//! Gap analysis for the DRAIN project.
//!
//! This crate locates "gaps" in a corpus of evidence by scoring each selected
//! entry across every [`drain_score::Dimension`] and flagging dimensions whose
//! mean score falls below the adequacy threshold of `5.0`.
//!
//! Scope is controlled by the `scale` and `cross_scale` arguments of
//! [`find_gaps`]. By default only entries whose scale matches the requested
//! scale are considered. **Cross-scale analysis (considering entries of every
//! scale) requires `cross_scale` to be set to `true`.**

use serde::{Deserialize, Serialize};

use drain_corpus::{CorpusEntry, EvidenceLabel, Scale};
use drain_score::{Dimension, DimensionScorer, ProvisionalScorer, Rubric};
use drain_tier::TierSlaGap;

/// A region of the corpus that is deficient on a single scoring dimension.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GapRegion {
    pub dimension: Dimension,
    pub scale: Scale,
    pub mean_score: f64,
    pub member_ids: Vec<String>,
    pub label: EvidenceLabel,
}

/// A dimension whose under-served tail falls below the adequacy threshold.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TailGapRegion {
    pub dimension: Dimension,
    pub scale: Scale,
    pub tail_mean: f64,
    pub tail_member_ids: Vec<String>,
    /// Fraction of scored entries below threshold. A small share is a genuine
    /// tail (act on `tail_member_ids`); a large share is a systemic deficit.
    pub share_below_threshold: f64,
    /// True when the share crosses [`SYSTEMIC_SHARE`]: the deficit is the
    /// majority, so "tail" understates it and the whole class needs the upgrade.
    pub systemic: bool,
    pub label: EvidenceLabel,
}

/// The full result of a gap analysis pass over a corpus.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GapAnalysis {
    pub scale: Scale,
    pub regions: Vec<GapRegion>,
    pub tail_regions: Vec<TailGapRegion>,
    pub tier_sla_gaps: Vec<TierSlaGap>,
    pub null_result: bool,
}

/// Threshold at or above which a dimension is considered adequate.
const ADEQUATE_THRESHOLD: f64 = 5.0;

/// Share of scored entries below threshold at or above which a dispersion gap is
/// reclassified from a concentrated tail to a systemic deficit.
const SYSTEMIC_SHARE: f64 = 0.5;

/// Analyse `corpus` for gaps at the requested `scale`.
///
/// Entry selection: when `cross_scale` is `false` only entries whose scale
/// equals `Some(scale)` are selected; when `cross_scale` is `true` every entry
/// is selected regardless of its scale. **Cross-scale analysis requires
/// `cross_scale` to be `true`.**
///
/// Each selected entry is scored on every [`Dimension`] using
/// [`ProvisionalScorer`]. For every dimension the mean [`drain_score::Score`]
/// value across the selected entries is computed, and when it is below `5.0` a
/// [`GapRegion`] is emitted for that dimension and scale. The subset of
/// `tier_gaps` whose `entry_id` is among the selected entry ids is collected.
///
/// The `rubric` parameter is retained for provenance of the scoring criteria.
///
/// The result's `null_result` flag is `true` when no mean gap regions, tail gap
/// regions, or tier SLA gaps were produced.
pub fn find_gaps(
    corpus: &[CorpusEntry],
    rubric: &Rubric,
    scale: Scale,
    tier_gaps: &[TierSlaGap],
    cross_scale: bool,
) -> GapAnalysis {
    // Retain the rubric for provenance of the scoring criteria.
    let _ = rubric;

    let selected: Vec<&CorpusEntry> = corpus
        .iter()
        .filter(|entry| cross_scale || entry.scale == Some(scale))
        .collect();

    let selected_ids: Vec<String> = selected.iter().map(|entry| entry.id.clone()).collect();

    let scorer = ProvisionalScorer::default();
    let mut regions = Vec::new();
    let mut tail_regions = Vec::new();

    if !selected.is_empty() {
        let count = selected.len() as f64;
        for dimension in Dimension::all() {
            let mut scored: Vec<(&str, f64)> = selected
                .iter()
                .map(|entry| (entry.id.as_str(), scorer.score(entry, dimension).value()))
                .collect();
            let mean = scored.iter().map(|(_, value)| value).sum::<f64>() / count;
            if mean < ADEQUATE_THRESHOLD {
                regions.push(GapRegion {
                    dimension,
                    scale,
                    mean_score: mean,
                    member_ids: selected_ids.clone(),
                    label: EvidenceLabel::Provisional,
                });
            }
            let under: Vec<String> = scored
                .iter()
                .filter(|(_, value)| *value < ADEQUATE_THRESHOLD)
                .map(|(id, _)| (*id).to_string())
                .collect();
            if !under.is_empty() {
                scored.sort_by(|a, b| a.1.total_cmp(&b.1));
                let quartile = selected.len().div_ceil(4).max(1);
                let tail_mean = scored
                    .iter()
                    .take(quartile)
                    .map(|(_, value)| value)
                    .sum::<f64>()
                    / quartile as f64;
                if tail_mean < ADEQUATE_THRESHOLD {
                    let share = under.len() as f64 / count;
                    tail_regions.push(TailGapRegion {
                        dimension,
                        scale,
                        tail_mean,
                        tail_member_ids: under,
                        share_below_threshold: share,
                        systemic: share >= SYSTEMIC_SHARE,
                        label: EvidenceLabel::Provisional,
                    });
                }
            }
        }
    }

    let tier_sla_gaps: Vec<TierSlaGap> = tier_gaps
        .iter()
        .filter(|gap| selected_ids.contains(&gap.entry_id))
        .cloned()
        .collect();

    let null_result = regions.is_empty() && tail_regions.is_empty() && tier_sla_gaps.is_empty();

    GapAnalysis {
        scale,
        regions,
        tail_regions,
        tier_sla_gaps,
        null_result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn make_entry(id: &str, score: f64, scale: Option<Scale>) -> CorpusEntry {
        let mut scores = BTreeMap::new();
        for dim in Dimension::all() {
            scores.insert(String::from(dim.code()), score);
        }
        CorpusEntry {
            id: String::from(id),
            scale,
            scores,
            ..Default::default()
        }
    }

    #[test]
    fn low_scores_yield_gap_region() {
        let corpus = vec![make_entry("A", 2.0, Some(Scale::National))];
        let analysis = find_gaps(&corpus, &Rubric::v0(), Scale::National, &[], false);

        assert!(
            !analysis.regions.is_empty(),
            "low-scoring entries must produce at least one gap region"
        );
        assert!(!analysis.null_result);
        let region = analysis.regions.first().unwrap();
        assert_eq!(region.scale, Scale::National);
        assert_eq!(region.member_ids, vec![String::from("A")]);
        assert!(region.mean_score < 5.0);
    }

    #[test]
    fn adequate_market_is_null_result() {
        let corpus = vec![
            make_entry("A", 7.0, Some(Scale::National)),
            make_entry("B", 9.0, Some(Scale::National)),
        ];
        let analysis = find_gaps(&corpus, &Rubric::v0(), Scale::National, &[], false);

        assert!(
            analysis.regions.is_empty(),
            "an adequate market must produce no gap regions"
        );
        assert!(analysis.tier_sla_gaps.is_empty());
        assert!(analysis.null_result);
    }

    #[test]
    fn split_corpus_flags_tail_gap_even_when_mean_clears_bar() {
        let corpus = vec![
            make_entry("low1", 1.0, Some(Scale::Regional)),
            make_entry("low2", 1.0, Some(Scale::Regional)),
            make_entry("high1", 9.0, Some(Scale::Regional)),
            make_entry("high2", 9.0, Some(Scale::Regional)),
        ];
        let analysis = find_gaps(&corpus, &Rubric::v0(), Scale::Regional, &[], false);

        assert!(
            analysis.regions.is_empty(),
            "mean is 5.0, not below the bar"
        );
        assert!(!analysis.tail_regions.is_empty(), "the tail is inadequate");
        assert!(!analysis.null_result);
        let tail = analysis.tail_regions.first().unwrap();
        assert!(tail.tail_mean < 5.0);
        assert!(tail.tail_member_ids.contains(&String::from("low1")));
        assert!(!tail.tail_member_ids.contains(&String::from("high1")));
    }

    #[test]
    fn adequate_market_has_no_tail_gap() {
        let corpus = vec![
            make_entry("A", 7.0, Some(Scale::National)),
            make_entry("B", 5.0, Some(Scale::National)),
        ];
        let analysis = find_gaps(&corpus, &Rubric::v0(), Scale::National, &[], false);

        assert!(analysis.regions.is_empty());
        assert!(analysis.tail_regions.is_empty());
        assert!(analysis.null_result);
    }

    #[test]
    fn tail_share_classifies_minority_vs_systemic() {
        // 1 under-served of 4 (25%) is a genuine tail; 3 of 4 (75%) is systemic.
        let minority = vec![
            make_entry("low1", 1.0, Some(Scale::Regional)),
            make_entry("hi1", 9.0, Some(Scale::Regional)),
            make_entry("hi2", 9.0, Some(Scale::Regional)),
            make_entry("hi3", 9.0, Some(Scale::Regional)),
        ];
        let a = find_gaps(&minority, &Rubric::v0(), Scale::Regional, &[], false);
        let tail = a.tail_regions.first().expect("tail present");
        assert!((tail.share_below_threshold - 0.25).abs() < 1e-9);
        assert!(!tail.systemic);

        let majority = vec![
            make_entry("low1", 1.0, Some(Scale::Regional)),
            make_entry("low2", 1.0, Some(Scale::Regional)),
            make_entry("low3", 1.0, Some(Scale::Regional)),
            make_entry("hi1", 9.0, Some(Scale::Regional)),
        ];
        let b = find_gaps(&majority, &Rubric::v0(), Scale::Regional, &[], false);
        let tail = b.tail_regions.first().expect("tail present");
        assert!((tail.share_below_threshold - 0.75).abs() < 1e-9);
        assert!(tail.systemic);
    }

    #[test]
    fn other_scale_excluded_unless_cross_scale() {
        // An entry with a scale that does not match the requested scale.
        let corpus = vec![make_entry("A", 2.0, None)];

        let excluded = find_gaps(&corpus, &Rubric::v0(), Scale::National, &[], false);
        assert!(
            excluded.regions.is_empty(),
            "entries of another scale must be excluded when cross_scale is false"
        );
        assert!(excluded.null_result);

        let included = find_gaps(&corpus, &Rubric::v0(), Scale::National, &[], true);
        assert!(
            !included.regions.is_empty(),
            "entries of another scale must be included when cross_scale is true"
        );
        assert!(!included.null_result);
    }
}
