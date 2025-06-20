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
pub struct BtmSketchConstraint2 {
    /// Type of JSON object.
    #[serde(rename = "btType", skip_serializing_if = "Option::is_none")]
    pub bt_type: Option<String>,
    /// Element microversion that is being imported.
    #[serde(rename = "importMicroversion", skip_serializing_if = "Option::is_none")]
    pub import_microversion: Option<String>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "combinedSketchEntityType", skip_serializing_if = "Option::is_none")]
    pub combined_sketch_entity_type: Option<Box<models::CombinedSketchEntityType>>,
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "entityIdAndReplaceInDependentFields", skip_serializing_if = "Option::is_none")]
    pub entity_id_and_replace_in_dependent_fields: Option<String>,
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<models::BtmParameter1>>,
    #[serde(rename = "constraintType", skip_serializing_if = "Option::is_none")]
    pub constraint_type: Option<models::GbtConstraintType>,
    #[serde(rename = "drivenDimension", skip_serializing_if = "Option::is_none")]
    pub driven_dimension: Option<bool>,
    #[serde(rename = "hasOffsetData1", skip_serializing_if = "Option::is_none")]
    pub has_offset_data1: Option<bool>,
    #[serde(rename = "hasOffsetData2", skip_serializing_if = "Option::is_none")]
    pub has_offset_data2: Option<bool>,
    #[serde(rename = "hasPierceParameter", skip_serializing_if = "Option::is_none")]
    pub has_pierce_parameter: Option<bool>,
    #[serde(rename = "helpParameters", skip_serializing_if = "Option::is_none")]
    pub help_parameters: Option<Vec<f64>>,
    #[serde(rename = "offsetDistance1", skip_serializing_if = "Option::is_none")]
    pub offset_distance1: Option<f64>,
    #[serde(rename = "offsetDistance2", skip_serializing_if = "Option::is_none")]
    pub offset_distance2: Option<f64>,
    #[serde(rename = "offsetOrientation1", skip_serializing_if = "Option::is_none")]
    pub offset_orientation1: Option<bool>,
    #[serde(rename = "offsetOrientation2", skip_serializing_if = "Option::is_none")]
    pub offset_orientation2: Option<bool>,
    #[serde(rename = "pierceParameter", skip_serializing_if = "Option::is_none")]
    pub pierce_parameter: Option<f64>,
}

impl BtmSketchConstraint2 {
    pub fn new() -> BtmSketchConstraint2 {
        BtmSketchConstraint2 {
            bt_type: None,
            import_microversion: None,
            node_id: None,
            combined_sketch_entity_type: None,
            entity_id: None,
            entity_id_and_replace_in_dependent_fields: None,
            index: None,
            namespace: None,
            parameters: None,
            constraint_type: None,
            driven_dimension: None,
            has_offset_data1: None,
            has_offset_data2: None,
            has_pierce_parameter: None,
            help_parameters: None,
            offset_distance1: None,
            offset_distance2: None,
            offset_orientation1: None,
            offset_orientation2: None,
            pierce_parameter: None,
        }
    }
}

