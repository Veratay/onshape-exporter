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
pub struct BtDocumentWithVersionAndElementId {
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "documentVersionId", skip_serializing_if = "Option::is_none")]
    pub document_version_id: Option<String>,
    #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(rename = "elementLibraryId", skip_serializing_if = "Option::is_none")]
    pub element_library_id: Option<Box<models::ObjectId>>,
    #[serde(rename = "elementLibraryVersion", skip_serializing_if = "Option::is_none")]
    pub element_library_version: Option<Box<models::ObjectId>>,
    #[serde(rename = "partNumber", skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(rename = "uniqueVersionId", skip_serializing_if = "Option::is_none")]
    pub unique_version_id: Option<String>,
    #[serde(rename = "validElementLibraryReference", skip_serializing_if = "Option::is_none")]
    pub valid_element_library_reference: Option<bool>,
    #[serde(rename = "validRevisionReference", skip_serializing_if = "Option::is_none")]
    pub valid_revision_reference: Option<bool>,
}

impl BtDocumentWithVersionAndElementId {
    pub fn new() -> BtDocumentWithVersionAndElementId {
        BtDocumentWithVersionAndElementId {
            document_id: None,
            document_version_id: None,
            element_id: None,
            element_library_id: None,
            element_library_version: None,
            part_number: None,
            revision: None,
            unique_version_id: None,
            valid_element_library_reference: None,
            valid_revision_reference: None,
        }
    }
}

