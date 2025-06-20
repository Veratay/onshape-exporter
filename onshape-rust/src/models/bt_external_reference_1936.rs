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
pub struct BtExternalReference1936 {
    /// Type of JSON object.
    #[serde(rename = "btType", skip_serializing_if = "Option::is_none")]
    pub bt_type: Option<String>,
    #[serde(rename = "configured", skip_serializing_if = "Option::is_none")]
    pub configured: Option<bool>,
    #[serde(rename = "documentVersionId", skip_serializing_if = "Option::is_none")]
    pub document_version_id: Option<String>,
    #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(rename = "externalDocumentWithVersion", skip_serializing_if = "Option::is_none")]
    pub external_document_with_version: Option<Box<models::BtDocumentWithVersionId>>,
    #[serde(rename = "externalDocumentWithVersionAndElementId", skip_serializing_if = "Option::is_none")]
    pub external_document_with_version_and_element_id: Option<Box<models::BtDocumentWithVersionAndElementId>>,
    #[serde(rename = "externalReference", skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<bool>,
    #[serde(rename = "fullElementId", skip_serializing_if = "Option::is_none")]
    pub full_element_id: Option<Box<models::BtFullElementId756>>,
    #[serde(rename = "microversionIdAndConfiguration", skip_serializing_if = "Option::is_none")]
    pub microversion_id_and_configuration: Option<Box<models::BtMicroversionIdAndConfiguration2338>>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl BtExternalReference1936 {
    pub fn new() -> BtExternalReference1936 {
        BtExternalReference1936 {
            bt_type: None,
            configured: None,
            document_version_id: None,
            element_id: None,
            external_document_with_version: None,
            external_document_with_version_and_element_id: None,
            external_reference: None,
            full_element_id: None,
            microversion_id_and_configuration: None,
            node_id: None,
        }
    }
}

