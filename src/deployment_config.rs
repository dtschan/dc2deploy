// Converted from https://github.com/openshift/api/blob/rebase-1.14.0/apps/v1/types.go

use std::collections::BTreeMap;

use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{PodTemplateSpec},
    },
    apimachinery::pkg::apis::meta::v1::LabelSelector,
};
use kube::{api::ObjectList, CustomResource};
//use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    deployment_config_strategy::DeploymentConfigStrategy,
    deployment_trigger_policy::{DeploymentTriggerPolicy, KubernetesImageTrigger},
};

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum DeploymentConfigOrList {
    DeploymentConfig(DeploymentConfig),
    List(ObjectList<DeploymentConfig>),
}

#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[kube(
    group = "apps.openshift.io",
    version = "v1",
    kind = "DeploymentConfig",
    plural = "deploymentconfigs",
    derive = "PartialEq",
    schema = "disabled",
    namespaced
)]
pub struct DeploymentConfigSpec {
    pub strategy: Option<DeploymentConfigStrategy>,
    pub min_ready_seconds: Option<i32>,
    pub triggers: Option<Vec<DeploymentTriggerPolicy>>,
    pub replicas: Option<i32>,
    pub revision_history_limit: Option<i32>,
    pub test: bool,
    pub paused: Option<bool>,
    pub selector: Option<BTreeMap<String, String>>,
    pub template: PodTemplateSpec,
}

impl Into<Deployment> for DeploymentConfig {
    fn into(mut self) -> Deployment {
        let triggers = self.spec.triggers.take().unwrap_or_default();
        let mut metadata = self.metadata;
        let mut spec: DeploymentSpec = self.spec.into();
        if !triggers.is_empty() {
            let image_change_triggers: Vec<KubernetesImageTrigger> = triggers
                .iter()
                .flat_map(|t| t.to_kubernetes_image_triggers())
                .collect();
            if !image_change_triggers.is_empty() {
                let annotations = metadata.annotations.get_or_insert_with(BTreeMap::new);
                annotations.insert(
                    "image.openshift.io/triggers".into(),
                    serde_json::to_string(&image_change_triggers).unwrap(),
                );
            }
        } else {
            // if triggers.as_ref().map_or(true, |t| t.is_empty()) {
            spec.paused = Some(true);
            eprintln!("WARNING: deployment has been set to paused because there are no triggers in the DeploymentConfig")
        }

        // Delete last-applied annoation as it is no longer correct after the conversion
        metadata
            .annotations
            .as_mut()
            .map(|a| a.remove("kubectl.kubernetes.io/last-applied-configuration"));

        // Images are optional in DeploymentConfig buts not in Deployments.
        // Replace emptys image with " ". Used when images are set with image change triggers.
        // iterate over all containers

        //for container in spec.template.spec.as_mut().unwrap_or(&mut PodSpec::default()).containers.iter_mut() {

        spec.template.spec.as_mut().map(|s| {
            s.containers.iter_mut().for_each(|c| {
                c.image.get_or_insert(" ".into());
            })
        });

        Deployment {
            metadata,
            spec: Some(spec),
            status: None,
        }
    }
}

impl Into<DeploymentSpec> for DeploymentConfigSpec {
    fn into(self) -> DeploymentSpec {
        let timeout_seconds = self
            .strategy
            .as_ref()
            .and_then(|s| s.timeout_seconds())
            .map(|t| t as i32);
        DeploymentSpec {
            replicas: self.replicas,
            selector: LabelSelector {
                match_labels: self.selector,
                ..Default::default()
            },
            template: self.template,
            min_ready_seconds: self.min_ready_seconds,
            revision_history_limit: self.revision_history_limit,
            paused: self.paused,
            strategy: self.strategy.map(Into::into),
            progress_deadline_seconds: timeout_seconds,
        }
    }
}
