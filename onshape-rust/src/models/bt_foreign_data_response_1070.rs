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
pub struct BtForeignDataResponse1070 {
    /// Type of JSON object.
    #[serde(rename = "btType", skip_serializing_if = "Option::is_none")]
    pub bt_type: Option<String>,
    #[serde(rename = "bucketName", skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "bucketPath", skip_serializing_if = "Option::is_none")]
    pub bucket_path: Option<String>,
    #[serde(rename = "cacheChunkList", skip_serializing_if = "Option::is_none")]
    pub cache_chunk_list: Option<Vec<String>>,
    #[serde(rename = "dataId", skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<models::GbtDataItemFormat>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "storageType", skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "useLocalStorage", skip_serializing_if = "Option::is_none")]
    pub use_local_storage: Option<bool>,
}

impl BtForeignDataResponse1070 {
    pub fn new() -> BtForeignDataResponse1070 {
        BtForeignDataResponse1070 {
            bt_type: None,
            bucket_name: None,
            bucket_path: None,
            cache_chunk_list: None,
            data_id: None,
            format: None,
            name: None,
            region: None,
            size: None,
            storage_type: None,
            use_local_storage: None,
        }
    }
}

