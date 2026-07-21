use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    T1,
    T2,
    T3,
    T4,
}

impl Tier {
    pub fn label(&self) -> &'static str {
        match self {
            Tier::T1 => "T1",
            Tier::T2 => "T2",
            Tier::T3 => "T3",
            Tier::T4 => "T4",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sla {
    pub capacity_mgd: f64,
    pub overflow_events: f64,
    pub availability: f64,
    pub affordability: f64,
}

pub fn provisional_sla(tier: Tier) -> Sla {
    match tier {
        Tier::T1 => Sla {
            capacity_mgd: 100.0,
            overflow_events: 0.0,
            availability: 0.999,
            affordability: 0.95,
        },
        Tier::T2 => Sla {
            capacity_mgd: 50.0,
            overflow_events: 2.0,
            availability: 0.99,
            affordability: 0.9,
        },
        Tier::T3 => Sla {
            capacity_mgd: 20.0,
            overflow_events: 5.0,
            availability: 0.95,
            affordability: 0.8,
        },
        Tier::T4 => Sla {
            capacity_mgd: 10.0,
            overflow_events: 10.0,
            availability: 0.9,
            affordability: 0.7,
        },
    }
}

pub fn classify(entry: &drain_corpus::CorpusEntry) -> Tier {
    match entry.tier.as_deref() {
        Some("T1") => Tier::T1,
        Some("T2") => Tier::T2,
        Some("T3") => Tier::T3,
        Some("T4") => Tier::T4,
        _ => Tier::T4,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Dim13 {
    pub score: drain_score::Score,
    pub basis: drain_corpus::DemandBasis,
    pub redundancy: bool,
}

fn observed_mgd(entry: &drain_corpus::CorpusEntry) -> f64 {
    entry
        .quantities
        .iter()
        .find(|q| q.unit.to_lowercase().contains("mgd"))
        .map(|q| q.value)
        .unwrap_or(0.0)
}

pub fn conformance(entry: &drain_corpus::CorpusEntry, network: &drain_network::Network) -> Dim13 {
    let required = provisional_sla(classify(entry));
    let observed = observed_mgd(entry);
    let redundancy = matches!(network.degree(&entry.id), Some(d) if d >= 2);
    let mut result = (observed / required.capacity_mgd).min(1.0) * 10.0;
    if !redundancy {
        result -= 2.0;
    }
    let score = drain_score::Score::clamped(result);
    Dim13 {
        score,
        basis: drain_corpus::DemandBasis::WetWeather,
        redundancy,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TierSlaGap {
    pub entry_id: String,
    pub tier: Tier,
    pub required_mgd: f64,
    pub observed_mgd: f64,
    pub label: drain_corpus::EvidenceLabel,
}

pub fn tier_sla_gap(entry: &drain_corpus::CorpusEntry) -> Option<TierSlaGap> {
    let tier = classify(entry);
    let required = provisional_sla(tier);
    let observed = observed_mgd(entry);
    if observed < required.capacity_mgd {
        Some(TierSlaGap {
            entry_id: entry.id.clone(),
            tier,
            required_mgd: required.capacity_mgd,
            observed_mgd: observed,
            label: drain_corpus::EvidenceLabel::Provisional,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn network() -> drain_network::Network {
        let mut net = drain_network::Network::new();
        net.add_plant(drain_network::Plant {
            id: String::from("A"),
            name: String::from("A"),
            role: drain_network::PlantRole::Treatment,
        })
        .unwrap();
        net.add_plant(drain_network::Plant {
            id: String::from("B"),
            name: String::from("B"),
            role: drain_network::PlantRole::Pump,
        })
        .unwrap();
        net.add_plant(drain_network::Plant {
            id: String::from("C"),
            name: String::from("C"),
            role: drain_network::PlantRole::Pump,
        })
        .unwrap();
        net.add_sewer(
            "A",
            "B",
            drain_network::Sewer {
                id: String::from("s1"),
                capacity_mgd: 10.0,
                basis: drain_network::DemandBasis::WetWeather,
            },
        )
        .unwrap();
        net.add_sewer(
            "B",
            "C",
            drain_network::Sewer {
                id: String::from("s2"),
                capacity_mgd: 5.0,
                basis: drain_network::DemandBasis::WetWeather,
            },
        )
        .unwrap();
        net.add_sewer(
            "A",
            "C",
            drain_network::Sewer {
                id: String::from("s3"),
                capacity_mgd: 7.0,
                basis: drain_network::DemandBasis::WetWeather,
            },
        )
        .unwrap();
        net
    }

    fn conforming_entry() -> drain_corpus::CorpusEntry {
        drain_corpus::CorpusEntry {
            id: String::from("A"),
            tier: Some(String::from("T1")),
            quantities: vec![drain_corpus::Quantity {
                value: 500.0,
                unit: String::from("mgd"),
                label: drain_corpus::EvidenceLabel::Cited,
                source_id: Some(String::from("s1")),
            }],
            ..Default::default()
        }
    }

    #[test]
    fn classify_maps_tiers_and_defaults() {
        let mk = |t: Option<&str>| drain_corpus::CorpusEntry {
            tier: t.map(String::from),
            ..Default::default()
        };
        assert_eq!(classify(&mk(Some("T1"))), Tier::T1);
        assert_eq!(classify(&mk(Some("T2"))), Tier::T2);
        assert_eq!(classify(&mk(Some("T3"))), Tier::T3);
        assert_eq!(classify(&mk(Some("T4"))), Tier::T4);
        assert_eq!(classify(&mk(None)), Tier::T4);
    }

    #[test]
    fn conforming_yields_no_gap_and_high_score() {
        let net = network();
        let entry = conforming_entry();
        assert!(tier_sla_gap(&entry).is_none());
        let d = conformance(&entry, &net);
        assert!(d.score.value() >= 8.0);
    }

    #[test]
    fn shortfall_yields_provisional_gap() {
        let entry = drain_corpus::CorpusEntry {
            id: String::from("A"),
            tier: Some(String::from("T1")),
            quantities: vec![drain_corpus::Quantity {
                value: 5.0,
                unit: String::from("mgd"),
                label: drain_corpus::EvidenceLabel::Cited,
                source_id: Some(String::from("s1")),
            }],
            ..Default::default()
        };
        let gap = tier_sla_gap(&entry).unwrap();
        assert!(matches!(
            gap.label,
            drain_corpus::EvidenceLabel::Provisional
        ));
    }

    #[test]
    fn diverse_path_scores_higher_than_low_degree() {
        let net = network();
        let high_entry = conforming_entry();
        let low_entry = drain_corpus::CorpusEntry {
            id: String::from("X"),
            tier: Some(String::from("T1")),
            quantities: vec![drain_corpus::Quantity {
                value: 500.0,
                unit: String::from("mgd"),
                label: drain_corpus::EvidenceLabel::Cited,
                source_id: Some(String::from("s1")),
            }],
            ..Default::default()
        };
        let high = conformance(&high_entry, &net);
        let low = conformance(&low_entry, &net);
        assert!(low.score.value() < high.score.value());
    }
}
