use k8s_openapi::api::core::v1::ObjectReference;
use serde::{Deserialize, Serialize};

// DeploymentTriggerPolicy describes a policy for a single trigger that results in a new deployment.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentTriggerPolicy {
    // Type of the trigger
    pub type_: Option<String>,
    // ImageChangeParams represents the parameters for the ImageChange trigger.
    pub image_change_params: Option<DeploymentTriggerImageChangeParams>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentTriggerImageChangeParams {
    // Automatic means that the detection of a new version should trigger a new deployment.
    pub automatic: Option<bool>,
    // ContainerNames is a list of names of containers
    pub container_names: Option<Vec<String>>,
    // From is a reference to a ReplicationController
    pub from: ObjectReference,
    // LastTriggeredImage is the image name from the previous deployment
    pub last_triggered_image: Option<String>,
}

// impl DeploymentTriggerImageChangeParams {
//     pub fn to_annotation(&self) -> Option<serde_json::Value> {
//         let mut result = Vec::new();
//         for container_name in self.container_names.iter().flatten() {
//             result.push(json!({
//                 "from": self.from,
//                 "fieldPath": format!("spec.template.spec.containers[?(@.name==\"{container_name}\")].image"),
//             }));
//         }

//         if result.is_empty() {
//             return None;
//         } else {
//             Some(serde_json::Value::Array(result))
//         }
//     }
// }

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KubernetesImageTrigger {
    from: ObjectReference,
    field_path: String,
}

// impl Into<BTreeMap<String, KubernetesImageTrigger>> for DeploymentTriggerImageChangeParams {
//     fn into(self) -> BTreeMap<String, KubernetesImageTrigger> {
//         let mut result = BTreeMap::new();
//         for container_name in self.container_names.iter().flatten() {
//             result.insert(
//                 container_name.clone(),
//                 KubernetesImageTrigger {
//                     from: self.from.clone(),
//                     field_path: format!(
//                         "spec.template.spec.containers[?(@.name==\"{container_name}\")].image"
//                     ),
//                 },
//             );
//         }
//         result
//     }
// }

impl DeploymentTriggerPolicy {
    pub fn to_kubernetes_image_triggers(&self) -> Vec<KubernetesImageTrigger> {
        let mut result = Vec::new();
        if self.type_.as_deref() == Some("ImageChange") {
            if let Some(params) = &self.image_change_params {
                for container_name in params.container_names.iter().flatten() {
                    result.push(KubernetesImageTrigger {
                        from: params.from.clone(),
                        field_path: format!(
                            "spec.template.spec.containers[?(@.name==\"{container_name}\")].image"
                        ),
                    });
                }
            }
        }

        result
    }
}
