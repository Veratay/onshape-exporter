/*
 * Onshape REST API
 *
 * ## Welcome to the Onshape REST API Explorer  To use this API explorer, sign in to your [Onshape](https://cad.onshape.com) account in another tab, then click the **Try it out** button below (it toggles to a **Cancel** button when selected).  See the **[API Explorer Guide](https://onshape-public.github.io/docs/api-intro/explorer/)** for help navigating this API Explorer, including **[authentication](https://onshape-public.github.io/docs/api-intro/explorer/#authentication)**.  **Tip:** To ensure the current session isn't used when trying other authentication techniques, make sure to [remove the Onshape cookie](https://support.google.com/chrome/answer/95647#zippy=%2Cdelete-cookies-from-a-site) as per the instructions for your browser. Alternatively, you can use a private or incognito window.  ## See Also  * [Onshape API Guide](https://onshape-public.github.io/docs/): Our full suite of developer guides, to be used as an accompaniment to this API Explorer. * [Onshape Developer Portal](https://cad.onshape.com/appstore/dev-portal): The Onshape portal for managing your API keys, OAuth2 credentials, your Onshape applications, and your Onshape App Store entries. * [Authentication Guide](https://onshape-public.github.io/docs/auth/): Our guide to using API keys, request signatures, and OAuth2 in  your Onshape applications.
 *
 * The version of the OpenAPI document: 1.198.56658-8969b956ea73
 * Contact: api-support@onshape.zendesk.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`get_user_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserSettingsError {
    DefaultResponse(models::BtUserSettingsInfo),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_settings_current_logged_in_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserSettingsCurrentLoggedInUserError {
    DefaultResponse(models::BtUserSettingsInfo),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`session`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionError {
    DefaultResponse(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`session_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionInfoError {
    DefaultResponse(models::BtUserOAuth2SummaryInfo),
    UnknownValue(serde_json::Value),
}


/// * Mouse button settings are contained in `reverseScrollWheelZoomDirection` and `viewManipulationMouseKeyMapping`.  * For each action in `viewManipulationMouseKeyMapping`, an array of modifier key/mouse combos is provided that performs that action.  * Possible modifier keys include `SHIFT` and `CTRL`.  * Possible mouse buttons include `MMB` (middle mouse button), `RMB` (right mouse button), and `SCROLLWHEEL`.  * Scrolling forward zooms in, unless `reverseScrollWheelZoomDirection` is set to `true`.
pub async fn get_user_settings(configuration: &configuration::Configuration, uid: &str, includematerials: Option<bool>) -> Result<models::BtUserSettingsInfo, Error<GetUserSettingsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_uid = uid;
    let p_includematerials = includematerials;

    let uri_str = format!("{}/users/{uid}/settings", configuration.base_path, uid=crate::apis::urlencode(p_uid));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_includematerials {
        req_builder = req_builder.query(&[("includematerials", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BtUserSettingsInfo`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BtUserSettingsInfo`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetUserSettingsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// * Non-admins can call this API for their own user ID.  * Mouse button settings are contained in `reverseScrollWheelZoomDirection` and `viewManipulationMouseKeyMapping`.  * For each action in `viewManipulationMouseKeyMapping`, an array of modifier key/mouse combos is provided that performs that action.  * Possible modifier keys include `SHIFT` and `CTRL`.  * Possible mouse buttons include `MMB` (middle mouse button), `RMB` (right mouse button), and `SCROLLWHEEL`.  * Scrolling forward zooms in, unless `reverseScrollWheelZoomDirection` is set to `true`.
pub async fn get_user_settings_current_logged_in_user(configuration: &configuration::Configuration, includematerials: Option<bool>) -> Result<models::BtUserSettingsInfo, Error<GetUserSettingsCurrentLoggedInUserError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_includematerials = includematerials;

    let uri_str = format!("{}/users/settings", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_includematerials {
        req_builder = req_builder.query(&[("includematerials", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BtUserSettingsInfo`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BtUserSettingsInfo`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetUserSettingsCurrentLoggedInUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returned information depends on caller's `OAuth2ReadPll` scope.
pub async fn session(configuration: &configuration::Configuration, bt_login_params: models::BtLoginParams) -> Result<serde_json::Value, Error<SessionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_bt_login_params = bt_login_params;

    let uri_str = format!("{}/users/session", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_bt_login_params);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SessionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returned information depends on caller's `OAuth2ReadPll` scope.
pub async fn session_info(configuration: &configuration::Configuration, ) -> Result<models::BtUserOAuth2SummaryInfo, Error<SessionInfoError>> {

    let uri_str = format!("{}/users/sessioninfo", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BtUserOAuth2SummaryInfo`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BtUserOAuth2SummaryInfo`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SessionInfoError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

