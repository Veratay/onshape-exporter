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
pub struct BtmParameterEnum145 {
    /// Type of JSON object.
    #[serde(rename = "btType", skip_serializing_if = "Option::is_none")]
    pub bt_type: Option<String>,
    /// Element microversion that is being imported.
    #[serde(rename = "importMicroversion", skip_serializing_if = "Option::is_none")]
    pub import_microversion: Option<String>,
    /// ID of the parameter's node.
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// Unique ID of the parameter.
    #[serde(rename = "parameterId", skip_serializing_if = "Option::is_none")]
    pub parameter_id: Option<String>,
    #[serde(rename = "enumName", skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl BtmParameterEnum145 {
    pub fn new() -> BtmParameterEnum145 {
        BtmParameterEnum145 {
            bt_type: None,
            import_microversion: None,
            node_id: None,
            parameter_id: None,
            enum_name: None,
            namespace: None,
            value: None,
        }
    }
}

