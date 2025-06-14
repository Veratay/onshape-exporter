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
#[serde(tag = "btType")]
pub enum BtpNode7 {
    #[serde(rename="BTPAnnotation-231")]
    BtpAnnotation231 {
        #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
        atomic: Option<bool>,
        #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
        documentation_type: Option<models::GbtpDefinitionType>,
        #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
        end_source_location: Option<i32>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
        short_descriptor: Option<String>,
        #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
        space_after: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
        space_before: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
        space_default: Option<bool>,
        #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
        start_source_location: Option<i32>,
    },
    #[serde(rename="BTPArgumentDeclaration-232")]
    BtpArgumentDeclaration232 {
        #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
        atomic: Option<bool>,
        #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
        documentation_type: Option<models::GbtpDefinitionType>,
        #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
        end_source_location: Option<i32>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
        short_descriptor: Option<String>,
        #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
        space_after: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
        space_before: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
        space_default: Option<bool>,
        #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
        start_source_location: Option<i32>,
    },
    #[serde(rename="BTPBuiltinIdentifier-233")]
    BtpBuiltinIdentifier233 {
        #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
        atomic: Option<bool>,
        #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
        documentation_type: Option<models::GbtpDefinitionType>,
        #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
        end_source_location: Option<i32>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
        short_descriptor: Option<String>,
        #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
        space_after: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
        space_before: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
        space_default: Option<bool>,
        #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
        start_source_location: Option<i32>,
    },
    #[serde(rename="BTPLValue-249")]
    BtplValue249 {
        #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
        atomic: Option<bool>,
        #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
        documentation_type: Option<models::GbtpDefinitionType>,
        #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
        end_source_location: Option<i32>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
        short_descriptor: Option<String>,
        #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
        space_after: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
        space_before: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
        space_default: Option<bool>,
        #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
        start_source_location: Option<i32>,
    },
    #[serde(rename="BTPLiteralMapEntry-257")]
    BtpLiteralMapEntry257 {
        #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
        atomic: Option<bool>,
        #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
        documentation_type: Option<models::GbtpDefinitionType>,
        #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
        end_source_location: Option<i32>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
        short_descriptor: Option<String>,
        #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
        space_after: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
        space_before: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
        space_default: Option<bool>,
        #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
        start_source_location: Option<i32>,
    },
    #[serde(rename="BTPModule-234")]
    BtpModule234 {
        #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
        atomic: Option<bool>,
        #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
        documentation_type: Option<models::GbtpDefinitionType>,
        #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
        end_source_location: Option<i32>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
        short_descriptor: Option<String>,
        #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
        space_after: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
        space_before: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
        space_default: Option<bool>,
        #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
        start_source_location: Option<i32>,
    },
    #[serde(rename="BTPModuleId-235")]
    BtpModuleId235 {
        #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
        atomic: Option<bool>,
        #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
        documentation_type: Option<models::GbtpDefinitionType>,
        #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
        end_source_location: Option<i32>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
        short_descriptor: Option<String>,
        #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
        space_after: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
        space_before: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
        space_default: Option<bool>,
        #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
        start_source_location: Option<i32>,
    },
    #[serde(rename="BTPName-261")]
    BtpName261 {
        #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
        atomic: Option<bool>,
        #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
        documentation_type: Option<models::GbtpDefinitionType>,
        #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
        end_source_location: Option<i32>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
        short_descriptor: Option<String>,
        #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
        space_after: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
        space_before: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
        space_default: Option<bool>,
        #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
        start_source_location: Option<i32>,
    },
    #[serde(rename="BTPPropertyAccessor-23")]
    BtpPropertyAccessor23 {
        #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
        atomic: Option<bool>,
        #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
        documentation_type: Option<models::GbtpDefinitionType>,
        #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
        end_source_location: Option<i32>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
        short_descriptor: Option<String>,
        #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
        space_after: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
        space_before: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
        space_default: Option<bool>,
        #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
        start_source_location: Option<i32>,
    },
    #[serde(rename="BTPStatement-269")]
    BtpStatement269 {
        #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
        atomic: Option<bool>,
        #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
        documentation_type: Option<models::GbtpDefinitionType>,
        #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
        end_source_location: Option<i32>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
        short_descriptor: Option<String>,
        #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
        space_after: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
        space_before: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
        space_default: Option<bool>,
        #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
        start_source_location: Option<i32>,
    },
    #[serde(rename="BTPTopLevelNode-286")]
    BtpTopLevelNode286 {
        #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
        atomic: Option<bool>,
        #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
        documentation_type: Option<models::GbtpDefinitionType>,
        #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
        end_source_location: Option<i32>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
        short_descriptor: Option<String>,
        #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
        space_after: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
        space_before: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
        space_default: Option<bool>,
        #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
        start_source_location: Option<i32>,
    },
    #[serde(rename="BTPTypeName-290")]
    BtpTypeName290 {
        #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
        atomic: Option<bool>,
        #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
        documentation_type: Option<models::GbtpDefinitionType>,
        #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
        end_source_location: Option<i32>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
        short_descriptor: Option<String>,
        #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
        space_after: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
        space_before: Option<Box<models::BtpSpace10>>,
        #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
        space_default: Option<bool>,
        #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
        start_source_location: Option<i32>,
    },
}

impl Default for BtpNode7 {
    fn default() -> Self {
        Self::BtpAnnotation231 {
            atomic: Default::default(),
            documentation_type: Default::default(),
            end_source_location: Default::default(),
            node_id: Default::default(),
            short_descriptor: Default::default(),
            space_after: Default::default(),
            space_before: Default::default(),
            space_default: Default::default(),
            start_source_location: Default::default(),
        }
        
    }
}


