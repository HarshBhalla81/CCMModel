//! Rules (R1)-(R3c) of Section 4.2: telling an empty neighbour from a vacated
//! one using only what scouts saw and what the group remembers.

use super::memory::{AgentState, Memory, Reached, ScoutRecord};
use crate::common::NodeType;
use ccm_core::{AgentId, PortId};

/// The rule that decided a probe, named as in the paper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProbeRule {
    /// Somebody was home.
    R1,
    /// The port-1 neighbour was occupied and a group member claims the node.
    R3aI,
    /// The port-1 neighbour was occupied and nobody claims the node: empty.
    R3aII,
    /// Nobody at the port-1 neighbour either, reached on its port 1: empty.
    R3b,
    /// Nobody two hops on: empty.
    R3cI,
    /// Two hops on was occupied, and a chain of claims leads back: vacated.
    R3cIIAlpha,
    /// Two hops on was occupied, but the chain breaks: empty.
    R3cIIBeta,
}

/// What a probe concluded about the neighbour behind one port.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Resolution {
    pub port: PortId,
    /// `psi(y)`: who owns the neighbour, or `None` if it is empty.
    pub owner: Option<AgentId>,
    pub node_type: NodeType,
    pub rule: ProbeRule,
}

/// Concludes one probe at the head, after every scout is back.
///
/// `pool` is the group as it stands at the head. A vacated node's agent is in
/// it, so a vacated node is recognised by finding the group member whose stored
/// port-1 neighbour is the one the scout saw.
pub(super) fn resolve(scout: &ScoutRecord, pool: &[Memory]) -> Resolution {
    let claimant = |p1: AgentId, port: PortId| {
        pool.iter().find(|member| {
            member.state == AgentState::SettledScout
                && member.p1_neighbor == Some(p1)
                && member.port_at_p1_neighbor == Some(port)
        })
    };
    let empty = |rule| Resolution {
        port: scout.port,
        owner: None,
        node_type: NodeType::Unvisited,
        rule,
    };
    let vacated = |member: &Memory, rule| Resolution {
        port: scout.port,
        owner: Some(member.id),
        node_type: member.node_type,
        rule,
    };

    match scout.reached {
        Reached::R1 => {
            let (owner, node_type) = scout.found.expect("R1 records who was home");
            Resolution {
                port: scout.port,
                owner: Some(owner),
                node_type,
                rule: ProbeRule::R1,
            }
        }
        Reached::R3b => empty(ProbeRule::R3b),
        Reached::R3a => {
            let z = scout.p1.expect("R3a records who was at z");
            claimant(z, scout.port_at_p1)
                .map_or_else(|| empty(ProbeRule::R3aII), |b| vacated(b, ProbeRule::R3aI))
        }
        Reached::R3c => {
            let Some(w) = scout.p1p1 else {
                return empty(ProbeRule::R3cI);
            };
            // c = psi(z), recognised by its own port-1 neighbour w; then
            // b = psi(y), recognised by having c as its port-1 neighbour.
            claimant(w, scout.port_at_p1p1)
                .and_then(|c| claimant(c.id, scout.port_at_p1))
                .map_or_else(
                    || empty(ProbeRule::R3cIIBeta),
                    |b| vacated(b, ProbeRule::R3cIIAlpha),
                )
        }
    }
}
