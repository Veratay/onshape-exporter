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
pub struct BtAclEntryInfo {
    #[serde(rename = "acceptOwnerTransfer", skip_serializing_if = "Option::is_none")]
    pub accept_owner_transfer: Option<bool>,
    #[serde(rename = "companyName", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "connectionId", skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "connectionName", skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "connectionUser", skip_serializing_if = "Option::is_none")]
    pub connection_user: Option<bool>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "enterpriseMember", skip_serializing_if = "Option::is_none")]
    pub enterprise_member: Option<bool>,
    #[serde(rename = "entryId", skip_serializing_if = "Option::is_none")]
    pub entry_id: Option<String>,
    #[serde(rename = "entryState", skip_serializing_if = "Option::is_none")]
    pub entry_state: Option<models::BtUserState>,
    #[serde(rename = "entryType", skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<i32>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "objectId", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "pendingOwnerTransfer", skip_serializing_if = "Option::is_none")]
    pub pending_owner_transfer: Option<bool>,
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<i64>,
    #[serde(rename = "permissionSet", skip_serializing_if = "Option::is_none")]
    pub permission_set: Option<Vec<String>>,
    #[serde(rename = "proCompanySubtype", skip_serializing_if = "Option::is_none")]
    pub pro_company_subtype: Option<i32>,
    #[serde(rename = "teamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
}

impl BtAclEntryInfo {
    pub fn new() -> BtAclEntryInfo {
        BtAclEntryInfo {
            accept_owner_transfer: None,
            company_name: None,
            connection_id: None,
            connection_name: None,
            connection_user: None,
            email: None,
            enterprise_member: None,
            entry_id: None,
            entry_state: None,
            entry_type: None,
            image: None,
            name: None,
            object_id: None,
            pending_owner_transfer: None,
            permission: None,
            permission_set: None,
            pro_company_subtype: None,
            team_name: None,
        }
    }
}

