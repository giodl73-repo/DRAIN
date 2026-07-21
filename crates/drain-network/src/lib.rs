//! drain-network — wastewater and sewer network graph kernel for the DRAIN project.
//!
//! This crate models a wastewater/sewer collection system as an undirected graph
//! whose nodes are [`Plant`]s (treatment, pumping, or outfall facilities) and whose
//! edges are [`Sewer`] conveyances connecting them.

use std::collections::{HashMap, HashSet};

use petgraph::algo::{astar, has_path_connecting};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::NodeFiltered;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The functional role a [`Plant`] plays within the network.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlantRole {
    /// A wastewater treatment facility.
    Treatment,
    /// A pumping / lift station.
    Pump,
    /// A discharge point to a receiving water body.
    Outfall,
}

/// The flow-demand basis a [`Sewer`] is sized against.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DemandBasis {
    /// Capacity reflects wet-weather (storm-influenced) flows.
    WetWeather,
    /// Capacity reflects dry-weather (base sanitary) flows.
    DryWeather,
}

/// A node in the network: a treatment plant, pump station, or outfall.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plant {
    /// Stable unique identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Functional role of the plant.
    pub role: PlantRole,
}

/// An edge in the network: a sewer conveyance between two plants.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sewer {
    /// Stable unique identifier.
    pub id: String,
    /// Hydraulic capacity in millions of gallons per day.
    pub capacity_mgd: f64,
    /// The demand basis the capacity was derived from.
    pub basis: DemandBasis,
}

/// Errors that can occur while building or mutating a [`Network`].
#[derive(Debug, Error)]
pub enum NetworkError {
    /// A plant with the given id already exists in the network.
    #[error("duplicate plant id: {0}")]
    DuplicatePlant(String),
    /// A referenced plant id does not exist in the network.
    #[error("unknown plant id: {0}")]
    UnknownPlant(String),
    /// A sewer was supplied with a non-positive capacity.
    #[error("non-positive sewer capacity: {0} mgd")]
    NonPositiveCapacity(f64),
}

/// An undirected wastewater/sewer network of [`Plant`] nodes and [`Sewer`] edges.
pub struct Network {
    graph: UnGraph<Plant, Sewer>,
    index: HashMap<String, NodeIndex>,
}

impl Network {
    /// Create a new, empty network.
    pub fn new() -> Self {
        Network {
            graph: UnGraph::new_undirected(),
            index: HashMap::new(),
        }
    }

    /// Add a plant to the network.
    ///
    /// Returns [`NetworkError::DuplicatePlant`] if a plant with the same id
    /// already exists.
    pub fn add_plant(&mut self, plant: Plant) -> Result<(), NetworkError> {
        if self.index.contains_key(&plant.id) {
            return Err(NetworkError::DuplicatePlant(plant.id));
        }
        let id = plant.id.clone();
        let idx = self.graph.add_node(plant);
        self.index.insert(id, idx);
        Ok(())
    }

    /// Add a sewer connecting two existing plants.
    ///
    /// Returns [`NetworkError::NonPositiveCapacity`] if `capacity_mgd <= 0.0`,
    /// or [`NetworkError::UnknownPlant`] if either endpoint id is not present.
    pub fn add_sewer(
        &mut self,
        from_id: &str,
        to_id: &str,
        sewer: Sewer,
    ) -> Result<(), NetworkError> {
        if sewer.capacity_mgd <= 0.0 {
            return Err(NetworkError::NonPositiveCapacity(sewer.capacity_mgd));
        }
        let from = *self
            .index
            .get(from_id)
            .ok_or_else(|| NetworkError::UnknownPlant(from_id.to_string()))?;
        let to = *self
            .index
            .get(to_id)
            .ok_or_else(|| NetworkError::UnknownPlant(to_id.to_string()))?;
        self.graph.add_edge(from, to, sewer);
        Ok(())
    }

    /// Number of plants in the network.
    pub fn plant_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Number of sewers in the network.
    pub fn sewer_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Number of sewers incident to the plant, or `None` if it does not exist.
    pub fn degree(&self, id: &str) -> Option<usize> {
        self.index.get(id).map(|idx| self.graph.edges(*idx).count())
    }

