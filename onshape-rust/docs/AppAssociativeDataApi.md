# \AppAssociativeDataApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**copy_associative_data**](AppAssociativeDataApi.md#copy_associative_data) | **POST** /appelements/d/{did}/w/{wid}/e/{eid}/copyassociativedata | Copy associative data from one view to another.
[**delete_associative_data**](AppAssociativeDataApi.md#delete_associative_data) | **DELETE** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/associativedata | Delete the associative data from the specified app element.
[**get_associative_data**](AppAssociativeDataApi.md#get_associative_data) | **GET** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/associativedata | Get the associative data for the specified app element.
[**post_associative_data**](AppAssociativeDataApi.md#post_associative_data) | **POST** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/associativedata | Set the associative data for the specified app element.



## copy_associative_data

> models::BtAppAssociativeDataArrayInfo copy_associative_data(did, wid, eid, bt_app_element_params_array_bt_copy_view_associative_data_params)
Copy associative data from one view to another.

Can only be copied between tabs in the same document. You can manage associativity with [translateIds](https://cad.onshape.com/glassworks/explorer/#/PartStudio/translateIds).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_app_element_params_array_bt_copy_view_associative_data_params** | Option<[**BtAppElementParamsArrayBtCopyViewAssociativeDataParams**](BtAppElementParamsArrayBtCopyViewAssociativeDataParams.md)> |  |  |

### Return type

[**models::BtAppAssociativeDataArrayInfo**](BTAppAssociativeDataArrayInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_associative_data

> models::BtAppElementBasicInfo delete_associative_data(did, eid, wvm, wvmid, transaction_id, parent_change_id, associative_data_id, external_document_id, element_id, view_id, microversion_id, document_microversion, deterministic_id, feature_id, entity_id, occurrence_id, reference_id)
Delete the associative data from the specified app element.

You can manage associativity with [translateIds](https://cad.onshape.com/glassworks/explorer/#/PartStudio/translateIds).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |[default to ]
**parent_change_id** | Option<**String**> |  |  |[default to ]
**associative_data_id** | Option<[**Vec<String>**](String.md)> |  |  |
**external_document_id** | Option<**String**> |  |  |[default to ]
**element_id** | Option<**String**> |  |  |[default to ]
**view_id** | Option<**String**> |  |  |[default to ]
**microversion_id** | Option<**String**> |  |  |[default to ]
**document_microversion** | Option<**String**> |  |  |[default to ]
**deterministic_id** | Option<**String**> |  |  |[default to ]
**feature_id** | Option<**String**> |  |  |[default to ]
**entity_id** | Option<**String**> |  |  |[default to ]
**occurrence_id** | Option<**String**> |  |  |[default to ]
**reference_id** | Option<**String**> |  |  |[default to ]

### Return type

[**models::BtAppElementBasicInfo**](BTAppElementBasicInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_associative_data

> models::BtAppAssociativeDataArrayInfo get_associative_data(did, wvm, wvmid, eid, link_document_id, transaction_id, change_id, associative_data_id, external_document_id, element_id, view_id, microversion_id, document_microversion, deterministic_id, feature_id, entity_id, occurrence_id, return_id_tags, reference_id)
Get the associative data for the specified app element.

You can manage associativity with [translateIds](https://cad.onshape.com/glassworks/explorer/#/PartStudio/translateIds).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**transaction_id** | Option<**String**> |  |  |[default to ]
**change_id** | Option<**String**> |  |  |[default to ]
**associative_data_id** | Option<[**Vec<String>**](String.md)> |  |  |
**external_document_id** | Option<**String**> |  |  |[default to ]
**element_id** | Option<**String**> |  |  |[default to ]
**view_id** | Option<**String**> |  |  |[default to ]
**microversion_id** | Option<**String**> |  |  |[default to ]
**document_microversion** | Option<**String**> |  |  |[default to ]
**deterministic_id** | Option<**String**> |  |  |[default to ]
**feature_id** | Option<**String**> |  |  |[default to ]
**entity_id** | Option<**String**> |  |  |[default to ]
**occurrence_id** | Option<**String**> |  |  |[default to ]
**return_id_tags** | Option<**bool**> |  |  |[default to false]
**reference_id** | Option<**String**> |  |  |[default to ]

### Return type

[**models::BtAppAssociativeDataArrayInfo**](BTAppAssociativeDataArrayInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_associative_data

> models::BtAppAssociativeDataArrayInfo post_associative_data(did, eid, wvm, wvmid, body)
Set the associative data for the specified app element.

You can manage associativity with [translateIds](https://cad.onshape.com/glassworks/explorer/#/PartStudio/translateIds).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

[**models::BtAppAssociativeDataArrayInfo**](BTAppAssociativeDataArrayInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

