use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nodes {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub spec: Spec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    #[serde(rename = "providerID")]
    pub provider_id: String,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node_id: String,
    pub region: String,
    pub subregion: Option<String>,
}

impl Nodes {
    pub fn get() -> Result<Self> {
        let kubectl_nodes = Command::new("sh")
            .arg("-c")
            .arg("kubectl get nodes -ojson")
            .output()
            .expect("failed executing `kubectl get nodes`");
        let nodes = serde_json::from_slice(&kubectl_nodes.stdout)?;
        Ok(nodes)
    }
    pub fn get_providers(self) -> Result<Vec<String>> {
        let providers: Vec<String> = self
            .items
            .iter()
            .map(|n| n.spec.provider_id.clone())
            .collect();
        Ok(providers)
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.subregion {
            Some(s) => write!(f, "Node: {} in {}{}", self.node_id, self.region, s),
            None => write!(f, "Node: {} in {}", self.node_id, self.region),
        }
    }
}

pub fn get_node_list() -> Result<Vec<Node>> {
    let node_providers: Vec<String> = Nodes::get()?.get_providers()?;
    let mut nodes: Vec<Node> = Vec::new();
    for s in node_providers {
        let (node_id, region) = s
            .split('/')
            .nth(4)
            .zip(s.split('/').nth(3))
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .unwrap();
        let (region_trimmed, subregion) = trim_region(region)?;
        nodes.push(Node {
            node_id,
            region: region_trimmed,
            subregion,
        });
    }

    Ok(nodes)
}
fn trim_region(region: String) -> Result<(String, Option<String>)> {
    let last_char = region.chars().last().unwrap();
    let (region_trimmed, subregion) = if last_char.is_alphabetic() {
        (
            region.trim_end_matches(last_char),
            Some(last_char.to_string()),
        )
    } else {
        (region.as_str(), None)
    };
    Ok((region_trimmed.to_string(), subregion))
}
