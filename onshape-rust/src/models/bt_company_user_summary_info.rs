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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "jsonType")]
pub enum BtCompanyUserSummaryInfo {
    #[serde(rename="companyuser")]
    BtCompanyUserSummaryInfo {
        /// URI to fetch complete information of the resource.
        #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
        href: Option<String>,
        /// Id of the resource.
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        /// Name of the resource.
        #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// URI to visualize the resource in a webclient if applicable.
        #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
        view_ref: Option<String>,
        #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
        image: Option<String>,
        #[serde(rename = "isOnshapeSupport", skip_serializing_if = "Option::is_none")]
        is_onshape_support: Option<bool>,
        #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
        state: Option<i32>,
        #[serde(rename = "documentationName", skip_serializing_if = "Option::is_none")]
        documentation_name: Option<String>,
        #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
        email: Option<String>,
        #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
        first_name: Option<String>,
        #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
        company: Option<Box<models::BtCompanySummaryInfo>>,
        #[serde(rename = "documentationNameOverride", skip_serializing_if = "Option::is_none")]
        documentation_name_override: Option<String>,
        #[serde(rename = "globalPermissions", skip_serializing_if = "Option::is_none")]
        global_permissions: Option<Box<models::GlobalPermissionInfo>>,
        #[serde(rename = "invitationState", skip_serializing_if = "Option::is_none")]
        invitation_state: Option<i32>,
        #[serde(rename = "isGuest", skip_serializing_if = "Option::is_none")]
        is_guest: Option<bool>,
        #[serde(rename = "isLight", skip_serializing_if = "Option::is_none")]
        is_light: Option<bool>,
        #[serde(rename = "lastLoginTime", skip_serializing_if = "Option::is_none")]
        last_login_time: Option<String>,
        #[serde(rename = "personalMessageAllowed", skip_serializing_if = "Option::is_none")]
        personal_message_allowed: Option<bool>,
        #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
        source: Option<i32>,
        #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
        admin: Option<bool>,
        #[serde(rename = "cls", skip_serializing_if = "Option::is_none")]
        cls: Option<String>,
    },
    #[serde(rename="companyuseradmin")]
    BtCompanyUserSummaryAdminInfo {
        /// URI to fetch complete information of the resource.
        #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
        href: Option<String>,
        /// Id of the resource.
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        /// Name of the resource.
        #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// URI to visualize the resource in a webclient if applicable.
        #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
        view_ref: Option<String>,
        #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
        image: Option<String>,
        #[serde(rename = "isOnshapeSupport", skip_serializing_if = "Option::is_none")]
        is_onshape_support: Option<bool>,
        #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
        state: Option<i32>,
        #[serde(rename = "documentationName", skip_serializing_if = "Option::is_none")]
        documentation_name: Option<String>,
        #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
        email: Option<String>,
        #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
        first_name: Option<String>,
        #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
        company: Option<Box<models::BtCompanySummaryInfo>>,
        #[serde(rename = "documentationNameOverride", skip_serializing_if = "Option::is_none")]
        documentation_name_override: Option<String>,
        #[serde(rename = "globalPermissions", skip_serializing_if = "Option::is_none")]
        global_permissions: Option<Box<models::GlobalPermissionInfo>>,
        #[serde(rename = "invitationState", skip_serializing_if = "Option::is_none")]
        invitation_state: Option<i32>,
        #[serde(rename = "isGuest", skip_serializing_if = "Option::is_none")]
        is_guest: Option<bool>,
        #[serde(rename = "isLight", skip_serializing_if = "Option::is_none")]
        is_light: Option<bool>,
        #[serde(rename = "lastLoginTime", skip_serializing_if = "Option::is_none")]
        last_login_time: Option<String>,
        #[serde(rename = "personalMessageAllowed", skip_serializing_if = "Option::is_none")]
        personal_message_allowed: Option<bool>,
        #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
        source: Option<i32>,
        #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
        admin: Option<bool>,
        #[serde(rename = "cls", skip_serializing_if = "Option::is_none")]
        cls: Option<String>,
    },
}

impl Default for BtCompanyUserSummaryInfo {
    fn default() -> Self {
        Self::BtCompanyUserSummaryInfo {
            href: Default::default(),
            id: Default::default(),
            name: Default::default(),
            view_ref: Default::default(),
            image: Default::default(),
            is_onshape_support: Default::default(),
            state: Default::default(),
            documentation_name: Default::default(),
            email: Default::default(),
            first_name: Default::default(),
            last_name: Default::default(),
            company: Default::default(),
            documentation_name_override: Default::default(),
            global_permissions: Default::default(),
            invitation_state: Default::default(),
            is_guest: Default::default(),
            is_light: Default::default(),
            last_login_time: Default::default(),
            personal_message_allowed: Default::default(),
            source: Default::default(),
            admin: Default::default(),
            cls: Default::default(),
        }
        
    }
}


