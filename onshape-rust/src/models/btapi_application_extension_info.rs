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
pub struct BtapiApplicationExtensionInfo {
    #[serde(rename = "actionBody", skip_serializing_if = "Option::is_none")]
    pub action_body: Option<String>,
    #[serde(rename = "actionType", skip_serializing_if = "Option::is_none")]
    pub action_type: Option<i32>,
    #[serde(rename = "actionUrl", skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "extensionContext", skip_serializing_if = "Option::is_none")]
    pub extension_context: Option<i64>,
    #[serde(rename = "extensionLocation", skip_serializing_if = "Option::is_none")]
    pub extension_location: Option<i64>,
    #[serde(rename = "hasIcon", skip_serializing_if = "Option::is_none")]
    pub has_icon: Option<bool>,
    #[serde(rename = "hasPendingIcon", skip_serializing_if = "Option::is_none")]
    pub has_pending_icon: Option<bool>,
    /// URI to fetch complete information of the resource.
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Id of the resource.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the resource.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parentAppPrimaryFormat", skip_serializing_if = "Option::is_none")]
    pub parent_app_primary_format: Option<String>,
    #[serde(rename = "showResponse", skip_serializing_if = "Option::is_none")]
    pub show_response: Option<bool>,
    /// URI to visualize the resource in a webclient if applicable.
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "visibilityRule", skip_serializing_if = "Option::is_none")]
    pub visibility_rule: Option<String>,
}

impl BtapiApplicationExtensionInfo {
    pub fn new() -> BtapiApplicationExtensionInfo {
        BtapiApplicationExtensionInfo {
            action_body: None,
            action_type: None,
            action_url: None,
            application_id: None,
            client_id: None,
            description: None,
            extension_context: None,
            extension_location: None,
            has_icon: None,
            has_pending_icon: None,
            href: None,
            icon_url: None,
            id: None,
            name: None,
            parent_app_primary_format: None,
            show_response: None,
            view_ref: None,
            visibility_rule: None,
        }
    }
}

