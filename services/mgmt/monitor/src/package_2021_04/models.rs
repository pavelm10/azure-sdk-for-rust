#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Action descriptor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[doc = "Specifies the action. Supported values - AlertingAction, LogToMetricAction"]
    #[serde(rename = "odata.type")]
    pub odata_type: String,
}
impl Action {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
    }
}
#[doc = "An Azure action group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    #[doc = "The short name of the action group. This will be used in SMS messages."]
    #[serde(rename = "groupShortName")]
    pub group_short_name: String,
    #[doc = "Indicates whether this action group is enabled. If an action group is not enabled, then none of its receivers will receive communications."]
    pub enabled: bool,
    #[doc = "The list of email receivers that are part of this action group."]
    #[serde(
        rename = "emailReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub email_receivers: Vec<EmailReceiver>,
    #[doc = "The list of SMS receivers that are part of this action group."]
    #[serde(
        rename = "smsReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sms_receivers: Vec<SmsReceiver>,
    #[doc = "The list of webhook receivers that are part of this action group."]
    #[serde(
        rename = "webhookReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub webhook_receivers: Vec<WebhookReceiver>,
    #[doc = "The list of ITSM receivers that are part of this action group."]
    #[serde(
        rename = "itsmReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub itsm_receivers: Vec<ItsmReceiver>,
    #[doc = "The list of AzureAppPush receivers that are part of this action group."]
    #[serde(
        rename = "azureAppPushReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub azure_app_push_receivers: Vec<AzureAppPushReceiver>,
    #[doc = "The list of AutomationRunbook receivers that are part of this action group."]
    #[serde(
        rename = "automationRunbookReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub automation_runbook_receivers: Vec<AutomationRunbookReceiver>,
    #[doc = "The list of voice receivers that are part of this action group."]
    #[serde(
        rename = "voiceReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub voice_receivers: Vec<VoiceReceiver>,
    #[doc = "The list of logic app receivers that are part of this action group."]
    #[serde(
        rename = "logicAppReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logic_app_receivers: Vec<LogicAppReceiver>,
    #[doc = "The list of azure function receivers that are part of this action group."]
    #[serde(
        rename = "azureFunctionReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub azure_function_receivers: Vec<AzureFunctionReceiver>,
    #[doc = "The list of ARM role receivers that are part of this action group. Roles are Azure RBAC roles and only built-in roles are supported."]
    #[serde(
        rename = "armRoleReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub arm_role_receivers: Vec<ArmRoleReceiver>,
}
impl ActionGroup {
    pub fn new(group_short_name: String, enabled: bool) -> Self {
        Self {
            group_short_name,
            enabled,
            email_receivers: Vec::new(),
            sms_receivers: Vec::new(),
            webhook_receivers: Vec::new(),
            itsm_receivers: Vec::new(),
            azure_app_push_receivers: Vec::new(),
            automation_runbook_receivers: Vec::new(),
            voice_receivers: Vec::new(),
            logic_app_receivers: Vec::new(),
            azure_function_receivers: Vec::new(),
            arm_role_receivers: Vec::new(),
        }
    }
}
#[doc = "A list of action groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupList {
    #[doc = "The list of action groups."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ActionGroupResource>,
    #[doc = "Provides the link to retrieve the next set of elements."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ActionGroupList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ActionGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure action group for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupPatch {
    #[doc = "Indicates whether this action group is enabled. If an action group is not enabled, then none of its actions will be activated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ActionGroupPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An action group object for the body of patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupPatchBody {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An Azure action group for patch operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroupPatch>,
}
impl ActionGroupPatchBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An action group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupResource {
    #[serde(flatten)]
    pub azure_resource: AzureResource,
    #[doc = "An Azure action group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroup>,
}
impl ActionGroupResource {
    pub fn new(azure_resource: AzureResource) -> Self {
        Self {
            azure_resource,
            properties: None,
        }
    }
}
#[doc = "An Azure activity log alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlert {
    #[doc = "A list of resourceIds that will be used as prefixes. The alert will only apply to activityLogs with resourceIds that fall under one of these prefixes. This list must include at least one item."]
    pub scopes: Vec<String>,
    #[doc = "Indicates whether this activity log alert is enabled. If an activity log alert is not enabled, then none of its actions will be activated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "An Activity Log alert condition that is met when all its member conditions are met."]
    pub condition: ActivityLogAlertAllOfCondition,
    #[doc = "A list of activity log alert actions."]
    pub actions: ActivityLogAlertActionList,
    #[doc = "A description of this activity log alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ActivityLogAlert {
    pub fn new(scopes: Vec<String>, condition: ActivityLogAlertAllOfCondition, actions: ActivityLogAlertActionList) -> Self {
        Self {
            scopes,
            enabled: None,
            condition,
            actions,
            description: None,
        }
    }
}
#[doc = "A pointer to an Azure Action Group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertActionGroup {
    #[doc = "The resourceId of the action group. This cannot be null or empty."]
    #[serde(rename = "actionGroupId")]
    pub action_group_id: String,
    #[doc = "the dictionary of custom properties to include with the post operation. These data are appended to the webhook payload."]
    #[serde(rename = "webhookProperties", default, skip_serializing_if = "Option::is_none")]
    pub webhook_properties: Option<serde_json::Value>,
}
impl ActivityLogAlertActionGroup {
    pub fn new(action_group_id: String) -> Self {
        Self {
            action_group_id,
            webhook_properties: None,
        }
    }
}
#[doc = "A list of activity log alert actions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityLogAlertActionList {
    #[doc = "The list of activity log alerts."]
    #[serde(
        rename = "actionGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub action_groups: Vec<ActivityLogAlertActionGroup>,
}
impl ActivityLogAlertActionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Activity Log alert condition that is met when all its member conditions are met."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertAllOfCondition {
    #[doc = "The list of activity log alert conditions."]
    #[serde(rename = "allOf")]
    pub all_of: Vec<ActivityLogAlertLeafCondition>,
}
impl ActivityLogAlertAllOfCondition {
    pub fn new(all_of: Vec<ActivityLogAlertLeafCondition>) -> Self {
        Self { all_of }
    }
}
#[doc = "An Activity Log alert condition that is met by comparing an activity log field and value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertLeafCondition {
    #[doc = "The name of the field that this condition will examine. The possible values for this field are (case-insensitive): 'resourceId', 'category', 'caller', 'level', 'operationName', 'resourceGroup', 'resourceProvider', 'status', 'subStatus', 'resourceType', or anything beginning with 'properties.'."]
    pub field: String,
    #[doc = "The field value will be compared to this value (case-insensitive) to determine if the condition is met."]
    pub equals: String,
}
impl ActivityLogAlertLeafCondition {
    pub fn new(field: String, equals: String) -> Self {
        Self { field, equals }
    }
}
#[doc = "A list of activity log alerts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityLogAlertList {
    #[doc = "The list of activity log alerts."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ActivityLogAlertResource>,
    #[doc = "Provides the link to retrieve the next set of elements."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ActivityLogAlertList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ActivityLogAlertList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure activity log alert for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityLogAlertPatch {
    #[doc = "Indicates whether this activity log alert is enabled. If an activity log alert is not enabled, then none of its actions will be activated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ActivityLogAlertPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An activity log alert object for the body of patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityLogAlertPatchBody {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An Azure activity log alert for patch operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActivityLogAlertPatch>,
}
impl ActivityLogAlertPatchBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An activity log alert resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "An Azure activity log alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActivityLogAlert>,
}
impl ActivityLogAlertResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "the aggregation type of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AggregationType {
    None,
    Average,
    Count,
    Minimum,
    Maximum,
    Total,
}
#[doc = "An alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRule {
    #[doc = "the name of the alert rule."]
    pub name: String,
    #[doc = "the description of the alert rule that will be included in the alert email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "the flag that indicates whether the alert rule is enabled."]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[doc = "The condition that results in the alert rule being activated."]
    pub condition: RuleCondition,
    #[doc = "The action that is performed when the alert rule becomes active, and when an alert condition is resolved."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[doc = "the array of actions that are performed when the alert rule becomes active, and when an alert condition is resolved."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<RuleAction>,
    #[doc = "Last time the rule was updated in ISO8601 format."]
    #[serde(rename = "lastUpdatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
}
impl AlertRule {
    pub fn new(name: String, is_enabled: bool, condition: RuleCondition) -> Self {
        Self {
            name,
            description: None,
            provisioning_state: None,
            is_enabled,
            condition,
            action: None,
            actions: Vec::new(),
            last_updated_time: None,
        }
    }
}
#[doc = "The alert rule resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "An alert rule."]
    pub properties: AlertRule,
}
impl AlertRuleResource {
    pub fn new(resource: Resource, properties: AlertRule) -> Self {
        Self { resource, properties }
    }
}
#[doc = "Represents a collection of alert rule resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleResourceCollection {
    #[doc = "the values for the alert rule resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AlertRuleResource>,
}
impl azure_core::Continuable for AlertRuleResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AlertRuleResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The alert rule object for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleResourcePatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertRule>,
}
impl AlertRuleResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Severity Level of Alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AlertSeverity")]
pub enum AlertSeverity {
    #[serde(rename = "0")]
    N0,
    #[serde(rename = "1")]
    N1,
    #[serde(rename = "2")]
    N2,
    #[serde(rename = "3")]
    N3,
    #[serde(rename = "4")]
    N4,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AlertSeverity {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AlertSeverity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AlertSeverity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N0 => serializer.serialize_unit_variant("AlertSeverity", 0u32, "0"),
            Self::N1 => serializer.serialize_unit_variant("AlertSeverity", 1u32, "1"),
            Self::N2 => serializer.serialize_unit_variant("AlertSeverity", 2u32, "2"),
            Self::N3 => serializer.serialize_unit_variant("AlertSeverity", 3u32, "3"),
            Self::N4 => serializer.serialize_unit_variant("AlertSeverity", 4u32, "4"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specify action need to be taken when rule type is Alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertingAction {
    #[serde(flatten)]
    pub action: Action,
    #[doc = "Severity Level of Alert"]
    pub severity: AlertSeverity,
    #[doc = "Azure action group"]
    #[serde(rename = "aznsAction", default, skip_serializing_if = "Option::is_none")]
    pub azns_action: Option<AzNsActionGroup>,
    #[doc = "time (in minutes) for which Alerts should be throttled or suppressed."]
    #[serde(rename = "throttlingInMin", default, skip_serializing_if = "Option::is_none")]
    pub throttling_in_min: Option<i32>,
    #[doc = "The condition that results in the Log Search rule."]
    pub trigger: TriggerCondition,
}
impl AlertingAction {
    pub fn new(action: Action, severity: AlertSeverity, trigger: TriggerCondition) -> Self {
        Self {
            action,
            severity,
            azns_action: None,
            throttling_in_min: None,
            trigger,
        }
    }
}
#[doc = "An arm role receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmRoleReceiver {
    #[doc = "The name of the arm role receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The arm role id."]
    #[serde(rename = "roleId")]
    pub role_id: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
impl ArmRoleReceiver {
    pub fn new(name: String, role_id: String) -> Self {
        Self {
            name,
            role_id,
            use_common_alert_schema: None,
        }
    }
}
#[doc = "The Azure Automation Runbook notification receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRunbookReceiver {
    #[doc = "The Azure automation account Id which holds this runbook and authenticate to Azure resource."]
    #[serde(rename = "automationAccountId")]
    pub automation_account_id: String,
    #[doc = "The name for this runbook."]
    #[serde(rename = "runbookName")]
    pub runbook_name: String,
    #[doc = "The resource id for webhook linked to this runbook."]
    #[serde(rename = "webhookResourceId")]
    pub webhook_resource_id: String,
    #[doc = "Indicates whether this instance is global runbook."]
    #[serde(rename = "isGlobalRunbook")]
    pub is_global_runbook: bool,
    #[doc = "Indicates name of the webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The URI where webhooks should be sent."]
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
impl AutomationRunbookReceiver {
    pub fn new(automation_account_id: String, runbook_name: String, webhook_resource_id: String, is_global_runbook: bool) -> Self {
        Self {
            automation_account_id,
            runbook_name,
            webhook_resource_id,
            is_global_runbook,
            name: None,
            service_uri: None,
            use_common_alert_schema: None,
        }
    }
}
#[doc = "Autoscale notification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleNotification {
    #[doc = "the operation associated with the notification and its value must be \"scale\""]
    pub operation: autoscale_notification::Operation,
    #[doc = "Email notification of an autoscale event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailNotification>,
    #[doc = "the collection of webhook notifications."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub webhooks: Vec<WebhookNotification>,
}
impl AutoscaleNotification {
    pub fn new(operation: autoscale_notification::Operation) -> Self {
        Self {
            operation,
            email: None,
            webhooks: Vec::new(),
        }
    }
}
pub mod autoscale_notification {
    use super::*;
    #[doc = "the operation associated with the notification and its value must be \"scale\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        Scale,
    }
}
#[doc = "Autoscale profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleProfile {
    #[doc = "the name of the profile."]
    pub name: String,
    #[doc = "The number of instances that can be used during this profile."]
    pub capacity: ScaleCapacity,
    #[doc = "the collection of rules that provide the triggers and parameters for the scaling action. A maximum of 10 rules can be specified."]
    pub rules: Vec<ScaleRule>,
    #[doc = "A specific date-time for the profile."]
    #[serde(rename = "fixedDate", default, skip_serializing_if = "Option::is_none")]
    pub fixed_date: Option<TimeWindow>,
    #[doc = "The repeating times at which this profile begins. This element is not used if the FixedDate element is used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
}
impl AutoscaleProfile {
    pub fn new(name: String, capacity: ScaleCapacity, rules: Vec<ScaleRule>) -> Self {
        Self {
            name,
            capacity,
            rules,
            fixed_date: None,
            recurrence: None,
        }
    }
}
#[doc = "A setting that contains all of the configuration for the automatic scaling of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSetting {
    #[doc = "the collection of automatic scaling profiles that specify different scaling parameters for different time periods. A maximum of 20 profiles can be specified."]
    pub profiles: Vec<AutoscaleProfile>,
    #[doc = "the collection of notifications."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub notifications: Vec<AutoscaleNotification>,
    #[doc = "the enabled flag. Specifies whether automatic scaling is enabled for the resource. The default value is 'false'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "the name of the autoscale setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the resource identifier of the resource that the autoscale setting should be added to."]
    #[serde(rename = "targetResourceUri", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_uri: Option<String>,
    #[doc = "the location of the resource that the autoscale setting should be added to."]
    #[serde(rename = "targetResourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_location: Option<String>,
}
impl AutoscaleSetting {
    pub fn new(profiles: Vec<AutoscaleProfile>) -> Self {
        Self {
            profiles,
            notifications: Vec::new(),
            enabled: None,
            name: None,
            target_resource_uri: None,
            target_resource_location: None,
        }
    }
}
#[doc = "The autoscale setting resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "A setting that contains all of the configuration for the automatic scaling of a resource."]
    pub properties: AutoscaleSetting,
}
impl AutoscaleSettingResource {
    pub fn new(resource: Resource, properties: AutoscaleSetting) -> Self {
        Self { resource, properties }
    }
}
#[doc = "Represents a collection of autoscale setting resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingResourceCollection {
    #[doc = "the values for the autoscale setting resources."]
    pub value: Vec<AutoscaleSettingResource>,
    #[doc = "URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AutoscaleSettingResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AutoscaleSettingResourceCollection {
    pub fn new(value: Vec<AutoscaleSettingResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The autoscale setting object for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoscaleSettingResourcePatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "A setting that contains all of the configuration for the automatic scaling of a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutoscaleSetting>,
}
impl AutoscaleSettingResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure action group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzNsActionGroup {
    #[doc = "Azure Action Group reference."]
    #[serde(
        rename = "actionGroup",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub action_group: Vec<String>,
    #[doc = "Custom subject override for all email ids in Azure action group"]
    #[serde(rename = "emailSubject", default, skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
    #[doc = "Custom payload to be sent for all webhook URI in Azure action group"]
    #[serde(rename = "customWebhookPayload", default, skip_serializing_if = "Option::is_none")]
    pub custom_webhook_payload: Option<String>,
}
impl AzNsActionGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Azure mobile App push notification receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAppPushReceiver {
    #[doc = "The name of the Azure mobile app push receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The email address registered for the Azure mobile app."]
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}
impl AzureAppPushReceiver {
    pub fn new(name: String, email_address: String) -> Self {
        Self { name, email_address }
    }
}
#[doc = "An azure function receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFunctionReceiver {
    #[doc = "The name of the azure function receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The azure resource id of the function app."]
    #[serde(rename = "functionAppResourceId")]
    pub function_app_resource_id: String,
    #[doc = "The function name in the function app."]
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[doc = "The http trigger url where http request sent to."]
    #[serde(rename = "httpTriggerUrl")]
    pub http_trigger_url: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
