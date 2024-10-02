// Converted from https://github.com/openshift/api/blob/rebase-1.14.0/apps/v1/types.go

use std::collections::BTreeMap;

use k8s_openapi::{
    api::{
        apps::v1::{DeploymentStrategy, RollingUpdateDeployment},
        core::v1::ResourceRequirements,
    },
    apimachinery::pkg::util::intstr::IntOrString,
};
use serde::{Deserialize, Serialize};

// DeploymentStrategy describes how to perform a deployment.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentConfigStrategy {
    // Type is the name of a deployment strategy.
    pub type_: Option<String>,

    // CustomParams are the input to the Custom deployment strategy, and may also
    // be specified for the Recreate and Rolling strategies to customize the execution
    // process that runs the deployment.
    pub custom_params: Option<CustomDeploymentStrategyParams>,
    // RecreateParams are the input to the Recreate deployment strategy.
    pub recreate_params: Option<RecreateDeploymentStrategyParams>,
    // RollingParams are the input to the Rolling deployment strategy.
    pub rolling_params: Option<RollingDeploymentStrategyParams>,

    // Resources contains resource requirements to execute the deployment and any hooks.
    pub resources: Option<ResourceRequirements>,
    // Labels is a set of key, value pairs added to custom deployer and lifecycle pre/post hook pods.
    pub labels: Option<BTreeMap<String, String>>,
    // Annotations is a set of key, value pairs added to custom deployer and lifecycle pre/post hook pods.
    pub annotations: Option<BTreeMap<String, String>>,

    // ActiveDeadlineSeconds is the duration in seconds that the deployer pods for this deployment
    // config may be active on a node before the system actively tries to terminate them.
    pub active_deadline_seconds: Option<i64>,
}

impl DeploymentConfigStrategy {
    pub fn timeout_seconds(&self) -> Option<i64> {
        match self.type_.as_deref() {
            Some("Rolling") => self.rolling_params.as_ref().and_then(|p| p.timeout_seconds),
            Some("Recreate") => self
                .recreate_params
                .as_ref()
                .and_then(|p| p.timeout_seconds),
            _ => None,
        }
    }
}

impl Into<DeploymentStrategy> for DeploymentConfigStrategy {
    fn into(self) -> DeploymentStrategy {
        DeploymentStrategy {
            type_: self.type_.map(|t| t.replace("Rolling", "RollingUpdate")),
            rolling_update: self.rolling_params.map(Into::into),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RollingDeploymentStrategyParams {
    // UpdatePeriodSeconds is the time to wait between individual pod updates.
    // If the value is nil, a default will be used.
    pub update_period_seconds: Option<i64>,
    // IntervalSeconds is the time to wait between polling deployment status
    // after update. If the value is nil, a default will be used.
    pub interval_seconds: Option<i64>,
    // TimeoutSeconds is the time to wait for updates before giving up. If the
    // value is nil, a default will be used.
    pub timeout_seconds: Option<i64>,
    // MaxUnavailable is the maximum number of pods that can be unavailable
    // during the update. Value can be an absolute number (ex: 5) or a
    // percentage of total pods at the start of update (ex: 10%). Absolute
    // number is calculated from percentage by rounding down.
    //
    // This cannot be 0 if MaxSurge is 0. By default, 25% is used.
    //
    // Example: when this is set to 30%, the old RC can be scaled down by 30%
    // immediately when the rolling update starts. Once new pods are ready, old
    // RC can be scaled down further, followed by scaling up the new RC,
    // ensuring that at least 70% of original number of pods are available at
    // all times during the update.
    pub max_unavailable: Option<IntOrString>,
    // MaxSurge is the maximum number of pods that can be scheduled above the
    // original number of pods. Value can be an absolute number (ex: 5) or a
    // percentage of total pods at the start of the update (ex: 10%). Absolute
    // number is calculated from percentage by rounding up.
    //
    // This cannot be 0 if MaxUnavailable is 0. By default, 25% is used.
    //
    // Example: when this is set to 30%, the new RC can be scaled up by 30%
    // immediately when the rolling update starts. Once old pods have been
    // killed, new RC can be scaled up further, ensuring that total number of
    // pods running at any time during the update is atmost 130% of original
    // pods.
    pub max_surge: Option<IntOrString>,
    // Pre is a lifecycle hook which is executed before the deployment process
    // begins. All LifecycleHookFailurePolicy values are supported.
    //pre *LifecycleHook `json:"pre,omitempty" protobuf:"bytes,7,opt,name=pre"`
    // Post is a lifecycle hook which is executed after the strategy has
    // finished all deployment logic. All LifecycleHookFailurePolicy values
    // are supported.
    // post *LifecycleHook `json:"post,omitempty" protobuf:"bytes,8,opt,name=post"`
}

impl Into<RollingUpdateDeployment> for RollingDeploymentStrategyParams {
    fn into(self) -> RollingUpdateDeployment {
        RollingUpdateDeployment {
            max_unavailable: self.max_unavailable,
            max_surge: self.max_surge,
        }
    }
}

// RecreateDeploymentStrategyParams are the input to the Recreate deployment
// strategy.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecreateDeploymentStrategyParams {
    // TimeoutSeconds is the time to wait for updates before giving up. If the
    // value is nil, a default will be used.
    pub timeout_seconds: Option<i64>,
    // Pre is a lifecycle hook which is executed before the strategy manipulates
    // the deployment. All LifecycleHookFailurePolicy values are supported.
    //Pre *LifecycleHook `json:"pre,omitempty" protobuf:"bytes,2,opt,name=pre"`
    // Mid is a lifecycle hook which is executed while the deployment is scaled down to zero before the first new
    // pod is created. All LifecycleHookFailurePolicy values are supported.
    // Mid *LifecycleHook `json:"mid,omitempty" protobuf:"bytes,3,opt,name=mid"`
    // Post is a lifecycle hook which is executed after the strategy has
    // finished all deployment logic. All LifecycleHookFailurePolicy values are supported.
    // Post *LifecycleHook `json:"post,omitempty" protobuf:"bytes,4,opt,name=post"`
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CustomDeploymentStrategyParams {}
