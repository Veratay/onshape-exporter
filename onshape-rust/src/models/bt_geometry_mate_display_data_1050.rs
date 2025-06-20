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
pub struct BtGeometryMateDisplayData1050 {
    /// Type of JSON object.
    #[serde(rename = "btType", skip_serializing_if = "Option::is_none")]
    pub bt_type: Option<String>,
    #[serde(rename = "firstDeterministicId", skip_serializing_if = "Option::is_none")]
    pub first_deterministic_id: Option<String>,
    #[serde(rename = "firstOccurrence", skip_serializing_if = "Option::is_none")]
    pub first_occurrence: Option<Box<models::BtOccurrence74>>,
    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(rename = "isDerivedFeature", skip_serializing_if = "Option::is_none")]
    pub is_derived_feature: Option<bool>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<models::BtCoordinateSystem387>>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "ownerOccurrence", skip_serializing_if = "Option::is_none")]
    pub owner_occurrence: Option<Box<models::BtOccurrence74>>,
    #[serde(rename = "secondDeterministicId", skip_serializing_if = "Option::is_none")]
    pub second_deterministic_id: Option<String>,
    #[serde(rename = "secondOccurrence", skip_serializing_if = "Option::is_none")]
    pub second_occurrence: Option<Box<models::BtOccurrence74>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::GbtAssemblyFeatureDisplayStatus>,
}

impl BtGeometryMateDisplayData1050 {
    pub fn new() -> BtGeometryMateDisplayData1050 {
        BtGeometryMateDisplayData1050 {
            bt_type: None,
            first_deterministic_id: None,
            first_occurrence: None,
            hidden: None,
            is_derived_feature: None,
            location: None,
            node_id: None,
            owner_occurrence: None,
            second_deterministic_id: None,
            second_occurrence: None,
            status: None,
        }
    }
}

