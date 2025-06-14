/*
 * Onshape REST API
 *
 * ## Welcome to the Onshape REST API Explorer  To use this API explorer, sign in to your [Onshape](https://cad.onshape.com) account in another tab, then click the **Try it out** button below (it toggles to a **Cancel** button when selected).  See the **[API Explorer Guide](https://onshape-public.github.io/docs/api-intro/explorer/)** for help navigating this API Explorer, including **[authentication](https://onshape-public.github.io/docs/api-intro/explorer/#authentication)**.  **Tip:** To ensure the current session isn't used when trying other authentication techniques, make sure to [remove the Onshape cookie](https://support.google.com/chrome/answer/95647#zippy=%2Cdelete-cookies-from-a-site) as per the instructions for your browser. Alternatively, you can use a private or incognito window.  ## See Also  * [Onshape API Guide](https://onshape-public.github.io/docs/): Our full suite of developer guides, to be used as an accompaniment to this API Explorer. * [Onshape Developer Portal](https://cad.onshape.com/appstore/dev-portal): The Onshape portal for managing your API keys, OAuth2 credentials, your Onshape applications, and your Onshape App Store entries. * [Authentication Guide](https://onshape-public.github.io/docs/auth/): Our guide to using API keys, request signatures, and OAuth2 in  your Onshape applications.
 *
 * The version of the OpenAPI document: 1.198.56658-8969b956ea73
 * Contact: api-support@onshape.zendesk.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BtmModel141 {
    /// Type of JSON object.
    #[serde(rename = "btType", skip_serializing_if = "Option::is_none")]
    pub bt_type: Option<String>,
    /// Microversion that resulted from the import.
    #[serde(rename = "importMicroversion", skip_serializing_if = "Option::is_none")]
    pub import_microversion: Option<String>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "allFeatures", skip_serializing_if = "Option::is_none")]
    pub all_features: Option<Vec<models::BtmFeature134>>,
    #[serde(rename = "allFeaturesAndOtherReferences", skip_serializing_if = "Option::is_none")]
    pub all_features_and_other_references: Option<Vec<models::BtmFeature134>>,
    #[serde(rename = "allFeaturesAndSubFeatures", skip_serializing_if = "Option::is_none")]
    pub all_features_and_sub_features: Option<Vec<models::BtmFeature134>>,
    #[serde(rename = "childNodeIdToIndex", skip_serializing_if = "Option::is_none")]
    pub child_node_id_to_index: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "configurableTreeNodes", skip_serializing_if = "Option::is_none")]
    pub configurable_tree_nodes: Option<Vec<models::BtConfigurableTreeNode>>,
    #[serde(rename = "configurationData", skip_serializing_if = "Option::is_none")]
    pub configuration_data: Option<Box<models::BtmConfigurationData1560>>,
    #[serde(rename = "configured", skip_serializing_if = "Option::is_none")]
    pub configured: Option<bool>,
    #[serde(rename = "deepImports", skip_serializing_if = "Option::is_none")]
    pub deep_imports: Option<std::collections::HashMap<String, Vec<models::BtImport>>>,
    #[serde(rename = "defaultFeatures", skip_serializing_if = "Option::is_none")]
    pub default_features: Option<Box<models::BtDefaultFeatures119>>,
    #[serde(rename = "defaultUnits", skip_serializing_if = "Option::is_none")]
    pub default_units: Option<Box<models::BtmUnitsDefault160>>,
    #[serde(rename = "featureImports", skip_serializing_if = "Option::is_none")]
    pub feature_imports: Option<std::collections::HashMap<String, Vec<models::BtImport>>>,
    #[serde(rename = "firstRollbackIndex", skip_serializing_if = "Option::is_none")]
    pub first_rollback_index: Option<i32>,
    #[serde(rename = "importSet", skip_serializing_if = "Option::is_none")]
    pub import_set: Option<Vec<models::BtpModuleId235>>,
    #[serde(rename = "imports", skip_serializing_if = "Option::is_none")]
    pub imports: Option<Vec<models::BtmImport136>>,
    #[serde(rename = "isVariableStudio", skip_serializing_if = "Option::is_none")]
    pub is_variable_studio: Option<bool>,
    #[serde(rename = "lastFeatureBeforeRollBack", skip_serializing_if = "Option::is_none")]
    pub last_feature_before_roll_back: Option<Box<models::BtmFeature134>>,
    #[serde(rename = "modelAnnotations", skip_serializing_if = "Option::is_none")]
    pub model_annotations: Option<Box<models::BtModelAnnotations3945>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "partProperties", skip_serializing_if = "Option::is_none")]
    pub part_properties: Option<Box<models::BtPartProperties293>>,
    #[serde(rename = "pathToCache", skip_serializing_if = "Option::is_none")]
    pub path_to_cache: Option<Box<models::BtCacheDataPath191>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<models::BtModelProperties1258>>,
    #[serde(rename = "rollbackBar", skip_serializing_if = "Option::is_none")]
    pub rollback_bar: Option<Box<models::BtmRollback150>>,
    #[serde(rename = "rolledBackToEnd", skip_serializing_if = "Option::is_none")]
    pub rolled_back_to_end: Option<bool>,
    #[serde(rename = "variableStudios", skip_serializing_if = "Option::is_none")]
    pub variable_studios: Option<Vec<models::BtmVariableStudioReference2764>>,
}

impl BtmModel141 {
    pub fn new() -> BtmModel141 {
        BtmModel141 {
            bt_type: None,
            import_microversion: None,
            node_id: None,
            all_features: None,
            all_features_and_other_references: None,
            all_features_and_sub_features: None,
            child_node_id_to_index: None,
            configurable_tree_nodes: None,
            configuration_data: None,
            configured: None,
            deep_imports: None,
            default_features: None,
            default_units: None,
            feature_imports: None,
            first_rollback_index: None,
            import_set: None,
            imports: None,
            is_variable_studio: None,
            last_feature_before_roll_back: None,
            model_annotations: None,
            name: None,
            part_properties: None,
            path_to_cache: None,
            properties: None,
            rollback_bar: None,
            rolled_back_to_end: None,
            variable_studios: None,
        }
    }
}

