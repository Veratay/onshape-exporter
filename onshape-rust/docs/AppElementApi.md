# \AppElementApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**abort_transaction**](AppElementApi.md#abort_transaction) | **DELETE** /appelements/d/{did}/w/{wid}/e/{eid}/transactions/{tid} | Abort a transaction.
[**bulk_create_element**](AppElementApi.md#bulk_create_element) | **POST** /appelements/d/{did}/w/{wid}/bulkcreate | Create multiple empty application elements at once.
[**commit_transactions**](AppElementApi.md#commit_transactions) | **POST** /appelements/d/{did}/w/{wid}/transactions | Merge multiple transactions into one microversion.
[**compare_app_element_json**](AppElementApi.md#compare_app_element_json) | **GET** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/compare | Compare app element JSON trees between workspaces/versions/microversions in a document.
[**create_element**](AppElementApi.md#create_element) | **POST** /appelements/d/{did}/w/{wid} | Create a new application element.
[**create_reference**](AppElementApi.md#create_reference) | **POST** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/references | Creates a reference to an app element.
[**delete_app_element_content**](AppElementApi.md#delete_app_element_content) | **DELETE** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/subelements/{sid} | Deletes the content from the specified app element.
[**delete_app_element_content_batch**](AppElementApi.md#delete_app_element_content_batch) | **DELETE** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/subelements | Delete multiple subelements array by document ID, workspace or version or microversion ID, tab ID, and subelement IDs.
[**delete_blob_subelement**](AppElementApi.md#delete_blob_subelement) | **DELETE** /appelements/d/{did}/w/{wid}/e/{eid}/blob/{bid} | Delete a blob subelement from an app element.
[**delete_reference**](AppElementApi.md#delete_reference) | **DELETE** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/references/{rid} | Delete an app element reference.
[**download_blob_subelement**](AppElementApi.md#download_blob_subelement) | **GET** /appelements/d/{did}/{vm}/{vmid}/e/{eid}/blob/{bid} | Download a blob subelement from the specified app element.
[**download_blob_subelement_workspace**](AppElementApi.md#download_blob_subelement_workspace) | **GET** /appelements/d/{did}/w/{wid}/e/{eid}/blob/{bid} | Download the blob element (i.e., a file) stored in an app element in a document's workspace.
[**get_app_element_history**](AppElementApi.md#get_app_element_history) | **GET** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/history | Get the history of the specified all element.
[**get_blob_subelement_ids**](AppElementApi.md#get_blob_subelement_ids) | **GET** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/blob | Get a list of all blob subelement IDs for the specified workspace, version, or microversion.
[**get_element_transactions**](AppElementApi.md#get_element_transactions) | **GET** /appelements/d/{did}/w/{wid}/e/{eid}/transactions | Get a list of all transactions performed on an element.
[**get_json**](AppElementApi.md#get_json) | **GET** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/json | Get the full JSON tree for the specified workspace/version/microversion.
[**get_json_paths**](AppElementApi.md#get_json_paths) | **POST** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/jsonpaths | Get the JSON at specified paths for an element.
[**get_sub_element_content**](AppElementApi.md#get_sub_element_content) | **GET** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content | Get a list of all subelement IDs in a specified workspace/version/microversion.
[**get_sub_element_content_batch**](AppElementApi.md#get_sub_element_content_batch) | **GET** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/subelements | Get a list of multiple subelements by document ID, workspace or version or microversion ID, tab ID, and subelement IDs.
[**get_subelement_ids**](AppElementApi.md#get_subelement_ids) | **GET** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/ids | Get a list of all subelement IDs in a specified workspace/version/microversion.
[**resolve_all_element_references**](AppElementApi.md#resolve_all_element_references) | **GET** /appelements/d/{did}/{wvm}/{wvmid}/resolvereferences | Resolves bulk app element references.
[**resolve_reference**](AppElementApi.md#resolve_reference) | **GET** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/references/{rid} | Resolves a single reference to an app element.
[**resolve_references**](AppElementApi.md#resolve_references) | **GET** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/resolvereferences | Resolves bulk app element references.
[**start_transaction**](AppElementApi.md#start_transaction) | **POST** /appelements/d/{did}/w/{wid}/e/{eid}/transactions | Start a transaction
[**update_app_element**](AppElementApi.md#update_app_element) | **POST** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content | Update the content for the specified app element.
[**update_reference**](AppElementApi.md#update_reference) | **POST** /appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/references/{rid} | Update an app element reference.
[**upload_blob_subelement**](AppElementApi.md#upload_blob_subelement) | **POST** /appelements/d/{did}/w/{wid}/e/{eid}/blob/{bid} | Create a new blob subelement from an uploaded file.



## abort_transaction

> serde_json::Value abort_transaction(did, eid, wid, tid, return_error)
Abort a transaction.

Deletes a microbranch (i.e., the branch with the microversion for the transaction).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**tid** | **String** |  | [required] |
**return_error** | Option<**bool**> |  |  |[default to true]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_create_element

> models::BtAppElementBulkCreateInfo bulk_create_element(did, wid, bt_app_element_bulk_create_params, link_document_id)
Create multiple empty application elements at once.

Call this faster API instead of creating multiple app elements one at a time or in parallel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**bt_app_element_bulk_create_params** | [**BtAppElementBulkCreateParams**](BtAppElementBulkCreateParams.md) |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtAppElementBulkCreateInfo**](BTAppElementBulkCreateInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commit_transactions

> models::BtAppElementBulkModifyInfo commit_transactions(did, wid, bt_app_element_commit_transaction_params, link_document_id)
Merge multiple transactions into one microversion.

If successful, all transactions will be committed to a single microversion. If the call raises an error, nothing will be committed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**bt_app_element_commit_transaction_params** | [**BtAppElementCommitTransactionParams**](BtAppElementCommitTransactionParams.md) |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtAppElementBulkModifyInfo**](BTAppElementBulkModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compare_app_element_json

> models::BtDiffJsonResponse2725 compare_app_element_json(did, wvm, wvmid, eid, workspace_id, version_id, microversion_id, link_document_id, json_difference_format)
Compare app element JSON trees between workspaces/versions/microversions in a document.

Specify the source workspace/version/microversion in the URL and specify the target workspace/version/microversion in the query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**workspace_id** | Option<**String**> |  |  |
**version_id** | Option<**String**> |  |  |
**microversion_id** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |
**json_difference_format** | Option<**String**> |  |  |

### Return type

[**models::BtDiffJsonResponse2725**](BTDiffJsonResponse-2725.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_element

> models::BtAppElementModifyInfo create_element(did, wid, bt_app_element_params, link_document_id)
Create a new application element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**bt_app_element_params** | [**BtAppElementParams**](BtAppElementParams.md) |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_reference

> models::BtAppElementReferenceInfo create_reference(did, eid, wvm, wvmid, bt_app_element_reference_params)
Creates a reference to an app element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**bt_app_element_reference_params** | [**BtAppElementReferenceParams**](BtAppElementReferenceParams.md) |  | [required] |

### Return type

[**models::BtAppElementReferenceInfo**](BTAppElementReferenceInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_app_element_content

> models::BtAppElementModifyInfo delete_app_element_content(did, eid, wvm, wvmid, sid, transaction_id, parent_change_id, description)
Deletes the content from the specified app element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**sid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**parent_change_id** | Option<**String**> |  |  |
**description** | Option<**String**> |  |  |

### Return type

[**models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_app_element_content_batch

> models::BtAppElementModifyInfo delete_app_element_content_batch(did, eid, wvm, wvmid, subelement_ids, transaction_id, parent_change_id, description)
Delete multiple subelements array by document ID, workspace or version or microversion ID, tab ID, and subelement IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**subelement_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**transaction_id** | Option<**String**> |  |  |
**parent_change_id** | Option<**String**> |  |  |
**description** | Option<**String**> |  |  |

### Return type

[**models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_blob_subelement

> models::BtAppElementModifyInfo delete_blob_subelement(did, wid, eid, bid, transaction_id, change_id)
Delete a blob subelement from an app element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**change_id** | Option<**String**> |  |  |

### Return type

[**models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_reference

> models::BtAppElementReferenceInfo delete_reference(did, eid, wvm, wvmid, rid, transaction_id, parent_change_id, description)
Delete an app element reference.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**rid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**parent_change_id** | Option<**String**> |  |  |
**description** | Option<**String**> |  |  |

### Return type

[**models::BtAppElementReferenceInfo**](BTAppElementReferenceInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_blob_subelement

> std::path::PathBuf download_blob_subelement(did, vm, vmid, eid, bid, content_disposition, if_none_match, transaction_id, change_id, link_document_id)
Download a blob subelement from the specified app element.

Download a blob subelement as a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**vm** | **String** |  | [required] |
**vmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bid** | **String** |  | [required] |
**content_disposition** | Option<**String**> |  |  |
**if_none_match** | Option<**String**> |  |  |
**transaction_id** | Option<**String**> |  |  |
**change_id** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_blob_subelement_workspace

> std::path::PathBuf download_blob_subelement_workspace(did, wid, eid, bid, content_disposition, if_none_match, transaction_id, change_id)
Download the blob element (i.e., a file) stored in an app element in a document's workspace.

The downloaded file can be used to retrieve stored subelements.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bid** | **String** |  | [required] |
**content_disposition** | Option<**String**> |  |  |
**if_none_match** | Option<**String**> |  |  |
**transaction_id** | Option<**String**> |  |  |
**change_id** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_element_history

> models::BtAppElementHistoryInfo get_app_element_history(did, eid, wvm, wvmid)
Get the history of the specified all element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |

### Return type

[**models::BtAppElementHistoryInfo**](BTAppElementHistoryInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blob_subelement_ids

> models::BtAppElementIdsInfo get_blob_subelement_ids(did, eid, wvm, wvmid, transaction_id, change_id)
Get a list of all blob subelement IDs for the specified workspace, version, or microversion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**change_id** | Option<**String**> |  |  |

### Return type

[**models::BtAppElementIdsInfo**](BTAppElementIdsInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_element_transactions

> models::BtAppElementTransactionsInfo get_element_transactions(did, eid, wid)
Get a list of all transactions performed on an element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wid** | **String** |  | [required] |

### Return type

[**models::BtAppElementTransactionsInfo**](BTAppElementTransactionsInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_json

> models::BtGetJsonResponse2137 get_json(did, wvm, wvmid, eid, link_document_id, transaction_id, change_id)
Get the full JSON tree for the specified workspace/version/microversion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**transaction_id** | Option<**String**> | The id of the transaction in which this operation should take place. Transaction ids can be generated through the AppElement startTransaction API. |  |
**change_id** | Option<**String**> | The id of the last change made to this application element. This can be retrieved from the response for any app element modification endpoint. |  |

### Return type

[**models::BtGetJsonResponse2137**](BTGetJsonResponse-2137.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_json_paths

> models::BtGetJsonPathsResponse1544 get_json_paths(did, wvm, wvmid, eid, link_document_id, transaction_id, change_id, bt_get_json_paths1697)
Get the JSON at specified paths for an element.

Use this endpoint to return the JSON at the specified path instead of returning the entire JSON for the element. This POST endpoint does not write any information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**transaction_id** | Option<**String**> |  |  |
**change_id** | Option<**String**> |  |  |
**bt_get_json_paths1697** | Option<[**BtGetJsonPaths1697**](BtGetJsonPaths1697.md)> |  |  |

### Return type

[**models::BtGetJsonPathsResponse1544**](BTGetJsonPathsResponse-1544.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sub_element_content

> models::BtAppElementContentInfo get_sub_element_content(did, wvm, wvmid, eid, link_document_id, transaction_id, change_id, base_change_id, subelement_id)
Get a list of all subelement IDs in a specified workspace/version/microversion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**transaction_id** | Option<**String**> |  |  |
**change_id** | Option<**String**> |  |  |
**base_change_id** | Option<**String**> |  |  |
**subelement_id** | Option<**String**> |  |  |

### Return type

[**models::BtAppElementContentInfo**](BTAppElementContentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sub_element_content_batch

> models::BtAppElementContentInfo get_sub_element_content_batch(did, wvm, wvmid, eid, link_document_id, transaction_id, change_id, base_change_id, subelement_ids)
Get a list of multiple subelements by document ID, workspace or version or microversion ID, tab ID, and subelement IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**transaction_id** | Option<**String**> | The id of the transaction in which this operation should take place. Transaction ids can be generated through the AppElement startTransaction API. |  |
**change_id** | Option<**String**> | The id of the last change made to this application element. This can be retrieved from the response for any app element modification endpoint. |  |
**base_change_id** | Option<**String**> | The id of a change made prior to the specified or implied changeId. If specified, only changes made after the base changeId are returned. |  |
**subelement_ids** | Option<[**Vec<String>**](String.md)> | The array of subelementIds in format: `&subelementIds=ID1& &subelementIds=ID2...&subelementIds=IDn.` |  |

### Return type

[**models::BtAppElementContentInfo**](BTAppElementContentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subelement_ids

> models::BtAppElementIdsInfo get_subelement_ids(did, eid, wvm, wvmid, transaction_id, change_id)
Get a list of all subelement IDs in a specified workspace/version/microversion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**change_id** | Option<**String**> |  |  |

### Return type

[**models::BtAppElementIdsInfo**](BTAppElementIdsInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resolve_all_element_references

> std::collections::HashMap<String, models::BtAppElementReferencesResolveInfo> resolve_all_element_references(did, wvm, wvmid, link_document_id, transaction_id, parent_change_id, include_internal, reference_ids, element_ids, drawings_only)
Resolves bulk app element references.

Resolve all references for all workspace elements. For bulk operations  only. Use `resolveReferences` for a single element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**transaction_id** | Option<**String**> | The id of the transaction in which this operation should take place. Transaction ids can be generated through the AppElement startTransaction API. |  |
**parent_change_id** | Option<**String**> | The id of the last change made to this application element. This can be retrieved from the response for any app element modification endpoint. |  |
**include_internal** | Option<**bool**> | Whether to include references that have been deleted or inactivated. |  |[default to false]
**reference_ids** | Option<**String**> | Comma separated string of reference ids find. |  |[default to ]
**element_ids** | Option<**String**> | Comma separated string of element ids to search for references in. |  |[default to ]
**drawings_only** | Option<**bool**> | Whether to find references for only Onshape drawing app elements. |  |[default to false]

### Return type

[**std::collections::HashMap<String, models::BtAppElementReferencesResolveInfo>**](BTAppElementReferencesResolveInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resolve_reference

> models::BtAppElementReferenceResolveInfo resolve_reference(did, eid, wvm, wvmid, rid, transaction_id, parent_change_id, include_internal, link_document_id)
Resolves a single reference to an app element.

For single operations only. Use `resolveReferences` for bulk operations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**rid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**parent_change_id** | Option<**String**> |  |  |
**include_internal** | Option<**bool**> |  |  |[default to false]
**link_document_id** | Option<**String**> |  |  |

### Return type

[**models::BtAppElementReferenceResolveInfo**](BTAppElementReferenceResolveInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resolve_references

> models::BtAppElementReferencesResolveInfo resolve_references(did, wvm, wvmid, eid, link_document_id, transaction_id, parent_change_id, include_internal, reference_ids)
Resolves bulk app element references.

For bulk operations only. Use `resolveReference` for a single operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**transaction_id** | Option<**String**> |  |  |
**parent_change_id** | Option<**String**> |  |  |
**include_internal** | Option<**bool**> |  |  |[default to false]
**reference_ids** | Option<**String**> |  |  |[default to ]

### Return type

[**models::BtAppElementReferencesResolveInfo**](BTAppElementReferencesResolveInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_transaction

> models::BtAppElementModifyInfo start_transaction(did, eid, wid, bt_app_element_start_transaction_params)
Start a transaction

Creates a microbranch (i.e., a branch for a new microversion).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_app_element_start_transaction_params** | [**BtAppElementStartTransactionParams**](BtAppElementStartTransactionParams.md) |  | [required] |

### Return type

[**models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_app_element

> models::BtAppElementModifyInfo update_app_element(did, eid, wvm, wvmid, bt_app_element_update_params)
Update the content for the specified app element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**bt_app_element_update_params** | Option<[**BtAppElementUpdateParams**](BtAppElementUpdateParams.md)> |  |  |

### Return type

[**models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_reference

> models::BtAppElementReferenceInfo update_reference(did, eid, wvm, wvmid, rid, bt_app_element_reference_params)
Update an app element reference.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**rid** | **String** |  | [required] |
**bt_app_element_reference_params** | [**BtAppElementReferenceParams**](BtAppElementReferenceParams.md) |  | [required] |

### Return type

[**models::BtAppElementReferenceInfo**](BTAppElementReferenceInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_blob_subelement

> models::BtAppElementModifyInfo upload_blob_subelement(did, wid, eid, bid, transaction_id, parent_change_id, description, file, file_content_length)
Create a new blob subelement from an uploaded file.

Request body parameters are multipart fields, so you must use `\"Content-Type\":\"multipart/form-data\"` in the request header.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**parent_change_id** | Option<**String**> |  |  |
**description** | Option<**String**> |  |  |
**file** | Option<[**serde_json::Value**](serde_json::Value.md)> | File to upload. |  |
**file_content_length** | Option<**i64**> |  |  |[default to -1]

### Return type

[**models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

