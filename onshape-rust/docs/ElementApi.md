# \ElementApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**copy_element_from_source_document**](ElementApi.md#copy_element_from_source_document) | **POST** /elements/copyelement/{did}/workspace/{wid} | Copy an element from a source document.
[**decode_configuration**](ElementApi.md#decode_configuration) | **GET** /elements/d/{did}/{wvm}/{wvmid}/e/{eid}/configurationencodings/{cid} | Decode a configuration string.
[**delete_element**](ElementApi.md#delete_element) | **DELETE** /elements/d/{did}/w/{wid}/e/{eid} | Delete an element from a document.
[**encode_configuration_map**](ElementApi.md#encode_configuration_map) | **POST** /elements/d/{did}/e/{eid}/configurationencodings | Encode a configuration option for use in other API calls.
[**get_configuration**](ElementApi.md#get_configuration) | **GET** /elements/d/{did}/{wvm}/{wvmid}/e/{eid}/configuration | Get the configuration definition for a Part Studio, Variable Studio, or Assembly.
[**get_element_translator_formats_by_version_or_workspace**](ElementApi.md#get_element_translator_formats_by_version_or_workspace) | **GET** /elements/translatorFormats/{did}/{wv}/{wvid}/{eid} | Gets the list of formats an element can be translated to or from.
[**update_configuration**](ElementApi.md#update_configuration) | **POST** /elements/d/{did}/{wvm}/{wvmid}/e/{eid}/configuration | Update the configuration definition for a Part Studio, Variable Studio, or Assembly.
[**update_references**](ElementApi.md#update_references) | **POST** /elements/d/{did}/w/{wid}/e/{eid}/updatereferences | Update or replace references in an element.



## copy_element_from_source_document

> models::BtDocumentElementInfo copy_element_from_source_document(did, wid, bt_copy_element_params)
Copy an element from a source document.

Specify the target document and workspace in the URL. Specify the source document, workspace, and element in the request body.  If `anchorElementId` is specified, the copied element will be inserted after the anchor element. If not specified, the copied element will be inserted at the end of the tab list.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_copy_element_params** | [**BtCopyElementParams**](BtCopyElementParams.md) |  | [required] |

### Return type

[**models::BtDocumentElementInfo**](BTDocumentElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decode_configuration

> models::BtConfigurationInfo decode_configuration(did, wvm, wvmid, eid, cid, link_document_id, include_display, configuration_is_id)
Decode a configuration string.

Decode a configuration string into its original JSON form to obtain configuration parameter ID and value. See the [Configuration API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for additional details.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**cid** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |
**include_display** | Option<**bool**> |  |  |[default to false]
**configuration_is_id** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtConfigurationInfo**](BTConfigurationInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_element

> serde_json::Value delete_element(did, wid, eid)
Delete an element from a document.

Attempting to delete the last element in a document will result in an error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encode_configuration_map

> models::BtEncodedConfigurationInfo encode_configuration_map(did, eid, bt_configuration_params, version_id, link_document_id)
Encode a configuration option for use in other API calls.

Returns a configuration string in the following form:  `configuration=parameterId%3DparameterValue`  The configuration string can be used in other Onshape API calls to specify which configuration option to use. See the [Configuration API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for additional details.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_configuration_params** | [**BtConfigurationParams**](BtConfigurationParams.md) |  | [required] |
**version_id** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

[**models::BtEncodedConfigurationInfo**](BTEncodedConfigurationInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration

> models::BtConfigurationResponse2019 get_configuration(did, wvm, wvmid, eid, link_document_id)
Get the configuration definition for a Part Studio, Variable Studio, or Assembly.

Use Configurations to create variations of elements. You can configure feature and parameter values, part properties, custom part properties, face and part appearances, and sketch text. Each Part Studio can have only one Configuration, but it can contain multiple Configuration inputs.  See the [Configuration API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for additional details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtConfigurationResponse2019**](BTConfigurationResponse-2019.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_element_translator_formats_by_version_or_workspace

> Vec<models::BtModelFormatInfo> get_element_translator_formats_by_version_or_workspace(did, wv, wvid, eid, link_document_id, check_content, configuration)
Gets the list of formats an element can be translated to or from.

See the [Translation API Guide](https://onshape-public.github.io/docs/api-adv/translation/) for additional details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wv** | **String** | Indicates which of workspace (w) or version (v) id is specified below. | [required] |
**wvid** | **String** | The id of the workspace, version in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**check_content** | Option<**bool**> |  |  |[default to true]
**configuration** | Option<**String**> |  |  |[default to ]

### Return type

[**Vec<models::BtModelFormatInfo>**](BTModelFormatInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_configuration

> models::BtConfigurationResponse2019 update_configuration(did, wvm, wvmid, eid, bt_configuration_update_call2933)
Update the configuration definition for a Part Studio, Variable Studio, or Assembly.

See the [Configuration API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for additional details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_configuration_update_call2933** | Option<[**BtConfigurationUpdateCall2933**](BtConfigurationUpdateCall2933.md)> |  |  |

### Return type

[**models::BtConfigurationResponse2019**](BTConfigurationResponse-2019.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_references

> serde_json::Value update_references(did, wid, eid, bt_update_reference_params)
Update or replace references in an element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_update_reference_params** | [**BtUpdateReferenceParams**](BtUpdateReferenceParams.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

