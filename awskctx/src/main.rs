use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eks {
    pub clusters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ec2 {
    #[serde(rename = "Regions")]
    pub regions: Vec<Region>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    #[serde(rename = "Endpoint")]
    pub endpoint: String,
    #[serde(rename = "RegionName")]
    pub region_name: String,
}

fn main() {
    // get a list of all regions
    let ec2command = Command::new("sh")
        .arg("-c")
        .arg("aws ec2 describe-regions --region us-west-2")
        .output()
        .expect("failed executing ec2command");
    let ec2: Ec2 = serde_json::from_slice(&ec2command.stdout).expect("failed to deserialize");

    let regions: Vec<String> = ec2
        .regions
        .iter()
        .map(|region| region.region_name.clone())
        .collect();
    // iterate over regions to get clusters
    regions.par_iter().for_each(|region| {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "aws eks list-clusters --output json --region {}",
                region
            ))
            .output()
            .expect("failed to execute process");
        let aws: Eks = serde_json::from_slice(&output.stdout).expect("json was not found");
        // configure
        aws.clusters.iter().for_each(|cluster| {
            let update_kubeconfig = Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "aws eks update-kubeconfig --name {} --alias {} --region {}",
                    cluster, cluster, region
                ))
                .output()
                .expect("failed updating kubeconfig");
            println!(
                "{:#?}",
                std::str::from_utf8(&update_kubeconfig.stdout).unwrap()
            );
            println!(
                "{:#?}",
                std::str::from_utf8(&update_kubeconfig.stderr).unwrap()
            );
        });
    });
}
