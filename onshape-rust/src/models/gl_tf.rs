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
pub struct GlTf {
    #[serde(rename = "accessors", skip_serializing_if = "Option::is_none")]
    pub accessors: Option<Vec<models::Accessor>>,
    #[serde(rename = "animations", skip_serializing_if = "Option::is_none")]
    pub animations: Option<Vec<models::Animation>>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<Box<models::Asset>>,
    #[serde(rename = "bufferViews", skip_serializing_if = "Option::is_none")]
    pub buffer_views: Option<Vec<models::BufferView>>,
    #[serde(rename = "buffers", skip_serializing_if = "Option::is_none")]
    pub buffers: Option<Vec<models::Buffer>>,
    #[serde(rename = "cameras", skip_serializing_if = "Option::is_none")]
    pub cameras: Option<Vec<models::Camera>>,
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "extensionsRequired", skip_serializing_if = "Option::is_none")]
    pub extensions_required: Option<Vec<String>>,
    #[serde(rename = "extensionsUsed", skip_serializing_if = "Option::is_none")]
    pub extensions_used: Option<Vec<String>>,
    #[serde(rename = "extras", skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<models::Image>>,
    #[serde(rename = "materials", skip_serializing_if = "Option::is_none")]
    pub materials: Option<Vec<models::Material>>,
    #[serde(rename = "meshes", skip_serializing_if = "Option::is_none")]
    pub meshes: Option<Vec<models::Mesh>>,
    #[serde(rename = "nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<models::Node>>,
    #[serde(rename = "samplers", skip_serializing_if = "Option::is_none")]
    pub samplers: Option<Vec<models::Sampler>>,
    #[serde(rename = "scene", skip_serializing_if = "Option::is_none")]
    pub scene: Option<i32>,
    #[serde(rename = "scenes", skip_serializing_if = "Option::is_none")]
    pub scenes: Option<Vec<models::Scene>>,
    #[serde(rename = "skins", skip_serializing_if = "Option::is_none")]
    pub skins: Option<Vec<models::Skin>>,
    #[serde(rename = "textures", skip_serializing_if = "Option::is_none")]
    pub textures: Option<Vec<models::Texture>>,
}

impl GlTf {
    pub fn new() -> GlTf {
        GlTf {
            accessors: None,
            animations: None,
            asset: None,
            buffer_views: None,
            buffers: None,
            cameras: None,
            extensions: None,
            extensions_required: None,
            extensions_used: None,
            extras: None,
            images: None,
            materials: None,
            meshes: None,
            nodes: None,
            samplers: None,
            scene: None,
            scenes: None,
            skins: None,
            textures: None,
        }
    }
}

