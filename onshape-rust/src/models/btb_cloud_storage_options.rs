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

/// BtbCloudStorageOptions : Options for exporting elements to cloud storage.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BtbCloudStorageOptions {
    /// Folder id where to store the exported model.
    #[serde(rename = "cloudObjectId")]
    pub cloud_object_id: String,
    /// Account id to access the cloud storage.
    #[serde(rename = "cloudStorageAccountId")]
    pub cloud_storage_account_id: String,
}

impl BtbCloudStorageOptions {
    /// Options for exporting elements to cloud storage.
    pub fn new(cloud_object_id: String, cloud_storage_account_id: String) -> BtbCloudStorageOptions {
        BtbCloudStorageOptions {
            cloud_object_id,
            cloud_storage_account_id,
        }
    }
}

