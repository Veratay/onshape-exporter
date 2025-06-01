# \MetadataApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_full_assembly_metadata**](MetadataApi.md#get_full_assembly_metadata) | **GET** /metadata/d/{did}/{wvm}/{wvmid}/e/{eid}/assembly-debug | Get the metadata for an assembly, including supporting metadata.
[**get_veop_standard_content_metadata**](MetadataApi.md#get_veop_standard_content_metadata) | **GET** /metadata/standardcontent/d/{did}/v/{vid}/e/{eid}/p/{pid} | Get the metadata for a standard content part.
[**get_wmve_metadata**](MetadataApi.md#get_wmve_metadata) | **GET** /metadata/d/{did}/{wvm}/{wvmid}/e/{eid} | Get the metadata for an element.
[**get_wmvep_metadata**](MetadataApi.md#get_wmvep_metadata) | **GET** /metadata/d/{did}/{wvm}/{wvmid}/e/{eid}/{iden}/{pid} | Get the metadata for a part.
[**get_wmveps_metadata**](MetadataApi.md#get_wmveps_metadata) | **GET** /metadata/d/{did}/{wvm}/{wvmid}/e/{eid}/p | Get the metadata for all parts in a document.
[**get_wmves_metadata**](MetadataApi.md#get_wmves_metadata) | **GET** /metadata/d/{did}/{wvm}/{wvmid}/e | Get the metadata for all elements in a document.
[**get_wv_metadata**](MetadataApi.md#get_wv_metadata) | **GET** /metadata/d/{did}/{wv}/{wvid} | Get the metadata for a workspace or version.
[**update_veop_standard_content_part_metadata**](MetadataApi.md#update_veop_standard_content_part_metadata) | **POST** /metadata/standardcontent/d/{did} | Update the metadata for a standard content part.
[**update_wv_metadata**](MetadataApi.md#update_wv_metadata) | **POST** /metadata/d/{did}/{wv}/{wvid} | Update the metadata for a workspace or version.
[**update_wve_metadata**](MetadataApi.md#update_wve_metadata) | **POST** /metadata/d/{did}/{wvm}/{wvmid}/e/{eid} | Update the metadata for an element.
[**update_wvep_metadata**](MetadataApi.md#update_wvep_metadata) | **POST** /metadata/d/{did}/{wvm}/{wvmid}/e/{eid}/{iden}/{pid} | Update the metadata for a part.



## get_full_assembly_metadata

> models::BtAssemblyItemMetadataInfo get_full_assembly_metadata(did, wvm, wvmid, eid, link_document_id, configuration)
Get the metadata for an assembly, including supporting metadata.

See [API Guide: Metadata](https://onshape-public.github.io/docs/api-adv/metadata/) for details.  * `linkDocumentId` can be specified where applicable and this combined with the query param `inferMetadataOwner` (default value is `false`) will be used to infer metadata owner.  * `configuration` optional query parameter defaults to default configuration.  * `includeComputedProperties` can be used to include or omit computed properties. Default value is `true`.  * `includeComputedAssemblyProperties` can be used to query computed assembly properties which are generally expensive. Default value is `false`.  * You can also choose to include a `thumbnail`. Default value is `false`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> |  |  |

### Return type

[**models::BtAssemblyItemMetadataInfo**](BTAssemblyItemMetadataInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_veop_standard_content_metadata

> models::BtMetadataObjectInfo get_veop_standard_content_metadata(did, vid, eid, pid, configuration, link_document_id, include_computed_properties, include_computed_assembly_properties, thumbnail)
Get the metadata for a standard content part.

See [API Guide: Metadata](https://onshape-public.github.io/docs/api-adv/metadata/) for details.  * Specify the part in the `pid` path parameter.  * The `configuration` and `linkDocumentId` query parameters are required.  * `includeComputedProperties` can be used to include or omit computed properties. Default value is `true`.  * `includeComputedAssemblyProperties` can be used to query computed assembly properties which are generally expensive. Default value is `false`.  * You can also choose to include a `thumbnail`. Default value is `false`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**vid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**pid** | **String** |  | [required] |
**configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |
**include_computed_properties** | Option<**bool**> |  |  |[default to true]
**include_computed_assembly_properties** | Option<**bool**> |  |  |[default to false]
**thumbnail** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtMetadataObjectInfo**](BTMetadataObjectInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wmve_metadata

> models::BtMetadataObjectInfo get_wmve_metadata(did, wvm, wvmid, eid, link_document_id, configuration, infer_metadata_owner, depth, include_computed_properties, include_computed_assembly_properties, thumbnail)
Get the metadata for an element.

See [API Guide: Metadata](https://onshape-public.github.io/docs/api-adv/metadata/) for details.  * `linkDocumentId` can be specified where applicable and this combined with the query param `inferMetadataOwner` (default value is `false`) will be used to infer metadata owner.  * `configuration` optional query parameter defaults to default configuration.  * You can specify an optional `depth` query parameter to get multiple levels in an assembly. Default `depth` is `1`.  * `includeComputedProperties` can be used to include or omit computed properties. Default value is `true`.  * `includeComputedAssemblyProperties` can be used to query computed assembly properties which are generally expensive. Default value is `false`.  * You can also choose to include a `thumbnail`. Default value is `false`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> |  |  |
**infer_metadata_owner** | Option<**bool**> |  |  |[default to false]
**depth** | Option<**String**> |  |  |[default to 1]
**include_computed_properties** | Option<**bool**> |  |  |[default to true]
**include_computed_assembly_properties** | Option<**bool**> |  |  |[default to false]
**thumbnail** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtMetadataObjectInfo**](BTMetadataObjectInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wmvep_metadata

> models::BtMetadataObjectInfo get_wmvep_metadata(did, wvm, wvmid, eid, iden, pid, link_document_id, configuration, rollback_bar_index, element_microversion_id, infer_metadata_owner, include_computed_properties, include_computed_assembly_properties, thumbnail)
Get the metadata for a part.

See [API Guide: Metadata](https://onshape-public.github.io/docs/api-adv/metadata/) for details.  * Specify the part in the `iden` or `pid` path parameter.  * The `configuration` optional query parameter uses the default configuration unless otherwise specified.  * `linkDocumentId` can be specified where applicable. Combined with `inferMetadataOwner` (default value is `false`), this is used to infer metadata owner.  * `includeComputedProperties` can be used to include or omit computed properties. Default value is `true`.  * `includeComputedAssemblyProperties` can be used to query computed assembly properties which are generally expensive. Default value is `false`.  * You can also choose to include a `thumbnail`. Default value is `false`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**iden** | **String** | Denotes whether the pid specified is a part id (p) or a part identity (pi). | [required] |
**pid** | **String** |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
**infer_metadata_owner** | Option<**bool**> |  |  |[default to false]
**include_computed_properties** | Option<**bool**> |  |  |[default to true]
**include_computed_assembly_properties** | Option<**bool**> |  |  |[default to false]
**thumbnail** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtMetadataObjectInfo**](BTMetadataObjectInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wmveps_metadata

> models::BtMetadataObjectListInfoBtMetadataPartInfo get_wmveps_metadata(did, wvm, wvmid, eid, link_document_id, configuration, infer_metadata_owner, include_computed_properties, include_computed_assembly_properties, thumbnail)
Get the metadata for all parts in a document.

See [API Guide: Metadata](https://onshape-public.github.io/docs/api-adv/metadata/) for details.  * The `configuration` optional query parameter uses the default configuration unless otherwise specified.  * You can specify an optional `depth` query parameter to get multiple levels in an assembly. Default `depth` is `1`.  * `linkDocumentId` can be specified where applicable. Combined with `inferMetadataOwner` (default value is `false`), this is used to infer metadata owner.  * `includeComputedProperties` can be used to include or omit computed properties. Default value is `true`.  * `includeComputedAssemblyProperties` can be used to query computed assembly properties which are generally expensive. Default value is `false`.  * You can also choose to include a `thumbnail`. Default value is `false`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**infer_metadata_owner** | Option<**bool**> |  |  |[default to false]
**include_computed_properties** | Option<**bool**> |  |  |[default to true]
**include_computed_assembly_properties** | Option<**bool**> |  |  |[default to false]
**thumbnail** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtMetadataObjectListInfoBtMetadataPartInfo**](BTMetadataObjectListInfoBTMetadataPartInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wmves_metadata

> models::BtMetadataObjectListInfoBtMetadataElementInfo get_wmves_metadata(did, wvm, wvmid, link_document_id, infer_metadata_owner, depth, include_computed_properties, include_computed_assembly_properties, thumbnail)
Get the metadata for all elements in a document.

See [API Guide: Metadata](https://onshape-public.github.io/docs/api-adv/metadata/) for details.  * You can specify the optional `depth` query parameter to get multiple levels in an assembly. Default `depth` is `1`.  * `linkDocumentId` can be specified where applicable. Combined with `inferMetadataOwner` (default value is `false`), this is used to infer metadata owner.  * `includeComputedProperties` can be used to include or omit computed properties. Default value is `true`.  * `includeComputedAssemblyProperties` can be used to query computed assembly properties which are generally expensive. Default value is `false`.  * You can also choose to include a `thumbnail`. Default value is `false`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |
**infer_metadata_owner** | Option<**bool**> |  |  |[default to false]
**depth** | Option<**String**> |  |  |[default to 1]
**include_computed_properties** | Option<**bool**> |  |  |[default to true]
**include_computed_assembly_properties** | Option<**bool**> |  |  |[default to false]
**thumbnail** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtMetadataObjectListInfoBtMetadataElementInfo**](BTMetadataObjectListInfoBTMetadataElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wv_metadata

> models::BtMetadataObjectInfo get_wv_metadata(did, wv, wvid, link_document_id, infer_metadata_owner, depth, include_computed_properties, include_computed_assembly_properties, thumbnail)
Get the metadata for a workspace or version.

See [API Guide: Metadata](https://onshape-public.github.io/docs/api-adv/metadata/) for details.  * `linkDocumentId` can be specified where applicable. Combined with `inferMetadataOwner` (default value is `false`), this is used to infer metadata owner.  * You can specify an optional `depth` query parameter to get multiple levels in an assembly. Default `depth` is `1`.  * `includeComputedProperties` can be used to include or omit computed properties. Default value is `true`.  * `includeComputedAssemblyProperties` can be used to query computed assembly properties which are generally expensive. Default value is `false`.  * You can also choose to include a `thumbnail`. Default value is `false`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |
**infer_metadata_owner** | Option<**bool**> |  |  |[default to false]
**depth** | Option<**String**> |  |  |[default to 1]
**include_computed_properties** | Option<**bool**> |  |  |[default to true]
**include_computed_assembly_properties** | Option<**bool**> |  |  |[default to false]
**thumbnail** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtMetadataObjectInfo**](BTMetadataObjectInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_veop_standard_content_part_metadata

> serde_json::Value update_veop_standard_content_part_metadata(did, link_document_id, body)
Update the metadata for a standard content part.

See [API Guide: Metadata](https://onshape-public.github.io/docs/api-adv/metadata/) for details.  * Specify the document ID to update in the `did` path param.  * Specify the document in which you have inserted the standard content part in the `linkDocumentId` query param.  * Specify the property metadata to update in the Request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**link_document_id** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_wv_metadata

> serde_json::Value update_wv_metadata(did, wv, wvid, body)
Update the metadata for a workspace or version.

See [API Guide: Metadata](https://onshape-public.github.io/docs/api-adv/metadata/) for details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_wve_metadata

> serde_json::Value update_wve_metadata(did, wvm, wvmid, eid, body, configuration)
Update the metadata for an element.

See [API Guide: Metadata](https://onshape-public.github.io/docs/api-adv/metadata/) for details.  * Microversion (`m`) in `wvm` path parameter option is not supported.  * Specify the property metadata to update in the Request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**body** | **String** |  | [required] |
**configuration** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_wvep_metadata

> serde_json::Value update_wvep_metadata(did, wvm, wvmid, eid, iden, pid, body, link_document_id, configuration, rollback_bar_index, element_microversion_id)
Update the metadata for a part.

See [API Guide: Metadata](https://onshape-public.github.io/docs/api-adv/metadata/) for details.  * Specify the part in the `iden` or `pid` path parameter.  * The `configuration` optional query parameter uses the default configuration unless otherwise specified.  * `linkDocumentId` can be specified where applicable. Combined with `inferMetadataOwner` (default value is `false`), this is used to infer metadata owner.  * Specify the property metadata to update in the Request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**iden** | **String** | Denotes whether the pid specified is a part id (p) or a part identity (pi). | [required] |
**pid** | **String** |  | [required] |
**body** | **String** |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