    /// Returns `true` when a path of sewers connects plants `a` and `b`.
    pub fn is_connected(&self, a: &str, b: &str) -> bool {
        match (self.index.get(a), self.index.get(b)) {
            (Some(&ia), Some(&ib)) => has_path_connecting(&self.graph, ia, ib, None),
            _ => false,
        }
    }

    /// Returns `true` when at least two node-disjoint paths exist between `a`
    /// and `b`.
    ///
    /// This is computed by finding one path, then checking whether a path still
    /// exists once the intermediate plants of that path are removed.
    pub fn has_diverse_path(&self, a: &str, b: &str) -> bool {
        let (ia, ib) = match (self.index.get(a), self.index.get(b)) {
            (Some(&ia), Some(&ib)) => (ia, ib),
            _ => return false,
        };

        let path = match astar(&self.graph, ia, |n| n == ib, |_| 1, |_| 0) {
            Some((_, p)) => p,
            None => return false,
        };

        let exclude: HashSet<NodeIndex> = path
            .iter()
            .skip(1)
            .take(path.len().saturating_sub(2))
            .copied()
            .collect();

        let filtered = NodeFiltered::from_fn(&self.graph, |n| !exclude.contains(&n));
        has_path_connecting(&filtered, ia, ib, None)
    }

    /// Sum of `capacity_mgd` across all sewers incident to the plant.
    ///
    /// Returns `0.0` for an unknown plant id.
    pub fn incident_capacity_mgd(&self, id: &str) -> f64 {
        let idx = match self.index.get(id) {
            Some(&idx) => idx,
            None => return 0.0,
        };
        let mut total = 0.0;
        for edge in self.graph.edge_indices() {
            let (source, target) = match self.graph.edge_endpoints(edge) {
                Some(pair) => pair,
                None => continue,
            };
            if source != idx && target != idx {
                continue;
            }
            if let Some(sewer) = self.graph.edge_weight(edge) {
                total += sewer.capacity_mgd;
            }
        }
        total
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plant(id: &str, role: PlantRole) -> Plant {
        Plant {
            id: id.to_string(),
            name: format!("plant-{id}"),
            role,
        }
    }

    fn sewer(id: &str, capacity_mgd: f64, basis: DemandBasis) -> Sewer {
        Sewer {
            id: id.to_string(),
            capacity_mgd,
            basis,
        }
    }

    #[test]
    fn build_counts_and_degree() {
        let mut net = Network::new();
        net.add_plant(plant("a", PlantRole::Pump)).unwrap();
        net.add_plant(plant("b", PlantRole::Treatment)).unwrap();
        net.add_plant(plant("c", PlantRole::Outfall)).unwrap();
        net.add_sewer("a", "b", sewer("s1", 1.0, DemandBasis::DryWeather))
            .unwrap();
        net.add_sewer("b", "c", sewer("s2", 2.0, DemandBasis::WetWeather))
            .unwrap();

        assert_eq!(net.plant_count(), 3);
        assert_eq!(net.sewer_count(), 2);
        assert_eq!(net.degree("a"), Some(1));
        assert_eq!(net.degree("b"), Some(2));
        assert_eq!(net.degree("missing"), None);
    }

    #[test]
    fn connectivity_and_gap() {
        let mut net = Network::default();
        for id in ["a", "b", "c", "d"] {
            net.add_plant(plant(id, PlantRole::Pump)).unwrap();
        }
        net.add_sewer("a", "b", sewer("s1", 1.0, DemandBasis::DryWeather))
            .unwrap();
        net.add_sewer("b", "c", sewer("s2", 1.0, DemandBasis::DryWeather))
            .unwrap();

        assert!(net.is_connected("a", "c"));
        assert!(!net.is_connected("a", "d"));
    }

    #[test]
    fn incident_capacity_sums() {
        let mut net = Network::new();
        net.add_plant(plant("a", PlantRole::Pump)).unwrap();
        net.add_plant(plant("b", PlantRole::Treatment)).unwrap();
        net.add_plant(plant("c", PlantRole::Outfall)).unwrap();
        net.add_sewer("a", "b", sewer("s1", 1.5, DemandBasis::DryWeather))
            .unwrap();
        net.add_sewer("a", "c", sewer("s2", 2.5, DemandBasis::WetWeather))
            .unwrap();

        assert!((net.incident_capacity_mgd("a") - 4.0).abs() < 1e-9);
        assert!((net.incident_capacity_mgd("b") - 1.5).abs() < 1e-9);
        assert!((net.incident_capacity_mgd("missing")).abs() < 1e-9);
    }

    #[test]
    fn basis_is_preserved() {
        let mut net = Network::new();
        net.add_plant(plant("a", PlantRole::Pump)).unwrap();
        net.add_plant(plant("b", PlantRole::Treatment)).unwrap();
        net.add_sewer("a", "b", sewer("wet", 3.0, DemandBasis::WetWeather))
            .unwrap();
        net.add_sewer("a", "b", sewer("dry", 4.0, DemandBasis::DryWeather))
            .unwrap();

        let mut wet_seen = false;
        let mut dry_seen = false;
        for edge in net.graph.edge_indices() {
            let stored = net.graph.edge_weight(edge).unwrap();
            match stored.id.as_str() {
                "wet" => {
                    assert_eq!(stored.basis, DemandBasis::WetWeather);
                    wet_seen = true;
                }
                "dry" => {
                    assert_eq!(stored.basis, DemandBasis::DryWeather);
                    dry_seen = true;
                }
                _ => {}
            }
        }
        assert!(wet_seen && dry_seen);
    }

    #[test]
    fn diverse_path_on_ring() {
        let mut net = Network::new();
        for id in ["a", "b", "c", "d"] {
            net.add_plant(plant(id, PlantRole::Pump)).unwrap();
        }
        net.add_sewer("a", "b", sewer("s1", 1.0, DemandBasis::DryWeather))
            .unwrap();
        net.add_sewer("b", "c", sewer("s2", 1.0, DemandBasis::DryWeather))
            .unwrap();
        net.add_sewer("c", "d", sewer("s3", 1.0, DemandBasis::DryWeather))
            .unwrap();
        net.add_sewer("d", "a", sewer("s4", 1.0, DemandBasis::DryWeather))
            .unwrap();

        assert!(net.has_diverse_path("a", "c"));
    }

    #[test]
    fn no_diverse_path_on_chain() {
        let mut net = Network::new();
        for id in ["a", "b", "c"] {
            net.add_plant(plant(id, PlantRole::Pump)).unwrap();
        }
        net.add_sewer("a", "b", sewer("s1", 1.0, DemandBasis::DryWeather))
            .unwrap();
        net.add_sewer("b", "c", sewer("s2", 1.0, DemandBasis::DryWeather))
            .unwrap();

        assert!(!net.has_diverse_path("a", "c"));
    }

    #[test]
    fn add_plant_rejects_duplicate() {
        let mut net = Network::new();
        net.add_plant(plant("a", PlantRole::Pump)).unwrap();
        let err = net.add_plant(plant("a", PlantRole::Treatment)).unwrap_err();
        assert!(matches!(err, NetworkError::DuplicatePlant(id) if id == "a"));
    }

    #[test]
    fn add_sewer_rejects_bad_inputs() {
        let mut net = Network::new();
        net.add_plant(plant("a", PlantRole::Pump)).unwrap();
        net.add_plant(plant("b", PlantRole::Treatment)).unwrap();

        let non_positive = net
            .add_sewer("a", "b", sewer("bad", 0.0, DemandBasis::DryWeather))
            .unwrap_err();
        assert!(matches!(
            non_positive,
            NetworkError::NonPositiveCapacity(c) if c == 0.0
        ));

        let unknown = net
            .add_sewer("a", "missing", sewer("s", 1.0, DemandBasis::DryWeather))
            .unwrap_err();
        assert!(matches!(unknown, NetworkError::UnknownPlant(id) if id == "missing"));
    }
}
