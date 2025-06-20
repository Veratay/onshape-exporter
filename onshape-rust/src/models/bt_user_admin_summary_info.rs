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
pub struct BtUserAdminSummaryInfo {
    #[serde(rename = "jsonType")]
    pub json_type: String,
    /// URI to fetch complete information of the resource.
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Id of the resource.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the resource.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URI to visualize the resource in a webclient if applicable.
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "isOnshapeSupport", skip_serializing_if = "Option::is_none")]
    pub is_onshape_support: Option<bool>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    #[serde(rename = "documentationName", skip_serializing_if = "Option::is_none")]
    pub documentation_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<Box<models::BtCompanySummaryInfo>>,
    #[serde(rename = "documentationNameOverride", skip_serializing_if = "Option::is_none")]
    pub documentation_name_override: Option<String>,
    #[serde(rename = "globalPermissions", skip_serializing_if = "Option::is_none")]
    pub global_permissions: Option<Box<models::GlobalPermissionInfo>>,
    #[serde(rename = "invitationState", skip_serializing_if = "Option::is_none")]
    pub invitation_state: Option<i32>,
    #[serde(rename = "isGuest", skip_serializing_if = "Option::is_none")]
    pub is_guest: Option<bool>,
    #[serde(rename = "isLight", skip_serializing_if = "Option::is_none")]
    pub is_light: Option<bool>,
    #[serde(rename = "lastLoginTime", skip_serializing_if = "Option::is_none")]
    pub last_login_time: Option<String>,
    #[serde(rename = "personalMessageAllowed", skip_serializing_if = "Option::is_none")]
    pub personal_message_allowed: Option<bool>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
    #[serde(rename = "activePlanId", skip_serializing_if = "Option::is_none")]
    pub active_plan_id: Option<String>,
    #[serde(rename = "billingUpdateRequired", skip_serializing_if = "Option::is_none")]
    pub billing_update_required: Option<bool>,
    #[serde(rename = "companyRoles", skip_serializing_if = "Option::is_none")]
    pub company_roles: Option<Vec<models::CompanyRole>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "forumId", skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<String>,
    #[serde(rename = "systemUser", skip_serializing_if = "Option::is_none")]
    pub system_user: Option<bool>,
    #[serde(rename = "totpEnabled", skip_serializing_if = "Option::is_none")]
    pub totp_enabled: Option<bool>,
}

impl BtUserAdminSummaryInfo {
    pub fn new(json_type: String) -> BtUserAdminSummaryInfo {
        BtUserAdminSummaryInfo {
            json_type,
            href: None,
            id: None,
            name: None,
            view_ref: None,
            image: None,
            is_onshape_support: None,
            state: None,
            documentation_name: None,
            email: None,
            first_name: None,
            last_name: None,
            company: None,
            documentation_name_override: None,
            global_permissions: None,
            invitation_state: None,
            is_guest: None,
            is_light: None,
            last_login_time: None,
            personal_message_allowed: None,
            source: None,
            active_plan_id: None,
            billing_update_required: None,
            company_roles: None,
            created_at: None,
            forum_id: None,
            system_user: None,
            totp_enabled: None,
        }
    }
}

