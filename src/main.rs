mod deployment_config;
mod deployment_config_strategy;
mod deployment_strategy_params;
mod deployment_trigger_policy;

use std::io;

use deployment_config::DeploymentConfigOrList;
use k8s_openapi::api::apps::v1::Deployment;

fn main() {
    let stdin = io::stdin().lock();
    let deployment_configs_or_list: DeploymentConfigOrList =
        serde_yaml::from_reader(stdin).unwrap();
    match deployment_configs_or_list {
        DeploymentConfigOrList::DeploymentConfig(deployment_config) => {
            let deployment: Deployment = deployment_config.into();
            serde_yaml::to_writer(io::stdout(), &deployment).unwrap();
        }
        DeploymentConfigOrList::List(deployment_config_list) => {
            for deployment_config in deployment_config_list.items {
                println!("---");
                let deployment: Deployment = deployment_config.into();
                serde_yaml::to_writer(io::stdout(), &deployment).unwrap();
            }
        }
    };
}
