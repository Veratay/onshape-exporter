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
pub struct BtAssociativeDataInfo {
    #[serde(rename = "associativeDataId", skip_serializing_if = "Option::is_none")]
    pub associative_data_id: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::BtNameValuePair>>,
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "documentMicroversion", skip_serializing_if = "Option::is_none")]
    pub document_microversion: Option<String>,
    #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "idTag", skip_serializing_if = "Option::is_none")]
    pub id_tag: Option<String>,
    #[serde(rename = "microversionId", skip_serializing_if = "Option::is_none")]
    pub microversion_id: Option<String>,
    #[serde(rename = "occurrenceId", skip_serializing_if = "Option::is_none")]
    pub occurrence_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::GbtAppElementAssociativeDataType>,
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl BtAssociativeDataInfo {
    pub fn new() -> BtAssociativeDataInfo {
        BtAssociativeDataInfo {
            associative_data_id: None,
            data: None,
            document_id: None,
            document_microversion: None,
            element_id: None,
            error: None,
            id_tag: None,
            microversion_id: None,
            occurrence_id: None,
            r#type: None,
            version_id: None,
        }
    }
}

