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


/// struct for typed errors of method [`copy_element_from_source_document`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopyElementFromSourceDocumentError {
    DefaultResponse(models::BtDocumentElementInfo),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`decode_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecodeConfigurationError {
    DefaultResponse(models::BtConfigurationInfo),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_element`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteElementError {
    DefaultResponse(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`encode_configuration_map`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EncodeConfigurationMapError {
    DefaultResponse(models::BtEncodedConfigurationInfo),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetConfigurationError {
    DefaultResponse(models::BtConfigurationResponse2019),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_element_translator_formats_by_version_or_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetElementTranslatorFormatsByVersionOrWorkspaceError {
    DefaultResponse(Vec<models::BtModelFormatInfo>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateConfigurationError {
    DefaultResponse(models::BtConfigurationResponse2019),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_references`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateReferencesError {
    DefaultResponse(serde_json::Value),
    UnknownValue(serde_json::Value),
}


/// Specify the target document and workspace in the URL. Specify the source document, workspace, and element in the request body.  If `anchorElementId` is specified, the copied element will be inserted after the anchor element. If not specified, the copied element will be inserted at the end of the tab list.  
pub async fn copy_element_from_source_document(configuration: &configuration::Configuration, did: &str, wid: &str, bt_copy_element_params: models::BtCopyElementParams) -> Result<models::BtDocumentElementInfo, Error<CopyElementFromSourceDocumentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_did = did;
    let p_wid = wid;
    let p_bt_copy_element_params = bt_copy_element_params;

    let uri_str = format!("{}/elements/copyelement/{did}/workspace/{wid}", configuration.base_path, did=crate::apis::urlencode(p_did), wid=crate::apis::urlencode(p_wid));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_bt_copy_element_params);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BtDocumentElementInfo`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BtDocumentElementInfo`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CopyElementFromSourceDocumentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Decode a configuration string into its original JSON form to obtain configuration parameter ID and value. See the [Configuration API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for additional details.  
pub async fn decode_configuration(configuration: &configuration::Configuration, did: &str, wvm: &str, wvmid: &str, eid: &str, cid: &str, link_document_id: Option<&str>, include_display: Option<bool>, configuration_is_id: Option<bool>) -> Result<models::BtConfigurationInfo, Error<DecodeConfigurationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_did = did;
    let p_wvm = wvm;
    let p_wvmid = wvmid;
    let p_eid = eid;
    let p_cid = cid;
    let p_link_document_id = link_document_id;
    let p_include_display = include_display;
    let p_configuration_is_id = configuration_is_id;

    let uri_str = format!("{}/elements/d/{did}/{wvm}/{wvmid}/e/{eid}/configurationencodings/{cid}", configuration.base_path, did=crate::apis::urlencode(p_did), wvm=crate::apis::urlencode(p_wvm), wvmid=crate::apis::urlencode(p_wvmid), eid=crate::apis::urlencode(p_eid), cid=crate::apis::urlencode(p_cid));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_link_document_id {
        req_builder = req_builder.query(&[("linkDocumentId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_display {
        req_builder = req_builder.query(&[("includeDisplay", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_configuration_is_id {
        req_builder = req_builder.query(&[("configurationIsId", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BtConfigurationInfo`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BtConfigurationInfo`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DecodeConfigurationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Attempting to delete the last element in a document will result in an error.
pub async fn delete_element(configuration: &configuration::Configuration, did: &str, wid: &str, eid: &str) -> Result<serde_json::Value, Error<DeleteElementError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_did = did;
    let p_wid = wid;
    let p_eid = eid;

    let uri_str = format!("{}/elements/d/{did}/w/{wid}/e/{eid}", configuration.base_path, did=crate::apis::urlencode(p_did), wid=crate::apis::urlencode(p_wid), eid=crate::apis::urlencode(p_eid));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteElementError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a configuration string in the following form:  `configuration=parameterId%3DparameterValue`  The configuration string can be used in other Onshape API calls to specify which configuration option to use. See the [Configuration API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for additional details.  
pub async fn encode_configuration_map(configuration: &configuration::Configuration, did: &str, eid: &str, bt_configuration_params: models::BtConfigurationParams, version_id: Option<&str>, link_document_id: Option<&str>) -> Result<models::BtEncodedConfigurationInfo, Error<EncodeConfigurationMapError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_did = did;
    let p_eid = eid;
    let p_bt_configuration_params = bt_configuration_params;
    let p_version_id = version_id;
    let p_link_document_id = link_document_id;

    let uri_str = format!("{}/elements/d/{did}/e/{eid}/configurationencodings", configuration.base_path, did=crate::apis::urlencode(p_did), eid=crate::apis::urlencode(p_eid));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_version_id {
        req_builder = req_builder.query(&[("versionId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_link_document_id {
        req_builder = req_builder.query(&[("linkDocumentId", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_bt_configuration_params);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BtEncodedConfigurationInfo`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BtEncodedConfigurationInfo`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EncodeConfigurationMapError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Use Configurations to create variations of elements. You can configure feature and parameter values, part properties, custom part properties, face and part appearances, and sketch text. Each Part Studio can have only one Configuration, but it can contain multiple Configuration inputs.  See the [Configuration API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for additional details. 
pub async fn get_configuration(configuration: &configuration::Configuration, did: &str, wvm: &str, wvmid: &str, eid: &str, link_document_id: Option<&str>) -> Result<models::BtConfigurationResponse2019, Error<GetConfigurationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_did = did;
    let p_wvm = wvm;
    let p_wvmid = wvmid;
    let p_eid = eid;
    let p_link_document_id = link_document_id;

    let uri_str = format!("{}/elements/d/{did}/{wvm}/{wvmid}/e/{eid}/configuration", configuration.base_path, did=crate::apis::urlencode(p_did), wvm=crate::apis::urlencode(p_wvm), wvmid=crate::apis::urlencode(p_wvmid), eid=crate::apis::urlencode(p_eid));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_link_document_id {
        req_builder = req_builder.query(&[("linkDocumentId", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BtConfigurationResponse2019`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BtConfigurationResponse2019`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetConfigurationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// See the [Translation API Guide](https://onshape-public.github.io/docs/api-adv/translation/) for additional details. 
pub async fn get_element_translator_formats_by_version_or_workspace(configuration: &configuration::Configuration, did: &str, wv: &str, wvid: &str, eid: &str, link_document_id: Option<&str>, check_content: Option<bool>, configuration2: Option<&str>) -> Result<Vec<models::BtModelFormatInfo>, Error<GetElementTranslatorFormatsByVersionOrWorkspaceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_did = did;
    let p_wv = wv;
    let p_wvid = wvid;
    let p_eid = eid;
    let p_link_document_id = link_document_id;
    let p_check_content = check_content;
    let p_configuration = configuration2;

    let uri_str = format!("{}/elements/translatorFormats/{did}/{wv}/{wvid}/{eid}", configuration.base_path, did=crate::apis::urlencode(p_did), wv=crate::apis::urlencode(p_wv), wvid=crate::apis::urlencode(p_wvid), eid=crate::apis::urlencode(p_eid));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_link_document_id {
        req_builder = req_builder.query(&[("linkDocumentId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_check_content {
        req_builder = req_builder.query(&[("checkContent", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_configuration {
        req_builder = req_builder.query(&[("configuration", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::BtModelFormatInfo&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::BtModelFormatInfo&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetElementTranslatorFormatsByVersionOrWorkspaceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// See the [Configuration API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for additional details
pub async fn update_configuration(configuration: &configuration::Configuration, did: &str, wvm: &str, wvmid: &str, eid: &str, bt_configuration_update_call2933: Option<models::BtConfigurationUpdateCall2933>) -> Result<models::BtConfigurationResponse2019, Error<UpdateConfigurationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_did = did;
    let p_wvm = wvm;
    let p_wvmid = wvmid;
    let p_eid = eid;
    let p_bt_configuration_update_call2933 = bt_configuration_update_call2933;

    let uri_str = format!("{}/elements/d/{did}/{wvm}/{wvmid}/e/{eid}/configuration", configuration.base_path, did=crate::apis::urlencode(p_did), wvm=crate::apis::urlencode(p_wvm), wvmid=crate::apis::urlencode(p_wvmid), eid=crate::apis::urlencode(p_eid));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_bt_configuration_update_call2933);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BtConfigurationResponse2019`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BtConfigurationResponse2019`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateConfigurationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_references(configuration: &configuration::Configuration, did: &str, wid: &str, eid: &str, bt_update_reference_params: models::BtUpdateReferenceParams) -> Result<serde_json::Value, Error<UpdateReferencesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_did = did;
    let p_wid = wid;
    let p_eid = eid;
    let p_bt_update_reference_params = bt_update_reference_params;

    let uri_str = format!("{}/elements/d/{did}/w/{wid}/e/{eid}/updatereferences", configuration.base_path, did=crate::apis::urlencode(p_did), wid=crate::apis::urlencode(p_wid), eid=crate::apis::urlencode(p_eid));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_bt_update_reference_params);

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
        let entity: Option<UpdateReferencesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

