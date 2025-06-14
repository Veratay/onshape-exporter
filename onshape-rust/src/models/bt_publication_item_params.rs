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
pub struct BtPublicationItemParams {
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(rename = "encodedConfiguration", skip_serializing_if = "Option::is_none")]
    pub encoded_configuration: Option<String>,
    #[serde(rename = "isApplication", skip_serializing_if = "Option::is_none")]
    pub is_application: Option<bool>,
    #[serde(rename = "isAssembly", skip_serializing_if = "Option::is_none")]
    pub is_assembly: Option<bool>,
    #[serde(rename = "isBlob", skip_serializing_if = "Option::is_none")]
    pub is_blob: Option<bool>,
    #[serde(rename = "isWholePartStudio", skip_serializing_if = "Option::is_none")]
    pub is_whole_part_studio: Option<bool>,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "partId", skip_serializing_if = "Option::is_none")]
    pub part_id: Option<String>,
    #[serde(rename = "partName", skip_serializing_if = "Option::is_none")]
    pub part_name: Option<String>,
    #[serde(rename = "partNumber", skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(rename = "revisionId", skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl BtPublicationItemParams {
    pub fn new() -> BtPublicationItemParams {
        BtPublicationItemParams {
            data_type: None,
            document_id: None,
            element_id: None,
            encoded_configuration: None,
            is_application: None,
            is_assembly: None,
            is_blob: None,
            is_whole_part_studio: None,
            mime_type: None,
            part_id: None,
            part_name: None,
            part_number: None,
            revision: None,
            revision_id: None,
            version_id: None,
        }
    }
}

