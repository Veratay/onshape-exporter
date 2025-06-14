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
pub struct BtCommentParams {
    #[serde(rename = "assemblyFeature", skip_serializing_if = "Option::is_none")]
    pub assembly_feature: Option<String>,
    /// Assign the comment during creation. Comments cannot be reassigned during an update at this time.
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Box<models::CoordinatesParams>>,
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "elementFeature", skip_serializing_if = "Option::is_none")]
    pub element_feature: Option<String>,
    #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(rename = "elementOccurrence", skip_serializing_if = "Option::is_none")]
    pub element_occurrence: Option<String>,
    #[serde(rename = "elementQuery", skip_serializing_if = "Option::is_none")]
    pub element_query: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "objectId", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "objectType", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<i32>,
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "viewData", skip_serializing_if = "Option::is_none")]
    pub view_data: Option<Box<models::BtViewDataParams>>,
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl BtCommentParams {
    pub fn new() -> BtCommentParams {
        BtCommentParams {
            assembly_feature: None,
            assignee: None,
            coordinates: None,
            document_id: None,
            element_feature: None,
            element_id: None,
            element_occurrence: None,
            element_query: None,
            id: None,
            message: None,
            object_id: None,
            object_type: None,
            parent_id: None,
            version_id: None,
            view_data: None,
            workspace_id: None,
        }
    }
}