impl AzureFunctionReceiver {
    pub fn new(name: String, function_app_resource_id: String, function_name: String, http_trigger_url: String) -> Self {
        Self {
            name,
            function_app_resource_id,
            function_name,
            http_trigger_url,
            use_common_alert_schema: None,
        }
    }
}
#[doc = "Azure Monitor Metrics destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMonitorMetricsDestination {
    #[doc = "A friendly name for the destination. \r\nThis name should be unique across all destinations (regardless of type) within the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl AzureMonitorMetricsDestination {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Monitor PrivateLinkScope definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMonitorPrivateLinkScope {
    #[serde(flatten)]
    pub private_link_scopes_resource: PrivateLinkScopesResource,
    #[doc = "Properties that define a Azure Monitor PrivateLinkScope resource."]
    pub properties: AzureMonitorPrivateLinkScopeProperties,
}
impl AzureMonitorPrivateLinkScope {
    pub fn new(private_link_scopes_resource: PrivateLinkScopesResource, properties: AzureMonitorPrivateLinkScopeProperties) -> Self {
        Self {
            private_link_scopes_resource,
            properties,
        }
    }
}
#[doc = "Describes the list of Azure Monitor PrivateLinkScope resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMonitorPrivateLinkScopeListResult {
    #[doc = "List of Azure Monitor PrivateLinkScope definitions."]
    pub value: Vec<AzureMonitorPrivateLinkScope>,
    #[doc = "The URI to get the next set of Azure Monitor PrivateLinkScope definitions if too many PrivateLinkScopes where returned in the result set."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureMonitorPrivateLinkScopeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AzureMonitorPrivateLinkScopeListResult {
    pub fn new(value: Vec<AzureMonitorPrivateLinkScope>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties that define a Azure Monitor PrivateLinkScope resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMonitorPrivateLinkScopeProperties {
    #[doc = "Current state of this PrivateLinkScope: whether or not is has been provisioned within the resource group it is defined. Users cannot change this value but are able to read from it. Values will include Provisioning ,Succeeded, Canceled and Failed."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "List of private endpoint connections."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl AzureMonitorPrivateLinkScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AzureResource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "Represents a baseline metadata value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaselineMetadata {
    #[doc = "Name of the baseline metadata."]
    pub name: String,
    #[doc = "Value of the baseline metadata."]
    pub value: String,
}
impl BaselineMetadata {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "Operators allowed in the rule condition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConditionOperator {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}
#[doc = "Result Condition Evaluation criteria."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConditionalOperator")]
pub enum ConditionalOperator {
    GreaterThanOrEqual,
    LessThanOrEqual,
    GreaterThan,
    LessThan,
    Equal,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConditionalOperator {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConditionalOperator {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConditionalOperator {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::GreaterThanOrEqual => serializer.serialize_unit_variant("ConditionalOperator", 0u32, "GreaterThanOrEqual"),
            Self::LessThanOrEqual => serializer.serialize_unit_variant("ConditionalOperator", 1u32, "LessThanOrEqual"),
            Self::GreaterThan => serializer.serialize_unit_variant("ConditionalOperator", 2u32, "GreaterThan"),
            Self::LessThan => serializer.serialize_unit_variant("ConditionalOperator", 3u32, "LessThan"),
            Self::Equal => serializer.serialize_unit_variant("ConditionalOperator", 4u32, "Equal"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for ConditionalOperator {
    fn default() -> Self {
        Self::GreaterThanOrEqual
    }
}
#[doc = "Definition of the endpoint used for accessing configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationAccessEndpointSpec {
    #[doc = "The endpoint. This property is READ-ONLY."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl ConfigurationAccessEndpointSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the criteria for converting log to metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Criteria {
    #[doc = "Name of the metric"]
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[doc = "List of Dimensions for creating metric"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<Dimension>,
}
impl Criteria {
    pub fn new(metric_name: String) -> Self {
        Self {
            metric_name,
            dimensions: Vec::new(),
        }
    }
}
#[doc = "Definition of data collection endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCollectionEndpoint {
    #[doc = "Description of the data collection endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The immutable ID of this data collection endpoint resource. This property is READ-ONLY."]
    #[serde(rename = "immutableId", default, skip_serializing_if = "Option::is_none")]
    pub immutable_id: Option<String>,
    #[doc = "The endpoint used by agents to access their configuration."]
    #[serde(rename = "configurationAccess", default, skip_serializing_if = "Option::is_none")]
    pub configuration_access: Option<serde_json::Value>,
    #[doc = "The endpoint used by clients to ingest logs."]
    #[serde(rename = "logsIngestion", default, skip_serializing_if = "Option::is_none")]
    pub logs_ingestion: Option<serde_json::Value>,
    #[doc = "Network access control rules for the endpoints."]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<serde_json::Value>,
    #[doc = "The resource provisioning state. This property is READ-ONLY."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<data_collection_endpoint::ProvisioningState>,
}
impl DataCollectionEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_collection_endpoint {
    use super::*;
    #[doc = "The resource provisioning state. This property is READ-ONLY."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCollectionEndpointResource {
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives."]
    pub location: String,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The kind of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<data_collection_endpoint_resource::Kind>,
    #[doc = "Fully qualified ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource entity tag (ETag)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl DataCollectionEndpointResource {
    pub fn new(location: String) -> Self {
        Self {
            properties: None,
            location,
            tags: None,
            kind: None,
            id: None,
            name: None,
            type_: None,
            etag: None,
            system_data: None,
        }
    }
}
pub mod data_collection_endpoint_resource {
    use super::*;
    #[doc = "The kind of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Linux,
        Windows,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Linux => serializer.serialize_unit_variant("Kind", 0u32, "Linux"),
                Self::Windows => serializer.serialize_unit_variant("Kind", 1u32, "Windows"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A pageable list of resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCollectionEndpointResourceListResult {
    #[doc = "A list of resources."]
    pub value: Vec<DataCollectionEndpointResource>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataCollectionEndpointResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataCollectionEndpointResourceListResult {
    pub fn new(value: Vec<DataCollectionEndpointResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Definition of what monitoring data to collect and where that data should be sent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCollectionRule {
    #[doc = "Description of the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The immutable ID of this data collection rule. This property is READ-ONLY."]
    #[serde(rename = "immutableId", default, skip_serializing_if = "Option::is_none")]
    pub immutable_id: Option<String>,
    #[doc = "The specification of data sources. \r\nThis property is optional and can be omitted if the rule is meant to be used via direct calls to the provisioned endpoint."]
    #[serde(rename = "dataSources", default, skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<serde_json::Value>,
    #[doc = "The specification of destinations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destinations: Option<serde_json::Value>,
    #[doc = "The specification of data flows."]
    #[serde(
        rename = "dataFlows",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_flows: Vec<DataFlow>,
    #[doc = "The resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<data_collection_rule::ProvisioningState>,
}
impl DataCollectionRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_collection_rule {
    use super::*;
    #[doc = "The resource provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of association of a data collection rule with a monitored Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCollectionRuleAssociation {
    #[doc = "Description of the association."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The resource ID of the data collection rule that is to be associated."]
    #[serde(rename = "dataCollectionRuleId", default, skip_serializing_if = "Option::is_none")]
    pub data_collection_rule_id: Option<String>,
    #[doc = "The resource ID of the data collection endpoint that is to be associated."]
    #[serde(rename = "dataCollectionEndpointId", default, skip_serializing_if = "Option::is_none")]
    pub data_collection_endpoint_id: Option<String>,
    #[doc = "The resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<data_collection_rule_association::ProvisioningState>,
}
impl DataCollectionRuleAssociation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_collection_rule_association {
    use super::*;
    #[doc = "The resource provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of generic ARM proxy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCollectionRuleAssociationProxyOnlyResource {
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Fully qualified ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource entity tag (ETag)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl DataCollectionRuleAssociationProxyOnlyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A pageable list of resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCollectionRuleAssociationProxyOnlyResourceListResult {
    #[doc = "A list of resources."]
    pub value: Vec<DataCollectionRuleAssociationProxyOnlyResource>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataCollectionRuleAssociationProxyOnlyResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataCollectionRuleAssociationProxyOnlyResourceListResult {
    pub fn new(value: Vec<DataCollectionRuleAssociationProxyOnlyResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Definition of ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCollectionRuleResource {
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives."]
    pub location: String,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The kind of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<data_collection_rule_resource::Kind>,
    #[doc = "Fully qualified ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource entity tag (ETag)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl DataCollectionRuleResource {
    pub fn new(location: String) -> Self {
        Self {
            properties: None,
            location,
            tags: None,
            kind: None,
            id: None,
            name: None,
            type_: None,
            etag: None,
            system_data: None,
        }
    }
}
pub mod data_collection_rule_resource {
    use super::*;
    #[doc = "The kind of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Linux,
        Windows,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Linux => serializer.serialize_unit_variant("Kind", 0u32, "Linux"),
                Self::Windows => serializer.serialize_unit_variant("Kind", 1u32, "Windows"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A pageable list of resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCollectionRuleResourceListResult {
    #[doc = "A list of resources."]
    pub value: Vec<DataCollectionRuleResource>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataCollectionRuleResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataCollectionRuleResourceListResult {
    pub fn new(value: Vec<DataCollectionRuleResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Information about a container with data for a given resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataContainer {
    #[doc = "Information about a Log Analytics Workspace."]
    pub workspace: WorkspaceInfo,
}
impl DataContainer {
    pub fn new(workspace: WorkspaceInfo) -> Self {
        Self { workspace }
    }
}
#[doc = "Definition of which streams are sent to which destinations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataFlow {
    #[doc = "List of streams for this data flow."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub streams: Vec<String>,
    #[doc = "List of destinations for this data flow."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub destinations: Vec<String>,
}
impl DataFlow {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specification of data sources that will be collected."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourcesSpec {
    #[doc = "The list of performance counter data source configurations."]
    #[serde(
        rename = "performanceCounters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub performance_counters: Vec<PerfCounterDataSource>,
    #[doc = "The list of Windows Event Log data source configurations."]
    #[serde(
        rename = "windowsEventLogs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub windows_event_logs: Vec<WindowsEventLogDataSource>,
    #[doc = "The list of Syslog data source configurations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub syslog: Vec<SyslogDataSource>,
    #[doc = "The list of Azure VM extension data source configurations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extensions: Vec<ExtensionDataSource>,
}
impl DataSourcesSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specification of destinations that can be used in data flows."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DestinationsSpec {
    #[doc = "List of Log Analytics destinations."]
    #[serde(
        rename = "logAnalytics",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub log_analytics: Vec<LogAnalyticsDestination>,
    #[doc = "Azure Monitor Metrics destination."]
    #[serde(rename = "azureMonitorMetrics", default, skip_serializing_if = "Option::is_none")]
    pub azure_monitor_metrics: Option<serde_json::Value>,
}
impl DestinationsSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The diagnostic settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettings {
    #[doc = "The resource ID of the storage account to which you would like to send Diagnostic Logs."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The service bus rule Id of the diagnostic setting. This is here to maintain backwards compatibility."]
    #[serde(rename = "serviceBusRuleId", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_rule_id: Option<String>,
    #[doc = "The resource Id for the event hub authorization rule."]
    #[serde(rename = "eventHubAuthorizationRuleId", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_authorization_rule_id: Option<String>,
    #[doc = "The name of the event hub. If none is specified, the default event hub will be selected."]
    #[serde(rename = "eventHubName", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
    #[doc = "The list of metric settings."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metrics: Vec<MetricSettings>,
    #[doc = "The list of logs settings."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logs: Vec<LogSettings>,
    #[doc = "The full ARM resource ID of the Log Analytics workspace to which you would like to send Diagnostic Logs. Example: /subscriptions/4b9e8510-67ab-4e9a-95a9-e2f1e570ea9c/resourceGroups/insights-integration/providers/Microsoft.OperationalInsights/workspaces/viruela2"]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "A string indicating whether the export to Log Analytics should use the default destination type, i.e. AzureDiagnostics, or use a destination type constructed as follows: <normalized service identity>_<normalized category name>. Possible values are: Dedicated and null (null is default.)"]
    #[serde(rename = "logAnalyticsDestinationType", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_destination_type: Option<String>,
}
impl DiagnosticSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The diagnostic settings Category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsCategory {
    #[doc = "The type of the diagnostic settings category."]
    #[serde(rename = "categoryType", default, skip_serializing_if = "Option::is_none")]
    pub category_type: Option<diagnostic_settings_category::CategoryType>,
}
impl DiagnosticSettingsCategory {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod diagnostic_settings_category {
    use super::*;
    #[doc = "The type of the diagnostic settings category."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CategoryType {
        Metrics,
        Logs,
    }
}
#[doc = "The diagnostic settings category resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsCategoryResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "The diagnostic settings Category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticSettingsCategory>,
}
impl DiagnosticSettingsCategoryResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a collection of diagnostic setting category resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsCategoryResourceCollection {
    #[doc = "The collection of diagnostic settings category resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DiagnosticSettingsCategoryResource>,
}
impl DiagnosticSettingsCategoryResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The diagnostic setting resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "The diagnostic settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticSettings>,
}
impl DiagnosticSettingsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a collection of alert rule resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsResourceCollection {
    #[doc = "The collection of diagnostic settings resources;."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DiagnosticSettingsResource>,
}
impl DiagnosticSettingsResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the criteria for converting log to metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    #[doc = "Name of the dimension"]
    pub name: String,
    #[doc = "Operator for dimension values"]
    pub operator: dimension::Operator,
    #[doc = "List of dimension values"]
    pub values: Vec<String>,
}
impl Dimension {
    pub fn new(name: String, operator: dimension::Operator, values: Vec<String>) -> Self {
        Self { name, operator, values }
    }
}
pub mod dimension {
    use super::*;
    #[doc = "Operator for dimension values"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Include,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Include => serializer.serialize_unit_variant("Operator", 0u32, "Include"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Criterion for dynamic threshold."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicMetricCriteria {
    #[serde(flatten)]
    pub multi_metric_criteria: MultiMetricCriteria,
    #[doc = "The operator used to compare the metric value against the threshold."]
    pub operator: dynamic_metric_criteria::Operator,
    #[doc = "The extent of deviation required to trigger an alert. This will affect how tight the threshold is to the metric series pattern."]
    #[serde(rename = "alertSensitivity")]
    pub alert_sensitivity: dynamic_metric_criteria::AlertSensitivity,
    #[doc = "The minimum number of violations required within the selected lookback time window required to raise an alert."]
    #[serde(rename = "failingPeriods")]
    pub failing_periods: DynamicThresholdFailingPeriods,
    #[doc = "Use this option to set the date from which to start learning the metric historical data and calculate the dynamic thresholds (in ISO8601 format)"]
    #[serde(rename = "ignoreDataBefore", default, with = "azure_core::date::rfc3339::option")]
    pub ignore_data_before: Option<time::OffsetDateTime>,
}
impl DynamicMetricCriteria {
    pub fn new(
        multi_metric_criteria: MultiMetricCriteria,
        operator: dynamic_metric_criteria::Operator,
        alert_sensitivity: dynamic_metric_criteria::AlertSensitivity,
        failing_periods: DynamicThresholdFailingPeriods,
    ) -> Self {
        Self {
            multi_metric_criteria,
            operator,
            alert_sensitivity,
            failing_periods,
            ignore_data_before: None,
        }
    }
}
pub mod dynamic_metric_criteria {
    use super::*;
    #[doc = "The operator used to compare the metric value against the threshold."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        GreaterThan,
        LessThan,
        GreaterOrLessThan,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 0u32, "GreaterThan"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 1u32, "LessThan"),
                Self::GreaterOrLessThan => serializer.serialize_unit_variant("Operator", 2u32, "GreaterOrLessThan"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The extent of deviation required to trigger an alert. This will affect how tight the threshold is to the metric series pattern."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AlertSensitivity")]
    pub enum AlertSensitivity {
        Low,
        Medium,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AlertSensitivity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AlertSensitivity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AlertSensitivity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Low => serializer.serialize_unit_variant("AlertSensitivity", 0u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("AlertSensitivity", 1u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("AlertSensitivity", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The minimum number of violations required within the selected lookback time window required to raise an alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicThresholdFailingPeriods {
    #[doc = "The number of aggregated lookback points. The lookback time window is calculated based on the aggregation granularity (windowSize) and the selected number of aggregated points."]
    #[serde(rename = "numberOfEvaluationPeriods")]
    pub number_of_evaluation_periods: f64,
    #[doc = "The number of violations to trigger an alert. Should be smaller or equal to numberOfEvaluationPeriods."]
    #[serde(rename = "minFailingPeriodsToAlert")]
    pub min_failing_periods_to_alert: f64,
}
impl DynamicThresholdFailingPeriods {
    pub fn new(number_of_evaluation_periods: f64, min_failing_periods_to_alert: f64) -> Self {
        Self {
            number_of_evaluation_periods,
            min_failing_periods_to_alert,
        }
    }
}
#[doc = "Email notification of an autoscale event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailNotification {
    #[doc = "a value indicating whether to send email to subscription administrator."]
    #[serde(rename = "sendToSubscriptionAdministrator", default, skip_serializing_if = "Option::is_none")]
    pub send_to_subscription_administrator: Option<bool>,
    #[doc = "a value indicating whether to send email to subscription co-administrators."]
    #[serde(rename = "sendToSubscriptionCoAdministrators", default, skip_serializing_if = "Option::is_none")]
    pub send_to_subscription_co_administrators: Option<bool>,
    #[doc = "the custom e-mails list. This value can be null or empty, in which case this attribute will be ignored."]
    #[serde(
        rename = "customEmails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_emails: Vec<String>,
}
impl EmailNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An email receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailReceiver {
    #[doc = "The name of the email receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The email address of this receiver."]
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
    #[doc = "Indicates the status of the receiver. Receivers that are not Enabled will not receive any communications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
impl EmailReceiver {
    pub fn new(name: String, email_address: String) -> Self {
        Self {
            name,
            email_address,
            use_common_alert_schema: None,
            status: None,
        }
    }
}
#[doc = "Describes a receiver that should be resubscribed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableRequest {
    #[doc = "The name of the receiver to resubscribe."]
    #[serde(rename = "receiverName")]
    pub receiver_name: String,
}
impl EnableRequest {
    pub fn new(receiver_name: String) -> Self {
        Self { receiver_name }
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[doc = "Error code identifying the specific error."]
    pub code: String,
    #[doc = "Error message in the caller's locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl Error {
    pub fn new(code: String) -> Self {
        Self { code, message: None }
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorContract {
    #[doc = "Describes the format of Error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl azure_core::Continuable for ErrorContract {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseCommon {
    #[serde(flatten)]
    pub error_response: ErrorResponse,
    #[doc = "The error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorResponseCommon>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorResponseCommon {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseCommonV2 {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl azure_core::Continuable for ErrorResponseCommonV2 {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponseCommonV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of event categories. Currently possible values are: Administrative, Security, ServiceHealth, Alert, Recommendation, Policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventCategoryCollection {
    #[doc = "the list that includes the Azure event categories."]
    pub value: Vec<LocalizableString>,
}
impl azure_core::Continuable for EventCategoryCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl EventCategoryCollection {
    pub fn new(value: Vec<LocalizableString>) -> Self {
        Self { value }
    }
}
#[doc = "The Azure event log entries are of type EventData"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventData {
    #[doc = "the authorization used by the user who has performed the operation that led to this event. This captures the RBAC properties of the event. These usually include the 'action', 'role' and the 'scope'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<SenderAuthorization>,
    #[doc = "key value pairs to identify ARM permissions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    #[doc = "the email address of the user who has performed the operation, the UPN claim or SPN claim based on availability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caller: Option<String>,
    #[doc = "the description of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "the Id of this event as required by ARM for RBAC. It contains the EventDataID and a timestamp information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "the event data Id. This is a unique identifier for an event."]
    #[serde(rename = "eventDataId", default, skip_serializing_if = "Option::is_none")]
    pub event_data_id: Option<String>,
    #[doc = "the correlation Id, usually a GUID in the string format. The correlation Id is shared among the events that belong to the same uber operation."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The localizable string class."]
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<LocalizableString>,
    #[doc = "The localizable string class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<LocalizableString>,
    #[doc = "The Http request info."]
    #[serde(rename = "httpRequest", default, skip_serializing_if = "Option::is_none")]
    pub http_request: Option<HttpRequestInfo>,
    #[doc = "the event level"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<event_data::Level>,
    #[doc = "the resource group name of the impacted resource."]
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[doc = "The localizable string class."]
    #[serde(rename = "resourceProviderName", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider_name: Option<LocalizableString>,
    #[doc = "the resource uri that uniquely identifies the resource that caused this event."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The localizable string class."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<LocalizableString>,
    #[doc = "It is usually a GUID shared among the events corresponding to single operation. This value should not be confused with EventName."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "The localizable string class."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<LocalizableString>,
    #[doc = "the set of <Key, Value> pairs (usually a Dictionary<String, String>) that includes details about the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The localizable string class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<LocalizableString>,
    #[doc = "The localizable string class."]
    #[serde(rename = "subStatus", default, skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<LocalizableString>,
    #[doc = "the timestamp of when the event was generated by the Azure service processing the request corresponding the event. It in ISO 8601 format."]
    #[serde(rename = "eventTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub event_timestamp: Option<time::OffsetDateTime>,
    #[doc = "the timestamp of when the event became available for querying via this API. It is in ISO 8601 format. This value should not be confused eventTimestamp. As there might be a delay between the occurrence time of the event, and the time that the event is submitted to the Azure logging infrastructure."]
    #[serde(rename = "submissionTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub submission_timestamp: Option<time::OffsetDateTime>,
    #[doc = "the Azure subscription Id usually a GUID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "the Azure tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl EventData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod event_data {
    use super::*;
    #[doc = "the event level"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        Critical,
        Error,
        Warning,
        Informational,
        Verbose,
    }
}
#[doc = "Represents collection of events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventDataCollection {
    #[doc = "this list that includes the Azure audit logs."]
    pub value: Vec<EventData>,
    #[doc = "Provides the link to retrieve the next set of events."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EventDataCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EventDataCollection {
    pub fn new(value: Vec<EventData>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Definition of which data will be collected from a separate VM extension that integrates with the Azure Monitor Agent.\r\nCollected from either Windows and Linux machines, depending on which extension is defined."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionDataSource {
    #[doc = "List of streams that this data source will be sent to.\r\nA stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub streams: Vec<String>,
    #[doc = "The name of the VM extension."]
    #[serde(rename = "extensionName")]
    pub extension_name: String,
    #[doc = "The extension settings. The format is specific for particular extension."]
    #[serde(rename = "extensionSettings", default, skip_serializing_if = "Option::is_none")]
    pub extension_settings: Option<serde_json::Value>,
    #[doc = "The list of data sources this extension needs data from."]
    #[serde(
        rename = "inputDataSources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_data_sources: Vec<String>,
    #[doc = "A friendly name for the data source. \r\nThis name should be unique across all data sources (regardless of type) within the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ExtensionDataSource {
    pub fn new(extension_name: String) -> Self {
        Self {
            streams: Vec::new(),
            extension_name,
            extension_settings: None,
            input_data_sources: Vec::new(),
            name: None,
        }
    }
}
#[doc = "The Http request info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpRequestInfo {
    #[doc = "the client request id."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "the client Ip Address"]
    #[serde(rename = "clientIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub client_ip_address: Option<String>,
    #[doc = "the Http request method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "the Uri."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl HttpRequestInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An alert incident indicates the activation status of an alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Incident {
    #[doc = "Incident name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Rule name that is associated with the incident."]
    #[serde(rename = "ruleName", default, skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[doc = "A boolean to indicate whether the incident is active or resolved."]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "The time at which the incident was activated in ISO8601 format."]
    #[serde(rename = "activatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub activated_time: Option<time::OffsetDateTime>,
    #[doc = "The time at which the incident was resolved in ISO8601 format. If null, it means the incident is still active."]
    #[serde(rename = "resolvedTime", default, with = "azure_core::date::rfc3339::option")]
    pub resolved_time: Option<time::OffsetDateTime>,
}
impl Incident {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List incidents operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncidentListResult {
    #[doc = "the incident collection."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Incident>,
}
impl azure_core::Continuable for IncidentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl IncidentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Itsm receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItsmReceiver {
    #[doc = "The name of the Itsm receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "OMS LA instance identifier."]
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[doc = "Unique identification of ITSM connection among multiple defined in above workspace."]
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[doc = "JSON blob for the configurations of the ITSM action. CreateMultipleWorkItems option will be part of this blob as well."]
    #[serde(rename = "ticketConfiguration")]
    pub ticket_configuration: String,
    #[doc = "Region in which workspace resides. Supported values:'centralindia','japaneast','southeastasia','australiasoutheast','uksouth','westcentralus','canadacentral','eastus','westeurope'"]
    pub region: String,
}
impl ItsmReceiver {
    pub fn new(name: String, workspace_id: String, connection_id: String, ticket_configuration: String, region: String) -> Self {
        Self {
            name,
            workspace_id,
            connection_id,
            ticket_configuration,
            region,
        }
    }
}
#[doc = "The localizable string class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizableString {
    #[doc = "the invariant value."]
    pub value: String,
    #[doc = "the locale specific value."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl LocalizableString {
    pub fn new(value: String) -> Self {
        Self {
            value,
            localized_value: None,
        }
    }
}
#[doc = "A rule condition based on a certain number of locations failing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationThresholdRuleCondition {
    #[serde(flatten)]
    pub rule_condition: RuleCondition,
    #[doc = "the period of time (in ISO 8601 duration format) that is used to monitor alert activity based on the threshold. If specified then it must be between 5 minutes and 1 day."]
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[doc = "the number of locations that must fail to activate the alert."]
    #[serde(rename = "failedLocationCount")]
    pub failed_location_count: i32,
}
impl LocationThresholdRuleCondition {
    pub fn new(rule_condition: RuleCondition, failed_location_count: i32) -> Self {
        Self {
            rule_condition,
            window_size: None,
            failed_location_count,
        }
    }
}
#[doc = "Log Analytics destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogAnalyticsDestination {
    #[doc = "The resource ID of the Log Analytics workspace."]
    #[serde(rename = "workspaceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_resource_id: Option<String>,
    #[doc = "The Customer ID of the Log Analytics workspace."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "A friendly name for the destination. \r\nThis name should be unique across all destinations (regardless of type) within the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl LogAnalyticsDestination {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A log metrics trigger descriptor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogMetricTrigger {
    #[doc = "Result Condition Evaluation criteria."]
    #[serde(rename = "thresholdOperator", default, skip_serializing_if = "Option::is_none")]
    pub threshold_operator: Option<ConditionalOperator>,
    #[doc = "The threshold of the metric trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[doc = "Metric Trigger Evaluation Type"]
    #[serde(rename = "metricTriggerType", default, skip_serializing_if = "Option::is_none")]
    pub metric_trigger_type: Option<MetricTriggerType>,
    #[doc = "Evaluation of metric on a particular column"]
    #[serde(rename = "metricColumn", default, skip_serializing_if = "Option::is_none")]
    pub metric_column: Option<String>,
}
impl LogMetricTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a collection of log profiles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogProfileCollection {
    #[doc = "the values of the log profiles."]
    pub value: Vec<LogProfileResource>,
}
impl azure_core::Continuable for LogProfileCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl LogProfileCollection {
    pub fn new(value: Vec<LogProfileResource>) -> Self {
        Self { value }
    }
}
#[doc = "The log profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogProfileProperties {
    #[doc = "the resource id of the storage account to which you would like to send the Activity Log."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The service bus rule ID of the service bus namespace in which you would like to have Event Hubs created for streaming the Activity Log. The rule ID is of the format: '{service bus resource ID}/authorizationrules/{key name}'."]
    #[serde(rename = "serviceBusRuleId", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_rule_id: Option<String>,
    #[doc = "List of regions for which Activity Log events should be stored or streamed. It is a comma separated list of valid ARM locations including the 'global' location."]
    pub locations: Vec<String>,
    #[doc = "the categories of the logs. These categories are created as is convenient to the user. Some values are: 'Write', 'Delete', and/or 'Action.'"]
    pub categories: Vec<String>,
    #[doc = "Specifies the retention policy for the log."]
    #[serde(rename = "retentionPolicy")]
    pub retention_policy: RetentionPolicy,
}
impl LogProfileProperties {
    pub fn new(locations: Vec<String>, categories: Vec<String>, retention_policy: RetentionPolicy) -> Self {
        Self {
            storage_account_id: None,
            service_bus_rule_id: None,
            locations,
            categories,
            retention_policy,
        }
    }
}
#[doc = "The log profile resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogProfileResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The log profile properties."]
    pub properties: LogProfileProperties,
}
impl LogProfileResource {
    pub fn new(resource: Resource, properties: LogProfileProperties) -> Self {
        Self { resource, properties }
    }
}
#[doc = "The log profile resource for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogProfileResourcePatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The log profile properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LogProfileProperties>,
}
impl LogProfileResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Log Search Rule Definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSearchRule {
    #[doc = "The api-version used when creating this alert rule"]
    #[serde(rename = "createdWithApiVersion", default, skip_serializing_if = "Option::is_none")]
    pub created_with_api_version: Option<String>,
    #[doc = "True if alert rule is legacy Log Analytic rule"]
    #[serde(rename = "isLegacyLogAnalyticsRule", default, skip_serializing_if = "Option::is_none")]
    pub is_legacy_log_analytics_rule: Option<bool>,
    #[doc = "The description of the Log Search rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name of the alert rule"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The flag that indicates whether the alert should be automatically resolved or not. The default is false."]
    #[serde(rename = "autoMitigate", default, skip_serializing_if = "Option::is_none")]
    pub auto_mitigate: Option<bool>,
    #[doc = "The flag which indicates whether the Log Search rule is enabled. Value should be true or false"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<log_search_rule::Enabled>,
    #[doc = "Last time the rule was updated in IS08601 format."]
    #[serde(rename = "lastUpdatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "Provisioning state of the scheduled query rule"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<log_search_rule::ProvisioningState>,
    #[doc = "Specifies the log search query."]
    pub source: Source,
    #[doc = "Defines how often to run the search and the time interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[doc = "Action descriptor."]
    pub action: Action,
}
impl LogSearchRule {
    pub fn new(source: Source, action: Action) -> Self {
        Self {
            created_with_api_version: None,
            is_legacy_log_analytics_rule: None,
            description: None,
            display_name: None,
            auto_mitigate: None,
            enabled: None,
            last_updated_time: None,
            provisioning_state: None,
            source,
            schedule: None,
            action,
        }
    }
}
pub mod log_search_rule {
    use super::*;
    #[doc = "The flag which indicates whether the Log Search rule is enabled. Value should be true or false"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Enabled")]
    pub enum Enabled {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Enabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Enabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Enabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Enabled", 0u32, "true"),
                Self::False => serializer.serialize_unit_variant("Enabled", 1u32, "false"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning state of the scheduled query rule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Deploying,
        Canceled,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Deploying => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deploying"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Log Search Rule Definition for Patching"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSearchRulePatch {
    #[doc = "The flag which indicates whether the Log Search rule is enabled. Value should be true or false"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<log_search_rule_patch::Enabled>,
}
impl LogSearchRulePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod log_search_rule_patch {
    use super::*;
    #[doc = "The flag which indicates whether the Log Search rule is enabled. Value should be true or false"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Enabled")]
    pub enum Enabled {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Enabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Enabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Enabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Enabled", 0u32, "true"),
                Self::False => serializer.serialize_unit_variant("Enabled", 1u32, "false"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Log Search Rule resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSearchRuleResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Log Search Rule Definition"]
    pub properties: LogSearchRule,
}
impl LogSearchRuleResource {
    pub fn new(resource: Resource, properties: LogSearchRule) -> Self {
        Self { resource, properties }
    }
}
#[doc = "Represents a collection of Log Search rule resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSearchRuleResourceCollection {
    #[doc = "The values for the Log Search Rule resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LogSearchRuleResource>,
}
impl azure_core::Continuable for LogSearchRuleResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl LogSearchRuleResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The log search rule resource for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSearchRuleResourcePatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Log Search Rule Definition for Patching"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LogSearchRulePatch>,
}
impl LogSearchRuleResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Part of MultiTenantDiagnosticSettings. Specifies the settings for a particular log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSettings {
    #[doc = "Name of a Diagnostic Log category for a resource type this setting is applied to. To obtain the list of Diagnostic Log categories for a resource, first perform a GET diagnostic settings operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "a value indicating whether this log is enabled."]
    pub enabled: bool,
    #[doc = "Specifies the retention policy for the log."]
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl LogSettings {
    pub fn new(enabled: bool) -> Self {
        Self {
            category: None,
            enabled,
            retention_policy: None,
        }
    }
}
#[doc = "Specify action need to be taken when rule type is converting log to metric"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogToMetricAction {
    #[serde(flatten)]
    pub action: Action,
    #[doc = "Criteria of Metric"]
    pub criteria: Vec<Criteria>,
}
impl LogToMetricAction {
    pub fn new(action: Action, criteria: Vec<Criteria>) -> Self {
        Self { action, criteria }
    }
}
#[doc = "A logic app receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogicAppReceiver {
    #[doc = "The name of the logic app receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The azure resource id of the logic app receiver."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The callback url where http request sent to."]
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
impl LogicAppReceiver {
    pub fn new(name: String, resource_id: String, callback_url: String) -> Self {
        Self {
            name,
            resource_id,
            callback_url,
            use_common_alert_schema: None,
        }
    }
}
#[doc = "Definition of the endpoint used for ingesting logs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogsIngestionEndpointSpec {
    #[doc = "The endpoint. This property is READ-ONLY."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl LogsIngestionEndpointSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "How the data that is collected should be combined over time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementEventAggregationCondition {
    #[doc = "Operators allowed in the rule condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<ConditionOperator>,
    #[doc = "The threshold value that activates the alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[doc = "the period of time (in ISO 8601 duration format) that is used to monitor alert activity based on the threshold. If specified then it must be between 5 minutes and 1 day."]
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
}
impl ManagementEventAggregationCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A management event rule condition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementEventRuleCondition {
    #[serde(flatten)]
    pub rule_condition: RuleCondition,
    #[doc = "How the data that is collected should be combined over time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<ManagementEventAggregationCondition>,
}
impl ManagementEventRuleCondition {
    pub fn new(rule_condition: RuleCondition) -> Self {
        Self {
            rule_condition,
            aggregation: None,
        }
    }
}
#[doc = "Represents a metric metadata value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataValue {
    #[doc = "The localizable string class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[doc = "the value of the metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl MetadataValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result data of a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    #[doc = "the metric Id."]
    pub id: String,
    #[doc = "the resource type of the metric resource."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The localizable string class."]
    pub name: LocalizableString,
    #[doc = "Detailed description of this metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "'Success' or the error details on query failures for this metric."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Error message encountered querying this specific metric."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The unit of the metric."]
    pub unit: Unit,
    #[doc = "the time series returned when a data query is performed."]
    pub timeseries: Vec<TimeSeriesElement>,
}
impl Metric {
    pub fn new(id: String, type_: String, name: LocalizableString, unit: Unit, timeseries: Vec<TimeSeriesElement>) -> Self {
        Self {
            id,
            type_,
            name,
            display_description: None,
            error_code: None,
            error_message: None,
            unit,
            timeseries,
        }
    }
}
#[doc = "An alert action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertAction {
    #[doc = "the id of the action group to use."]
    #[serde(rename = "actionGroupId", default, skip_serializing_if = "Option::is_none")]
    pub action_group_id: Option<String>,
    #[doc = "This field allows specifying custom properties, which would be appended to the alert payload sent as input to the webhook."]
    #[serde(rename = "webHookProperties", default, skip_serializing_if = "Option::is_none")]
    pub web_hook_properties: Option<serde_json::Value>,
}
impl MetricAlertAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The rule criteria that defines the conditions of the alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertCriteria {
    #[doc = "specifies the type of the alert criteria."]
    #[serde(rename = "odata.type")]
    pub odata_type: metric_alert_criteria::OdataType,
}
impl MetricAlertCriteria {
    pub fn new(odata_type: metric_alert_criteria::OdataType) -> Self {
        Self { odata_type }
    }
}
pub mod metric_alert_criteria {
    use super::*;
    #[doc = "specifies the type of the alert criteria."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OdataType")]
    pub enum OdataType {
        #[serde(rename = "Microsoft.Azure.Monitor.SingleResourceMultipleMetricCriteria")]
        MicrosoftAzureMonitorSingleResourceMultipleMetricCriteria,
        #[serde(rename = "Microsoft.Azure.Monitor.MultipleResourceMultipleMetricCriteria")]
        MicrosoftAzureMonitorMultipleResourceMultipleMetricCriteria,
        #[serde(rename = "Microsoft.Azure.Monitor.WebtestLocationAvailabilityCriteria")]
        MicrosoftAzureMonitorWebtestLocationAvailabilityCriteria,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OdataType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OdataType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OdataType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftAzureMonitorSingleResourceMultipleMetricCriteria => {
                    serializer.serialize_unit_variant("OdataType", 0u32, "Microsoft.Azure.Monitor.SingleResourceMultipleMetricCriteria")
                }
                Self::MicrosoftAzureMonitorMultipleResourceMultipleMetricCriteria => {
                    serializer.serialize_unit_variant("OdataType", 1u32, "Microsoft.Azure.Monitor.MultipleResourceMultipleMetricCriteria")
                }
                Self::MicrosoftAzureMonitorWebtestLocationAvailabilityCriteria => {
                    serializer.serialize_unit_variant("OdataType", 2u32, "Microsoft.Azure.Monitor.WebtestLocationAvailabilityCriteria")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the metric alert criteria for multiple resource that has multiple metric criteria."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertMultipleResourceMultipleMetricCriteria {
    #[serde(flatten)]
    pub metric_alert_criteria: MetricAlertCriteria,
    #[doc = "the list of multiple metric criteria for this 'all of' operation. "]
    #[serde(
        rename = "allOf",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub all_of: Vec<MultiMetricCriteria>,
}
impl MetricAlertMultipleResourceMultipleMetricCriteria {
    pub fn new(metric_alert_criteria: MetricAlertCriteria) -> Self {
        Self {
            metric_alert_criteria,
            all_of: Vec::new(),
        }
    }
}
#[doc = "An alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertProperties {
    #[doc = "the description of the metric alert that will be included in the alert email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Alert severity {0, 1, 2, 3, 4}"]
    pub severity: i32,
    #[doc = "the flag that indicates whether the metric alert is enabled."]
    pub enabled: bool,
    #[doc = "the list of resource id's that this metric alert is scoped to."]
    pub scopes: Vec<String>,
    #[doc = "how often the metric alert is evaluated represented in ISO 8601 duration format."]
    #[serde(rename = "evaluationFrequency")]
    pub evaluation_frequency: String,
    #[doc = "the period of time (in ISO 8601 duration format) that is used to monitor alert activity based on the threshold."]
    #[serde(rename = "windowSize")]
    pub window_size: String,
    #[doc = "the resource type of the target resource(s) on which the alert is created/updated. Mandatory if the scope contains a subscription, resource group, or more than one resource."]
    #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[doc = "the region of the target resource(s) on which the alert is created/updated. Mandatory if the scope contains a subscription, resource group, or more than one resource."]
    #[serde(rename = "targetResourceRegion", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_region: Option<String>,
    #[doc = "The rule criteria that defines the conditions of the alert rule."]
    pub criteria: MetricAlertCriteria,
    #[doc = "the flag that indicates whether the alert should be auto resolved or not. The default is true."]
    #[serde(rename = "autoMitigate", default, skip_serializing_if = "Option::is_none")]
    pub auto_mitigate: Option<bool>,
    #[doc = "the array of actions that are performed when the alert rule becomes active, and when an alert condition is resolved."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<MetricAlertAction>,
    #[doc = "Last time the rule was updated in ISO8601 format."]
    #[serde(rename = "lastUpdatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "the value indicating whether this alert rule is migrated."]
    #[serde(rename = "isMigrated", default, skip_serializing_if = "Option::is_none")]
    pub is_migrated: Option<bool>,
}
impl MetricAlertProperties {
    pub fn new(
        severity: i32,
        enabled: bool,
        scopes: Vec<String>,
        evaluation_frequency: String,
        window_size: String,
        criteria: MetricAlertCriteria,
    ) -> Self {
        Self {
            description: None,
            severity,
            enabled,
            scopes,
            evaluation_frequency,
            window_size,
            target_resource_type: None,
            target_resource_region: None,
            criteria,
            auto_mitigate: None,
            actions: Vec::new(),
            last_updated_time: None,
            is_migrated: None,
        }
    }
}
#[doc = "An alert rule properties for patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertPropertiesPatch {
    #[doc = "the description of the metric alert that will be included in the alert email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Alert severity {0, 1, 2, 3, 4}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
    #[doc = "the flag that indicates whether the metric alert is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "the list of resource id's that this metric alert is scoped to."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<String>,
    #[doc = "how often the metric alert is evaluated represented in ISO 8601 duration format."]
    #[serde(rename = "evaluationFrequency", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_frequency: Option<String>,
    #[doc = "the period of time (in ISO 8601 duration format) that is used to monitor alert activity based on the threshold."]
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[doc = "the resource type of the target resource(s) on which the alert is created/updated. Mandatory for MultipleResourceMultipleMetricCriteria."]
    #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[doc = "the region of the target resource(s) on which the alert is created/updated. Mandatory for MultipleResourceMultipleMetricCriteria."]
    #[serde(rename = "targetResourceRegion", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_region: Option<String>,
    #[doc = "The rule criteria that defines the conditions of the alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria: Option<MetricAlertCriteria>,
    #[doc = "the flag that indicates whether the alert should be auto resolved or not. The default is true."]
    #[serde(rename = "autoMitigate", default, skip_serializing_if = "Option::is_none")]
    pub auto_mitigate: Option<bool>,
    #[doc = "the array of actions that are performed when the alert rule becomes active, and when an alert condition is resolved."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<MetricAlertAction>,
    #[doc = "Last time the rule was updated in ISO8601 format."]
    #[serde(rename = "lastUpdatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "the value indicating whether this alert rule is migrated."]
    #[serde(rename = "isMigrated", default, skip_serializing_if = "Option::is_none")]
    pub is_migrated: Option<bool>,
}
impl MetricAlertPropertiesPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metric alert resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "An alert rule."]
    pub properties: MetricAlertProperties,
}
impl MetricAlertResource {
    pub fn new(resource: Resource, properties: MetricAlertProperties) -> Self {
        Self { resource, properties }
    }
}
#[doc = "Represents a collection of alert rule resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertResourceCollection {
    #[doc = "the values for the alert rule resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<MetricAlertResource>,
}
impl azure_core::Continuable for MetricAlertResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricAlertResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metric alert resource for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertResourcePatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An alert rule properties for patch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetricAlertPropertiesPatch>,
}
impl MetricAlertResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the metric alert criteria for a single resource that has multiple metric criteria."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertSingleResourceMultipleMetricCriteria {
    #[serde(flatten)]
    pub metric_alert_criteria: MetricAlertCriteria,
    #[doc = "The list of metric criteria for this 'all of' operation. "]
    #[serde(
        rename = "allOf",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub all_of: Vec<MetricCriteria>,
}
impl MetricAlertSingleResourceMultipleMetricCriteria {
    pub fn new(metric_alert_criteria: MetricAlertCriteria) -> Self {
        Self {
            metric_alert_criteria,
            all_of: Vec::new(),
        }
    }
}
#[doc = "An alert status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertStatus {
    #[doc = "The status name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The alert rule arm id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The extended resource type name."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "An alert status properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetricAlertStatusProperties>,
}
impl MetricAlertStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a collection of alert rule resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertStatusCollection {
    #[doc = "the values for the alert rule resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<MetricAlertStatus>,
}
impl MetricAlertStatusCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An alert status properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertStatusProperties {
    #[doc = "An object describing the type of the dimensions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<serde_json::Value>,
    #[doc = "status value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "UTC time when the status was checked."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
}
impl MetricAlertStatusProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric availability specifies the time grain (aggregation interval or frequency) and the retention period for that time grain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailability {
    #[doc = "the time grain specifies the aggregation interval for the metric. Expressed as a duration 'PT1M', 'P1D', etc."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "the retention period for the metric at the specified timegrain.  Expressed as a duration 'PT1M', 'P1D', etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
}
impl MetricAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a metric baselines query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBaselinesProperties {
    #[doc = "The timespan for which the data was retrieved. Its value consists of two datetimes concatenated, separated by '/'.  This may be adjusted in the future and returned back from what was originally requested."]
    pub timespan: String,
    #[doc = "The interval (window size) for which the metric data was returned in.  This may be adjusted in the future and returned back from what was originally requested.  This is not present if a metadata request was made."]
    pub interval: String,
    #[doc = "The namespace of the metrics been queried."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The baseline for each time series that was queried."]
    pub baselines: Vec<TimeSeriesBaseline>,
}
impl MetricBaselinesProperties {
    pub fn new(timespan: String, interval: String, baselines: Vec<TimeSeriesBaseline>) -> Self {
        Self {
            timespan,
            interval,
            namespace: None,
            baselines,
        }
    }
}
#[doc = "A list of metric baselines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricBaselinesResponse {
    #[doc = "The list of metric baselines."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SingleMetricBaseline>,
}
impl azure_core::Continuable for MetricBaselinesResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricBaselinesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetricClass")]
pub enum MetricClass {
    Availability,
    Transactions,
    Errors,
    Latency,
    Saturation,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MetricClass {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetricClass {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetricClass {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Availability => serializer.serialize_unit_variant("MetricClass", 0u32, "Availability"),
            Self::Transactions => serializer.serialize_unit_variant("MetricClass", 1u32, "Transactions"),
            Self::Errors => serializer.serialize_unit_variant("MetricClass", 2u32, "Errors"),
            Self::Latency => serializer.serialize_unit_variant("MetricClass", 3u32, "Latency"),
            Self::Saturation => serializer.serialize_unit_variant("MetricClass", 4u32, "Saturation"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Criterion to filter metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricCriteria {
    #[serde(flatten)]
    pub multi_metric_criteria: MultiMetricCriteria,
    #[doc = "the criteria operator."]
    pub operator: metric_criteria::Operator,
    #[doc = "the criteria threshold value that activates the alert."]
    pub threshold: f64,
}
impl MetricCriteria {
    pub fn new(multi_metric_criteria: MultiMetricCriteria, operator: metric_criteria::Operator, threshold: f64) -> Self {
        Self {
            multi_metric_criteria,
            operator,
            threshold,
        }
    }
}
pub mod metric_criteria {
    use super::*;
    #[doc = "the criteria operator."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Equals,
        GreaterThan,
        GreaterThanOrEqual,
        LessThan,
        LessThanOrEqual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Equals => serializer.serialize_unit_variant("Operator", 0u32, "Equals"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 1u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 2u32, "GreaterThanOrEqual"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 3u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 4u32, "LessThanOrEqual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Metric definition class specifies the metadata for a metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDefinition {
    #[doc = "Flag to indicate whether the dimension is required."]
    #[serde(rename = "isDimensionRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_dimension_required: Option<bool>,
    #[doc = "the resource identifier of the resource that emitted the metric."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "the namespace the metric belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The localizable string class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[doc = "Detailed description of this metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Custom category name for this metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The class of the metric."]
    #[serde(rename = "metricClass", default, skip_serializing_if = "Option::is_none")]
    pub metric_class: Option<MetricClass>,
    #[doc = "The unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[doc = "the aggregation type of the metric."]
    #[serde(rename = "primaryAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub primary_aggregation_type: Option<AggregationType>,
    #[doc = "the collection of what aggregation types are supported."]
    #[serde(
        rename = "supportedAggregationTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_aggregation_types: Vec<AggregationType>,
    #[doc = "the collection of what aggregation intervals are available to be queried."]
    #[serde(
        rename = "metricAvailabilities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metric_availabilities: Vec<MetricAvailability>,
    #[doc = "the resource identifier of the metric definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "the name and the display name of the dimension, i.e. it is a localizable string."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<LocalizableString>,
}
impl MetricDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents collection of metric definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinitionCollection {
    #[doc = "the values for the metric definitions."]
    pub value: Vec<MetricDefinition>,
}
impl azure_core::Continuable for MetricDefinitionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricDefinitionCollection {
    pub fn new(value: Vec<MetricDefinition>) -> Self {
        Self { value }
    }
}
#[doc = "Specifies a metric dimension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    #[doc = "Name of the dimension."]
    pub name: String,
    #[doc = "the dimension operator. Only 'Include' and 'Exclude' are supported"]
    pub operator: String,
    #[doc = "list of dimension values."]
    pub values: Vec<String>,
}
impl MetricDimension {
    pub fn new(name: String, operator: String, values: Vec<String>) -> Self {
        Self { name, operator, values }
    }
}
#[doc = "Metric namespace class specifies the metadata for a metric namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricNamespace {
    #[doc = "The ID of the metric namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the namespace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The escaped name of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Kind of namespace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<NamespaceClassification>,
    #[doc = "The fully qualified metric namespace name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetricNamespaceName>,
}
impl MetricNamespace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents collection of metric namespaces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricNamespaceCollection {
    #[doc = "The values for the metric namespaces."]
    pub value: Vec<MetricNamespace>,
}
impl azure_core::Continuable for MetricNamespaceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricNamespaceCollection {
    pub fn new(value: Vec<MetricNamespace>) -> Self {
        Self { value }
    }
}
#[doc = "The fully qualified metric namespace name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricNamespaceName {
    #[doc = "The metric namespace name."]
    #[serde(rename = "metricNamespaceName", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace_name: Option<String>,
}
impl MetricNamespaceName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Part of MultiTenantDiagnosticSettings. Specifies the settings for a particular metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSettings {
    #[doc = "the timegrain of the metric in ISO8601 format."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "Name of a Diagnostic Metric category for a resource type this setting is applied to. To obtain the list of Diagnostic metric categories for a resource, first perform a GET diagnostic settings operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "a value indicating whether this category is enabled."]
    pub enabled: bool,
    #[doc = "Specifies the retention policy for the log."]
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl MetricSettings {
    pub fn new(enabled: bool) -> Self {
        Self {
            time_grain: None,
            category: None,
            enabled,
            retention_policy: None,
        }
    }
}
#[doc = "The metric dimension name and value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSingleDimension {
    #[doc = "Name of the dimension."]
    pub name: String,
    #[doc = "Value of the dimension."]
    pub value: String,
}
impl MetricSingleDimension {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "The trigger that results in a scaling action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTrigger {
    #[doc = "the name of the metric that defines what the rule monitors."]
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[doc = "the namespace of the metric that defines what the rule monitors."]
    #[serde(rename = "metricNamespace", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[doc = "the resource identifier of the resource the rule monitors."]
    #[serde(rename = "metricResourceUri")]
    pub metric_resource_uri: String,
    #[doc = "the location of the resource the rule monitors."]
    #[serde(rename = "metricResourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub metric_resource_location: Option<String>,
    #[doc = "the granularity of metrics the rule monitors. Must be one of the predefined values returned from metric definitions for the metric. Must be between 12 hours and 1 minute."]
    #[serde(rename = "timeGrain")]
    pub time_grain: String,
    #[doc = "the metric statistic type. How the metrics from multiple instances are combined."]
    pub statistic: metric_trigger::Statistic,
    #[doc = "the range of time in which instance data is collected. This value must be greater than the delay in metric collection, which can vary from resource-to-resource. Must be between 12 hours and 5 minutes."]
    #[serde(rename = "timeWindow")]
    pub time_window: String,
    #[doc = "time aggregation type. How the data that is collected should be combined over time. The default value is Average."]
    #[serde(rename = "timeAggregation")]
    pub time_aggregation: metric_trigger::TimeAggregation,
    #[doc = "the operator that is used to compare the metric data and the threshold."]
    pub operator: metric_trigger::Operator,
    #[doc = "the threshold of the metric that triggers the scale action."]
    pub threshold: f64,
    #[doc = "List of dimension conditions. For example: [{\"DimensionName\":\"AppName\",\"Operator\":\"Equals\",\"Values\":[\"App1\"]},{\"DimensionName\":\"Deployment\",\"Operator\":\"Equals\",\"Values\":[\"default\"]}]."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<ScaleRuleMetricDimension>,
    #[doc = "a value indicating whether metric should divide per instance."]
    #[serde(rename = "dividePerInstance", default, skip_serializing_if = "Option::is_none")]
    pub divide_per_instance: Option<bool>,
}
impl MetricTrigger {
    pub fn new(
        metric_name: String,
        metric_resource_uri: String,
        time_grain: String,
        statistic: metric_trigger::Statistic,
        time_window: String,
        time_aggregation: metric_trigger::TimeAggregation,
        operator: metric_trigger::Operator,
        threshold: f64,
    ) -> Self {
        Self {
            metric_name,
            metric_namespace: None,
            metric_resource_uri,
            metric_resource_location: None,
            time_grain,
            statistic,
            time_window,
            time_aggregation,
            operator,
            threshold,
            dimensions: Vec::new(),
            divide_per_instance: None,
        }
    }
}
pub mod metric_trigger {
    use super::*;
    #[doc = "the metric statistic type. How the metrics from multiple instances are combined."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Statistic {
        Average,
        Min,
        Max,
        Sum,
        Count,
    }
    #[doc = "time aggregation type. How the data that is collected should be combined over time. The default value is Average."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TimeAggregation {
        Average,
        Minimum,
        Maximum,
        Total,
        Count,
        Last,
    }
    #[doc = "the operator that is used to compare the metric data and the threshold."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equals,
        NotEquals,
        GreaterThan,
        GreaterThanOrEqual,
        LessThan,
        LessThanOrEqual,
    }
}
#[doc = "Metric Trigger Evaluation Type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetricTriggerType")]
pub enum MetricTriggerType {
    Consecutive,
    Total,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MetricTriggerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetricTriggerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetricTriggerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Consecutive => serializer.serialize_unit_variant("MetricTriggerType", 0u32, "Consecutive"),
            Self::Total => serializer.serialize_unit_variant("MetricTriggerType", 1u32, "Total"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for MetricTriggerType {
    fn default() -> Self {
        Self::Consecutive
    }
}
#[doc = "Represents a metric value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricValue {
    #[doc = "the timestamp for the metric value in ISO 8601 format."]
    #[serde(rename = "timeStamp", with = "azure_core::date::rfc3339")]
    pub time_stamp: time::OffsetDateTime,
    #[doc = "the average value in the time range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[doc = "the least value in the time range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[doc = "the greatest value in the time range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[doc = "the sum of all of the values in the time range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[doc = "the number of samples in the time range. Can be used to determine the number of values that contributed to the average value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
}
impl MetricValue {
    pub fn new(time_stamp: time::OffsetDateTime) -> Self {
        Self {
            time_stamp,
            average: None,
            minimum: None,
            maximum: None,
            total: None,
            count: None,
        }
    }
}
#[doc = "The types of conditions for a multi resource alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiMetricCriteria {
    #[doc = "Specifies the type of threshold criteria"]
    #[serde(rename = "criterionType")]
    pub criterion_type: multi_metric_criteria::CriterionType,
    #[doc = "Name of the criteria."]
    pub name: String,
    #[doc = "Name of the metric."]
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[doc = "Namespace of the metric."]
    #[serde(rename = "metricNamespace", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[doc = "the criteria time aggregation types."]
    #[serde(rename = "timeAggregation")]
    pub time_aggregation: multi_metric_criteria::TimeAggregation,
    #[doc = "List of dimension conditions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<MetricDimension>,
    #[doc = "Allows creating an alert rule on a custom metric that isn't yet emitted, by causing the metric validation to be skipped."]
    #[serde(rename = "skipMetricValidation", default, skip_serializing_if = "Option::is_none")]
    pub skip_metric_validation: Option<bool>,
}
impl MultiMetricCriteria {
    pub fn new(
        criterion_type: multi_metric_criteria::CriterionType,
        name: String,
        metric_name: String,
        time_aggregation: multi_metric_criteria::TimeAggregation,
    ) -> Self {
        Self {
            criterion_type,
            name,
            metric_name,
            metric_namespace: None,
            time_aggregation,
            dimensions: Vec::new(),
            skip_metric_validation: None,
        }
    }
}
pub mod multi_metric_criteria {
    use super::*;
    #[doc = "Specifies the type of threshold criteria"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CriterionType")]
    pub enum CriterionType {
        StaticThresholdCriterion,
        DynamicThresholdCriterion,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CriterionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CriterionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CriterionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StaticThresholdCriterion => serializer.serialize_unit_variant("CriterionType", 0u32, "StaticThresholdCriterion"),
                Self::DynamicThresholdCriterion => serializer.serialize_unit_variant("CriterionType", 1u32, "DynamicThresholdCriterion"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "the criteria time aggregation types."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TimeAggregation")]
    pub enum TimeAggregation {
        Average,
        Count,
        Minimum,
        Maximum,
        Total,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TimeAggregation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TimeAggregation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TimeAggregation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Average => serializer.serialize_unit_variant("TimeAggregation", 0u32, "Average"),
                Self::Count => serializer.serialize_unit_variant("TimeAggregation", 1u32, "Count"),
                Self::Minimum => serializer.serialize_unit_variant("TimeAggregation", 2u32, "Minimum"),
                Self::Maximum => serializer.serialize_unit_variant("TimeAggregation", 3u32, "Maximum"),
                Self::Total => serializer.serialize_unit_variant("TimeAggregation", 4u32, "Total"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Kind of namespace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NamespaceClassification")]
pub enum NamespaceClassification {
    Platform,
    Custom,
    Qos,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NamespaceClassification {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NamespaceClassification {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NamespaceClassification {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Platform => serializer.serialize_unit_variant("NamespaceClassification", 0u32, "Platform"),
            Self::Custom => serializer.serialize_unit_variant("NamespaceClassification", 1u32, "Custom"),
            Self::Qos => serializer.serialize_unit_variant("NamespaceClassification", 2u32, "Qos"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Definition of the network rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSet {
    #[doc = "The configuration to set whether network access from public internet to the endpoints are allowed."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<network_rule_set::PublicNetworkAccess>,
}
impl NetworkRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_rule_set {
    use super::*;
    #[doc = "The configuration to set whether network access from public internet to the endpoints are allowed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Microsoft Insights API operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Insights"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: AlertRules, Autoscale, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Microsoft.Insights operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Microsoft.Insights provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "The operation Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Start time of the job in standard ISO8601 format."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the job in standard ISO8601 format."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The resource management error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseCommon>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of which performance counters will be collected and how they will be collected by this data collection rule.\r\nCollected from both Windows and Linux machines where the counter is present."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerfCounterDataSource {
    #[doc = "List of streams that this data source will be sent to.\r\nA stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub streams: Vec<String>,
    #[doc = "The number of seconds between consecutive counter measurements (samples)."]
    #[serde(rename = "samplingFrequencyInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub sampling_frequency_in_seconds: Option<i32>,
    #[doc = "A list of specifier names of the performance counters you want to collect.\r\nUse a wildcard (*) to collect a counter for all instances.\r\nTo get a list of performance counters on Windows, run the command 'typeperf'."]
    #[serde(
        rename = "counterSpecifiers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub counter_specifiers: Vec<String>,
    #[doc = "A friendly name for the data source. \r\nThis name should be unique across all data sources (regardless of type) within the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PerfCounterDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private endpoint connections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "Private endpoint which the connection belongs to."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointProperty>,
    #[doc = "State of the private endpoint connection."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionStateProperty>,
    #[doc = "State of the private endpoint connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint which the connection belongs to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointProperty {
    #[doc = "Resource id of the private endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpointProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkScopesResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PrivateLinkScopesResource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "State of the private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionStateProperty {
    #[doc = "The private link service connection status."]
    pub status: String,
    #[doc = "The private link service connection description."]
    pub description: String,
    #[doc = "The actions required for private link service connection."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionStateProperty {
    pub fn new(status: String, description: String) -> Self {
        Self {
            status,
            description,
            actions_required: None,
        }
    }
}
#[doc = "A proxy only azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyOnlyResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ProxyOnlyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Set value to 'ResultAccount'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "QueryType")]
pub enum QueryType {
    ResultCount,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for QueryType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for QueryType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for QueryType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ResultCount => serializer.serialize_unit_variant("QueryType", 0u32, "ResultCount"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates the status of the receiver. Receivers that are not Enabled will not receive any communications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReceiverStatus {
    NotSpecified,
    Enabled,
    Disabled,
}
#[doc = "The repeating times at which this profile begins. This element is not used if the FixedDate element is used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recurrence {
    #[doc = "the recurrence frequency. How often the schedule profile should take effect. This value must be Week, meaning each week will have the same set of profiles. For example, to set a daily schedule, set **schedule** to every day of the week. The frequency property specifies that the schedule is repeated weekly."]
    pub frequency: recurrence::Frequency,
    #[doc = "The scheduling constraints for when the profile begins."]
    pub schedule: RecurrentSchedule,
}
impl Recurrence {
    pub fn new(frequency: recurrence::Frequency, schedule: RecurrentSchedule) -> Self {
        Self { frequency, schedule }
    }
}
pub mod recurrence {
    use super::*;
    #[doc = "the recurrence frequency. How often the schedule profile should take effect. This value must be Week, meaning each week will have the same set of profiles. For example, to set a daily schedule, set **schedule** to every day of the week. The frequency property specifies that the schedule is repeated weekly."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Frequency {
        None,
        Second,
        Minute,
        Hour,
        Day,
        Week,
        Month,
        Year,
    }
}
#[doc = "The scheduling constraints for when the profile begins."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurrentSchedule {
    #[doc = "the timezone for the hours of the profile. Some examples of valid time zones are: Dateline Standard Time, UTC-11, Hawaiian Standard Time, Alaskan Standard Time, Pacific Standard Time (Mexico), Pacific Standard Time, US Mountain Standard Time, Mountain Standard Time (Mexico), Mountain Standard Time, Central America Standard Time, Central Standard Time, Central Standard Time (Mexico), Canada Central Standard Time, SA Pacific Standard Time, Eastern Standard Time, US Eastern Standard Time, Venezuela Standard Time, Paraguay Standard Time, Atlantic Standard Time, Central Brazilian Standard Time, SA Western Standard Time, Pacific SA Standard Time, Newfoundland Standard Time, E. South America Standard Time, Argentina Standard Time, SA Eastern Standard Time, Greenland Standard Time, Montevideo Standard Time, Bahia Standard Time, UTC-02, Mid-Atlantic Standard Time, Azores Standard Time, Cape Verde Standard Time, Morocco Standard Time, UTC, GMT Standard Time, Greenwich Standard Time, W. Europe Standard Time, Central Europe Standard Time, Romance Standard Time, Central European Standard Time, W. Central Africa Standard Time, Namibia Standard Time, Jordan Standard Time, GTB Standard Time, Middle East Standard Time, Egypt Standard Time, Syria Standard Time, E. Europe Standard Time, South Africa Standard Time, FLE Standard Time, Turkey Standard Time, Israel Standard Time, Kaliningrad Standard Time, Libya Standard Time, Arabic Standard Time, Arab Standard Time, Belarus Standard Time, Russian Standard Time, E. Africa Standard Time, Iran Standard Time, Arabian Standard Time, Azerbaijan Standard Time, Russia Time Zone 3, Mauritius Standard Time, Georgian Standard Time, Caucasus Standard Time, Afghanistan Standard Time, West Asia Standard Time, Ekaterinburg Standard Time, Pakistan Standard Time, India Standard Time, Sri Lanka Standard Time, Nepal Standard Time, Central Asia Standard Time, Bangladesh Standard Time, N. Central Asia Standard Time, Myanmar Standard Time, SE Asia Standard Time, North Asia Standard Time, China Standard Time, North Asia East Standard Time, Singapore Standard Time, W. Australia Standard Time, Taipei Standard Time, Ulaanbaatar Standard Time, Tokyo Standard Time, Korea Standard Time, Yakutsk Standard Time, Cen. Australia Standard Time, AUS Central Standard Time, E. Australia Standard Time, AUS Eastern Standard Time, West Pacific Standard Time, Tasmania Standard Time, Magadan Standard Time, Vladivostok Standard Time, Russia Time Zone 10, Central Pacific Standard Time, Russia Time Zone 11, New Zealand Standard Time, UTC+12, Fiji Standard Time, Kamchatka Standard Time, Tonga Standard Time, Samoa Standard Time, Line Islands Standard Time"]
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[doc = "the collection of days that the profile takes effect on. Possible values are Sunday through Saturday."]
    pub days: Vec<String>,
    #[doc = "A collection of hours that the profile takes effect on. Values supported are 0 to 23 on the 24-hour clock (AM/PM times are not supported)."]
    pub hours: Vec<i32>,
    #[doc = "A collection of minutes at which the profile takes effect at."]
    pub minutes: Vec<i32>,
}
impl RecurrentSchedule {
    pub fn new(time_zone: String, days: Vec<String>, hours: Vec<i32>, minutes: Vec<i32>) -> Self {
        Self {
            time_zone,
            days,
            hours,
            minutes,
        }
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "Definition of ARM tracked top level resource properties for update operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceForUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceForUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a metrics query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Response {
    #[doc = "The integer value representing the relative cost of the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[doc = "The timespan for which the data was retrieved. Its value consists of two datetimes concatenated, separated by '/'.  This may be adjusted in the future and returned back from what was originally requested."]
    pub timespan: String,
    #[doc = "The interval (window size) for which the metric data was returned in.  This may be adjusted in the future and returned back from what was originally requested.  This is not present if a metadata request was made."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[doc = "The namespace of the metrics being queried"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The region of the resource being queried for metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resourceregion: Option<String>,
    #[doc = "the value of the collection."]
    pub value: Vec<Metric>,
}
impl Response {
    pub fn new(timespan: String, value: Vec<Metric>) -> Self {
        Self {
            cost: None,
            timespan,
            interval: None,
            namespace: None,
            resourceregion: None,
            value,
        }
    }
}
#[doc = "An error response from the API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithError {
    #[doc = "Error details."]
    pub error: Error,
}
impl ResponseWithError {
    pub fn new(error: Error) -> Self {
        Self { error }
    }
}
#[doc = "Specifies the retention policy for the log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[doc = "a value indicating whether the retention policy is enabled."]
    pub enabled: bool,
    #[doc = "the number of days for the retention in days. A value of 0 will retain the events indefinitely."]
    pub days: i32,
}
impl RetentionPolicy {
    pub fn new(enabled: bool, days: i32) -> Self {
        Self { enabled, days }
    }
}
#[doc = "The action that is performed when the alert rule becomes active, and when an alert condition is resolved."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleAction {
    #[doc = "specifies the type of the action. There are two types of actions: RuleEmailAction and RuleWebhookAction."]
    #[serde(rename = "odata.type")]
    pub odata_type: String,
}
impl RuleAction {
    pub fn new(odata_type: String) -> Self {
        Self { odata_type }
    }
}
#[doc = "The condition that results in the alert rule being activated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleCondition {
    #[doc = "specifies the type of condition. This can be one of three types: ManagementEventRuleCondition (occurrences of management events), LocationThresholdRuleCondition (based on the number of failures of a web test), and ThresholdRuleCondition (based on the threshold of a metric)."]
    #[serde(rename = "odata.type")]
    pub odata_type: String,
    #[doc = "The resource from which the rule collects its data."]
    #[serde(rename = "dataSource", default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<RuleDataSource>,
}
impl RuleCondition {
    pub fn new(odata_type: String) -> Self {
        Self {
            odata_type,
            data_source: None,
        }
    }
}
#[doc = "The resource from which the rule collects its data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleDataSource {
    #[doc = "specifies the type of data source. There are two types of rule data sources: RuleMetricDataSource and RuleManagementEventDataSource"]
    #[serde(rename = "odata.type")]
    pub odata_type: String,
    #[doc = "the resource identifier of the resource the rule monitors. **NOTE**: this property cannot be updated for an existing rule."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "the legacy resource identifier of the resource the rule monitors. **NOTE**: this property cannot be updated for an existing rule."]
    #[serde(rename = "legacyResourceId", default, skip_serializing_if = "Option::is_none")]
    pub legacy_resource_id: Option<String>,
    #[doc = "the location of the resource."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "the namespace of the metric."]
    #[serde(rename = "metricNamespace", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
}
impl RuleDataSource {
    pub fn new(odata_type: String) -> Self {
        Self {
            odata_type,
            resource_uri: None,
            legacy_resource_id: None,
            resource_location: None,
            metric_namespace: None,
        }
    }
}
#[doc = "Specifies the action to send email when the rule condition is evaluated. The discriminator is always RuleEmailAction in this case."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleEmailAction {
    #[serde(flatten)]
    pub rule_action: RuleAction,
    #[doc = "Whether the administrators (service and co-administrators) of the service should be notified when the alert is activated."]
    #[serde(rename = "sendToServiceOwners", default, skip_serializing_if = "Option::is_none")]
    pub send_to_service_owners: Option<bool>,
    #[doc = "the list of administrator's custom email addresses to notify of the activation of the alert."]
    #[serde(
        rename = "customEmails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_emails: Vec<String>,
}
impl RuleEmailAction {
    pub fn new(rule_action: RuleAction) -> Self {
        Self {
            rule_action,
            send_to_service_owners: None,
            custom_emails: Vec::new(),
        }
    }
}
#[doc = "The claims for a rule management event data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleManagementEventClaimsDataSource {
    #[doc = "the email address."]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}
impl RuleManagementEventClaimsDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A rule management event data source. The discriminator fields is always RuleManagementEventDataSource in this case."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleManagementEventDataSource {
    #[serde(flatten)]
    pub rule_data_source: RuleDataSource,
    #[doc = "the event name."]
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[doc = "the event source."]
    #[serde(rename = "eventSource", default, skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    #[doc = "the level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[doc = "The name of the operation that should be checked for. If no name is provided, any operation will match."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "the resource group name."]
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[doc = "the resource provider name."]
    #[serde(rename = "resourceProviderName", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider_name: Option<String>,
    #[doc = "The status of the operation that should be checked for. If no status is provided, any status will match."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "the substatus."]
    #[serde(rename = "subStatus", default, skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<String>,
    #[doc = "The claims for a rule management event data source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<RuleManagementEventClaimsDataSource>,
}
impl RuleManagementEventDataSource {
    pub fn new(rule_data_source: RuleDataSource) -> Self {
        Self {
            rule_data_source,
            event_name: None,
            event_source: None,
            level: None,
            operation_name: None,
            resource_group_name: None,
            resource_provider_name: None,
            status: None,
            sub_status: None,
            claims: None,
        }
    }
}
#[doc = "A rule metric data source. The discriminator value is always RuleMetricDataSource in this case."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleMetricDataSource {
    #[serde(flatten)]
    pub rule_data_source: RuleDataSource,
    #[doc = "the name of the metric that defines what the rule monitors."]
    #[serde(rename = "metricName", default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}
impl RuleMetricDataSource {
    pub fn new(rule_data_source: RuleDataSource) -> Self {
        Self {
            rule_data_source,
            metric_name: None,
        }
    }
}
#[doc = "Specifies the action to post to service when the rule condition is evaluated. The discriminator is always RuleWebhookAction in this case."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleWebhookAction {
    #[serde(flatten)]
    pub rule_action: RuleAction,
    #[doc = "the service uri to Post the notification when the alert activates or resolves."]
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[doc = "the dictionary of custom properties to include with the post operation. These data are appended to the webhook payload."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl RuleWebhookAction {
    pub fn new(rule_action: RuleAction) -> Self {
        Self {
            rule_action,
            service_uri: None,
            properties: None,
        }
    }
}
#[doc = "The parameters for the scaling action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleAction {
    #[doc = "the scale direction. Whether the scaling action increases or decreases the number of instances."]
    pub direction: scale_action::Direction,
    #[doc = "the type of action that should occur when the scale rule fires."]
    #[serde(rename = "type")]
    pub type_: scale_action::Type,
    #[doc = "the number of instances that are involved in the scaling action. This value must be 1 or greater. The default value is 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "the amount of time to wait since the last scaling action before this action occurs. It must be between 1 week and 1 minute in ISO 8601 format."]
    pub cooldown: String,
}
impl ScaleAction {
    pub fn new(direction: scale_action::Direction, type_: scale_action::Type, cooldown: String) -> Self {
        Self {
            direction,
            type_,
            value: None,
            cooldown,
        }
    }
}
pub mod scale_action {
    use super::*;
    #[doc = "the scale direction. Whether the scaling action increases or decreases the number of instances."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Direction {
        None,
        Increase,
        Decrease,
    }
    #[doc = "the type of action that should occur when the scale rule fires."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        ChangeCount,
        PercentChangeCount,
        ExactCount,
        ServiceAllowedNextValue,
    }
}
#[doc = "The number of instances that can be used during this profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleCapacity {
    #[doc = "the minimum number of instances for the resource."]
    pub minimum: String,
    #[doc = "the maximum number of instances for the resource. The actual maximum number of instances is limited by the cores that are available in the subscription."]
    pub maximum: String,
    #[doc = "the number of instances that will be set if metrics are not available for evaluation. The default is only used if the current instance count is lower than the default."]
    pub default: String,
}
impl ScaleCapacity {
    pub fn new(minimum: String, maximum: String, default: String) -> Self {
        Self { minimum, maximum, default }
    }
}
#[doc = "A rule that provide the triggers and parameters for the scaling action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleRule {
    #[doc = "The trigger that results in a scaling action."]
    #[serde(rename = "metricTrigger")]
    pub metric_trigger: MetricTrigger,
    #[doc = "The parameters for the scaling action."]
    #[serde(rename = "scaleAction")]
    pub scale_action: ScaleAction,
}
impl ScaleRule {
    pub fn new(metric_trigger: MetricTrigger, scale_action: ScaleAction) -> Self {
        Self {
            metric_trigger,
            scale_action,
        }
    }
}
#[doc = "Specifies an auto scale rule metric dimension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleRuleMetricDimension {
    #[doc = "Name of the dimension."]
    #[serde(rename = "DimensionName")]
    pub dimension_name: String,
    #[doc = "the dimension operator. Only 'Equals' and 'NotEquals' are supported. 'Equals' being equal to any of the values. 'NotEquals' being not equal to all of the values"]
    #[serde(rename = "Operator")]
    pub operator: scale_rule_metric_dimension::Operator,
    #[doc = "list of dimension values. For example: [\"App1\",\"App2\"]."]
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}
impl ScaleRuleMetricDimension {
    pub fn new(dimension_name: String, operator: scale_rule_metric_dimension::Operator, values: Vec<String>) -> Self {
        Self {
            dimension_name,
            operator,
            values,
        }
    }
}
pub mod scale_rule_metric_dimension {
    use super::*;
    #[doc = "the dimension operator. Only 'Equals' and 'NotEquals' are supported. 'Equals' being equal to any of the values. 'NotEquals' being not equal to all of the values"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Equals,
        NotEquals,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Equals => serializer.serialize_unit_variant("Operator", 0u32, "Equals"),
                Self::NotEquals => serializer.serialize_unit_variant("Operator", 1u32, "NotEquals"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines how often to run the search and the time interval."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    #[doc = "frequency (in minutes) at which rule condition should be evaluated."]
    #[serde(rename = "frequencyInMinutes")]
    pub frequency_in_minutes: i32,
    #[doc = "Time window for which data needs to be fetched for query (should be greater than or equal to frequencyInMinutes)."]
    #[serde(rename = "timeWindowInMinutes")]
    pub time_window_in_minutes: i32,
}
impl Schedule {
    pub fn new(frequency_in_minutes: i32, time_window_in_minutes: i32) -> Self {
        Self {
            frequency_in_minutes,
            time_window_in_minutes,
        }
    }
}
#[doc = "A private link scoped resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopedResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private link scoped resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScopedResourceProperties>,
}
impl ScopedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of scoped resources in a private link scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopedResourceListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ScopedResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScopedResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScopedResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link scoped resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopedResourceProperties {
    #[doc = "The resource id of the scoped Azure monitor resource."]
    #[serde(rename = "linkedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub linked_resource_id: Option<String>,
    #[doc = "State of the private endpoint connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ScopedResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "the authorization used by the user who has performed the operation that led to this event. This captures the RBAC properties of the event. These usually include the 'action', 'role' and the 'scope'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SenderAuthorization {
    #[doc = "the permissible actions. For instance: microsoft.support/supporttickets/write"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "the role of the user. For instance: Subscription Admin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "the scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl SenderAuthorization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The baseline values for a single sensitivity value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleBaseline {
    #[doc = "the sensitivity of the baseline."]
    pub sensitivity: single_baseline::Sensitivity,
    #[doc = "The low thresholds of the baseline."]
    #[serde(rename = "lowThresholds")]
    pub low_thresholds: Vec<f64>,
    #[doc = "The high thresholds of the baseline."]
    #[serde(rename = "highThresholds")]
    pub high_thresholds: Vec<f64>,
}
impl SingleBaseline {
    pub fn new(sensitivity: single_baseline::Sensitivity, low_thresholds: Vec<f64>, high_thresholds: Vec<f64>) -> Self {
        Self {
            sensitivity,
            low_thresholds,
            high_thresholds,
        }
    }
}
pub mod single_baseline {
    use super::*;
    #[doc = "the sensitivity of the baseline."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Sensitivity")]
    pub enum Sensitivity {
        Low,
        Medium,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Sensitivity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Sensitivity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Sensitivity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Low => serializer.serialize_unit_variant("Sensitivity", 0u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("Sensitivity", 1u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("Sensitivity", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The baseline results of a single metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleMetricBaseline {
    #[doc = "The metric baseline Id."]
    pub id: String,
    #[doc = "The resource type of the metric baseline resource."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The name of the metric for which the baselines were retrieved."]
    pub name: String,
    #[doc = "The response to a metric baselines query."]
    pub properties: MetricBaselinesProperties,
}
impl SingleMetricBaseline {
    pub fn new(id: String, type_: String, name: String, properties: MetricBaselinesProperties) -> Self {
        Self {
            id,
            type_,
            name,
            properties,
        }
    }
}
#[doc = "An SMS receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsReceiver {
    #[doc = "The name of the SMS receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The country code of the SMS receiver."]
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[doc = "The phone number of the SMS receiver."]
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[doc = "Indicates the status of the receiver. Receivers that are not Enabled will not receive any communications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
impl SmsReceiver {
    pub fn new(name: String, country_code: String, phone_number: String) -> Self {
        Self {
            name,
            country_code,
            phone_number,
            status: None,
        }
    }
}
#[doc = "Specifies the log search query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Source {
    #[doc = "Log search query. Required for action type - AlertingAction"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "List of  Resource referred into query"]
    #[serde(
        rename = "authorizedResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub authorized_resources: Vec<String>,
    #[doc = "The resource uri over which log search query is to be run."]
    #[serde(rename = "dataSourceId")]
    pub data_source_id: String,
    #[doc = "Set value to 'ResultAccount'"]
    #[serde(rename = "queryType", default, skip_serializing_if = "Option::is_none")]
    pub query_type: Option<QueryType>,
}
impl Source {
    pub fn new(data_source_id: String) -> Self {
        Self {
            query: None,
            authorized_resources: Vec::new(),
            data_source_id,
            query_type: None,
        }
    }
}
#[doc = "Definition of which syslog data will be collected and how it will be collected.\r\nOnly collected from Linux machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyslogDataSource {
    #[doc = "List of streams that this data source will be sent to.\r\nA stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub streams: Vec<String>,
    #[doc = "The list of facility names."]
    #[serde(
        rename = "facilityNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub facility_names: Vec<String>,
    #[doc = "The log levels to collect."]
    #[serde(
        rename = "logLevels",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub log_levels: Vec<String>,
    #[doc = "A friendly name for the data source. \r\nThis name should be unique across all data sources (regardless of type) within the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SyslogDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container holding only the Tags for a resource, allowing the user to update the tags on a PrivateLinkScope instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsResource {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A rule condition based on a metric crossing a threshold."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThresholdRuleCondition {
    #[serde(flatten)]
    pub rule_condition: RuleCondition,
    #[doc = "Operators allowed in the rule condition."]
    pub operator: ConditionOperator,
    #[doc = "the threshold value that activates the alert."]
    pub threshold: f64,
    #[doc = "the period of time (in ISO 8601 duration format) that is used to monitor alert activity based on the threshold. If specified then it must be between 5 minutes and 1 day."]
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[doc = "Aggregation operators allowed in a rule."]
    #[serde(rename = "timeAggregation", default, skip_serializing_if = "Option::is_none")]
    pub time_aggregation: Option<TimeAggregationOperator>,
}
impl ThresholdRuleCondition {
    pub fn new(rule_condition: RuleCondition, operator: ConditionOperator, threshold: f64) -> Self {
        Self {
            rule_condition,
            operator,
            threshold,
            window_size: None,
            time_aggregation: None,
        }
    }
}
#[doc = "Aggregation operators allowed in a rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TimeAggregationOperator {
    Average,
    Minimum,
    Maximum,
    Total,
    Last,
}
#[doc = "The baseline values for a single time series."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesBaseline {
    #[doc = "The aggregation type of the metric."]
    pub aggregation: String,
    #[doc = "The dimensions of this time series."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<MetricSingleDimension>,
    #[doc = "The list of timestamps of the baselines."]
    pub timestamps: Vec<time::OffsetDateTime>,
    #[doc = "The baseline values for each sensitivity."]
    pub data: Vec<SingleBaseline>,
    #[doc = "The baseline metadata values."]
    #[serde(
        rename = "metadataValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadata_values: Vec<BaselineMetadata>,
}
impl TimeSeriesBaseline {
    pub fn new(aggregation: String, timestamps: Vec<time::OffsetDateTime>, data: Vec<SingleBaseline>) -> Self {
        Self {
            aggregation,
            dimensions: Vec::new(),
            timestamps,
            data,
            metadata_values: Vec::new(),
        }
    }
}
#[doc = "A time series result type. The discriminator value is always TimeSeries in this case."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeSeriesElement {
    #[doc = "the metadata values returned if $filter was specified in the call."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadatavalues: Vec<MetadataValue>,
    #[doc = "An array of data points representing the metric values.  This is only returned if a result type of data is specified."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data: Vec<MetricValue>,
}
impl TimeSeriesElement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A specific date-time for the profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeWindow {
    #[doc = "the timezone of the start and end times for the profile. Some examples of valid time zones are: Dateline Standard Time, UTC-11, Hawaiian Standard Time, Alaskan Standard Time, Pacific Standard Time (Mexico), Pacific Standard Time, US Mountain Standard Time, Mountain Standard Time (Mexico), Mountain Standard Time, Central America Standard Time, Central Standard Time, Central Standard Time (Mexico), Canada Central Standard Time, SA Pacific Standard Time, Eastern Standard Time, US Eastern Standard Time, Venezuela Standard Time, Paraguay Standard Time, Atlantic Standard Time, Central Brazilian Standard Time, SA Western Standard Time, Pacific SA Standard Time, Newfoundland Standard Time, E. South America Standard Time, Argentina Standard Time, SA Eastern Standard Time, Greenland Standard Time, Montevideo Standard Time, Bahia Standard Time, UTC-02, Mid-Atlantic Standard Time, Azores Standard Time, Cape Verde Standard Time, Morocco Standard Time, UTC, GMT Standard Time, Greenwich Standard Time, W. Europe Standard Time, Central Europe Standard Time, Romance Standard Time, Central European Standard Time, W. Central Africa Standard Time, Namibia Standard Time, Jordan Standard Time, GTB Standard Time, Middle East Standard Time, Egypt Standard Time, Syria Standard Time, E. Europe Standard Time, South Africa Standard Time, FLE Standard Time, Turkey Standard Time, Israel Standard Time, Kaliningrad Standard Time, Libya Standard Time, Arabic Standard Time, Arab Standard Time, Belarus Standard Time, Russian Standard Time, E. Africa Standard Time, Iran Standard Time, Arabian Standard Time, Azerbaijan Standard Time, Russia Time Zone 3, Mauritius Standard Time, Georgian Standard Time, Caucasus Standard Time, Afghanistan Standard Time, West Asia Standard Time, Ekaterinburg Standard Time, Pakistan Standard Time, India Standard Time, Sri Lanka Standard Time, Nepal Standard Time, Central Asia Standard Time, Bangladesh Standard Time, N. Central Asia Standard Time, Myanmar Standard Time, SE Asia Standard Time, North Asia Standard Time, China Standard Time, North Asia East Standard Time, Singapore Standard Time, W. Australia Standard Time, Taipei Standard Time, Ulaanbaatar Standard Time, Tokyo Standard Time, Korea Standard Time, Yakutsk Standard Time, Cen. Australia Standard Time, AUS Central Standard Time, E. Australia Standard Time, AUS Eastern Standard Time, West Pacific Standard Time, Tasmania Standard Time, Magadan Standard Time, Vladivostok Standard Time, Russia Time Zone 10, Central Pacific Standard Time, Russia Time Zone 11, New Zealand Standard Time, UTC+12, Fiji Standard Time, Kamchatka Standard Time, Tonga Standard Time, Samoa Standard Time, Line Islands Standard Time"]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "the start time for the profile in ISO 8601 format."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub start: time::OffsetDateTime,
    #[doc = "the end time for the profile in ISO 8601 format."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub end: time::OffsetDateTime,
}
impl TimeWindow {
    pub fn new(start: time::OffsetDateTime, end: time::OffsetDateTime) -> Self {
        Self {
            time_zone: None,
            start,
            end,
        }
    }
}
#[doc = "The condition that results in the Log Search rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerCondition {
    #[doc = "Result Condition Evaluation criteria."]
    #[serde(rename = "thresholdOperator")]
    pub threshold_operator: ConditionalOperator,
    #[doc = "Result or count threshold based on which rule should be triggered."]
    pub threshold: f64,
    #[doc = "A log metrics trigger descriptor."]
    #[serde(rename = "metricTrigger", default, skip_serializing_if = "Option::is_none")]
    pub metric_trigger: Option<LogMetricTrigger>,
}
impl TriggerCondition {
    pub fn new(threshold_operator: ConditionalOperator, threshold: f64) -> Self {
        Self {
            threshold_operator,
            threshold,
            metric_trigger: None,
        }
    }
}
#[doc = "The unit of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Unit")]
pub enum Unit {
    Count,
    Bytes,
    Seconds,
    CountPerSecond,
    BytesPerSecond,
    Percent,
    MilliSeconds,
    ByteSeconds,
    Unspecified,
    Cores,
    MilliCores,
    NanoCores,
    BitsPerSecond,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Unit {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
            Self::Bytes => serializer.serialize_unit_variant("Unit", 1u32, "Bytes"),
            Self::Seconds => serializer.serialize_unit_variant("Unit", 2u32, "Seconds"),
            Self::CountPerSecond => serializer.serialize_unit_variant("Unit", 3u32, "CountPerSecond"),
            Self::BytesPerSecond => serializer.serialize_unit_variant("Unit", 4u32, "BytesPerSecond"),
            Self::Percent => serializer.serialize_unit_variant("Unit", 5u32, "Percent"),
            Self::MilliSeconds => serializer.serialize_unit_variant("Unit", 6u32, "MilliSeconds"),
            Self::ByteSeconds => serializer.serialize_unit_variant("Unit", 7u32, "ByteSeconds"),
            Self::Unspecified => serializer.serialize_unit_variant("Unit", 8u32, "Unspecified"),
            Self::Cores => serializer.serialize_unit_variant("Unit", 9u32, "Cores"),
            Self::MilliCores => serializer.serialize_unit_variant("Unit", 10u32, "MilliCores"),
            Self::NanoCores => serializer.serialize_unit_variant("Unit", 11u32, "NanoCores"),
            Self::BitsPerSecond => serializer.serialize_unit_variant("Unit", 12u32, "BitsPerSecond"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "VM Insights onboarding status for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmInsightsOnboardingStatus {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<vm_insights_onboarding_status::Properties>,
}
impl VmInsightsOnboardingStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vm_insights_onboarding_status {
    use super::*;
    #[doc = "Resource properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Azure Resource Manager identifier of the resource whose onboarding status is being represented."]
        #[serde(rename = "resourceId")]
        pub resource_id: String,
        #[doc = "The onboarding status for the resource. Note that, a higher level scope, e.g., resource group or subscription, is considered onboarded if at least one resource under it is onboarded."]
        #[serde(rename = "onboardingStatus")]
        pub onboarding_status: properties::OnboardingStatus,
        #[doc = "The status of VM Insights data from the resource. When reported as `present` the data array will contain information about the data containers to which data for the specified resource is being routed."]
        #[serde(rename = "dataStatus")]
        pub data_status: properties::DataStatus,
        #[doc = "Containers that currently store VM Insights data for the specified resource."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub data: Vec<DataContainer>,
    }
    impl Properties {
        pub fn new(resource_id: String, onboarding_status: properties::OnboardingStatus, data_status: properties::DataStatus) -> Self {
            Self {
                resource_id,
                onboarding_status,
                data_status,
                data: Vec::new(),
            }
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The onboarding status for the resource. Note that, a higher level scope, e.g., resource group or subscription, is considered onboarded if at least one resource under it is onboarded."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "OnboardingStatus")]
        pub enum OnboardingStatus {
            #[serde(rename = "onboarded")]
            Onboarded,
            #[serde(rename = "notOnboarded")]
            NotOnboarded,
            #[serde(rename = "unknown")]
            Unknown,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for OnboardingStatus {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for OnboardingStatus {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for OnboardingStatus {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Onboarded => serializer.serialize_unit_variant("OnboardingStatus", 0u32, "onboarded"),
                    Self::NotOnboarded => serializer.serialize_unit_variant("OnboardingStatus", 1u32, "notOnboarded"),
                    Self::Unknown => serializer.serialize_unit_variant("OnboardingStatus", 2u32, "unknown"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "The status of VM Insights data from the resource. When reported as `present` the data array will contain information about the data containers to which data for the specified resource is being routed."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "DataStatus")]
        pub enum DataStatus {
            #[serde(rename = "present")]
            Present,
            #[serde(rename = "notPresent")]
            NotPresent,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for DataStatus {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for DataStatus {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for DataStatus {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Present => serializer.serialize_unit_variant("DataStatus", 0u32, "present"),
                    Self::NotPresent => serializer.serialize_unit_variant("DataStatus", 1u32, "notPresent"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "A voice receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoiceReceiver {
    #[doc = "The name of the voice receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The country code of the voice receiver."]
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[doc = "The phone number of the voice receiver."]
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
}
impl VoiceReceiver {
    pub fn new(name: String, country_code: String, phone_number: String) -> Self {
        Self {
            name,
            country_code,
            phone_number,
        }
    }
}
#[doc = "Webhook notification of an autoscale event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookNotification {
    #[doc = "the service address to receive the notification."]
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[doc = "a property bag of settings. This value can be empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl WebhookNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A webhook receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookReceiver {
    #[doc = "The name of the webhook receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The URI where webhooks should be sent."]
    #[serde(rename = "serviceUri")]
    pub service_uri: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
    #[doc = "Indicates whether or not use AAD authentication."]
    #[serde(rename = "useAadAuth", default, skip_serializing_if = "Option::is_none")]
    pub use_aad_auth: Option<bool>,
    #[doc = "Indicates the webhook app object Id for aad auth."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Indicates the identifier uri for aad auth."]
    #[serde(rename = "identifierUri", default, skip_serializing_if = "Option::is_none")]
    pub identifier_uri: Option<String>,
    #[doc = "Indicates the tenant id for aad auth."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl WebhookReceiver {
    pub fn new(name: String, service_uri: String) -> Self {
        Self {
            name,
            service_uri,
            use_common_alert_schema: None,
            use_aad_auth: None,
            object_id: None,
            identifier_uri: None,
            tenant_id: None,
        }
    }
}
#[doc = "Specifies the metric alert rule criteria for a web test resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebtestLocationAvailabilityCriteria {
    #[serde(flatten)]
    pub metric_alert_criteria: MetricAlertCriteria,
    #[doc = "The Application Insights web test Id."]
    #[serde(rename = "webTestId")]
    pub web_test_id: String,
    #[doc = "The Application Insights resource Id."]
    #[serde(rename = "componentId")]
    pub component_id: String,
    #[doc = "The number of failed locations."]
    #[serde(rename = "failedLocationCount")]
    pub failed_location_count: f64,
}
impl WebtestLocationAvailabilityCriteria {
    pub fn new(metric_alert_criteria: MetricAlertCriteria, web_test_id: String, component_id: String, failed_location_count: f64) -> Self {
        Self {
            metric_alert_criteria,
            web_test_id,
            component_id,
            failed_location_count,
        }
    }
}
#[doc = "Definition of which Windows Event Log events will be collected and how they will be collected.\r\nOnly collected from Windows machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsEventLogDataSource {
    #[doc = "List of streams that this data source will be sent to.\r\nA stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub streams: Vec<String>,
    #[doc = "A list of Windows Event Log queries in XPATH format."]
    #[serde(
        rename = "xPathQueries",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub x_path_queries: Vec<String>,
    #[doc = "A friendly name for the data source. \r\nThis name should be unique across all data sources (regardless of type) within the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl WindowsEventLogDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a Log Analytics Workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    #[doc = "Azure Resource Manager identifier of the Log Analytics Workspace."]
    pub id: String,
    #[doc = "Location of the Log Analytics workspace."]
    pub location: String,
    #[doc = "Resource properties."]
    pub properties: workspace_info::Properties,
}
impl WorkspaceInfo {
    pub fn new(id: String, location: String, properties: workspace_info::Properties) -> Self {
        Self { id, location, properties }
    }
}
pub mod workspace_info {
    use super::*;
    #[doc = "Resource properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Log Analytics workspace identifier."]
        #[serde(rename = "customerId")]
        pub customer_id: String,
    }
    impl Properties {
        pub fn new(customer_id: String) -> Self {
            Self { customer_id }
        }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
