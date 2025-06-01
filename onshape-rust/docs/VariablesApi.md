# \VariablesApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_variable_studio**](VariablesApi.md#create_variable_studio) | **POST** /variables/d/{did}/w/{wid}/variablestudio | Create a new Variable Studio in a document and workspace.
[**get_variable_studio_references**](VariablesApi.md#get_variable_studio_references) | **GET** /variables/d/{did}/{wv}/{wvid}/e/{eid}/variablestudioreferences | Get the Variable Studio references for an element.
[**get_variable_studio_scope**](VariablesApi.md#get_variable_studio_scope) | **GET** /variables/d/{did}/{wv}/{wvid}/e/{eid}/variablestudioscope | Get the scope of a Variable Studio.
[**get_variables**](VariablesApi.md#get_variables) | **GET** /variables/d/{did}/{wv}/{wvid}/e/{eid}/variables | Get the contents of all variable tables in an element.
[**set_variable_studio_references**](VariablesApi.md#set_variable_studio_references) | **POST** /variables/d/{did}/w/{wid}/e/{eid}/variablestudioreferences | Set the Variable Studio references for an element.
[**set_variable_studio_scope**](VariablesApi.md#set_variable_studio_scope) | **POST** /variables/d/{did}/w/{wid}/e/{eid}/variablestudioscope | Set the scope the Variable Studio.
[**set_variables**](VariablesApi.md#set_variables) | **POST** /variables/d/{did}/w/{wid}/e/{eid}/variables | Assign variables to a Variable Studio



## create_variable_studio

> models::BtDocumentElementInfo create_variable_studio(did, wid, bt_model_element_params, link_document_id)
Create a new Variable Studio in a document and workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**bt_model_element_params** | [**BtModelElementParams**](BtModelElementParams.md) |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtDocumentElementInfo**](BTDocumentElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_variable_studio_references

> models::BtVariableStudioReferenceListInfo get_variable_studio_references(did, wv, wvid, eid, link_document_id)
Get the Variable Studio references for an element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wv** | **String** | Indicates which of workspace (w) or version (v) id is specified below. | [required] |
**wvid** | **String** | The id of the workspace, version in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtVariableStudioReferenceListInfo**](BTVariableStudioReferenceListInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_variable_studio_scope

> models::BtVariableStudioScopeInfo get_variable_studio_scope(did, wv, wvid, eid, link_document_id)
Get the scope of a Variable Studio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wv** | **String** | Indicates which of workspace (w) or version (v) id is specified below. | [required] |
**wvid** | **String** | The id of the workspace, version in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtVariableStudioScopeInfo**](BTVariableStudioScopeInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_variables

> Vec<models::BtVariableTableInfo> get_variables(did, wv, wvid, eid, link_document_id, configuration, include_values_and_referenced_variables)
Get the contents of all variable tables in an element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wv** | **String** | Indicates which of workspace (w) or version (v) id is specified below. | [required] |
**wvid** | **String** | The id of the workspace, version in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**include_values_and_referenced_variables** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::BtVariableTableInfo>**](BTVariableTableInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_variable_studio_references

> serde_json::Value set_variable_studio_references(did, wid, eid, bt_variable_studio_reference_list_info, link_document_id)
Set the Variable Studio references for an element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**eid** | **String** |  | [required] |
**bt_variable_studio_reference_list_info** | [**BtVariableStudioReferenceListInfo**](BtVariableStudioReferenceListInfo.md) |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_variable_studio_scope

> serde_json::Value set_variable_studio_scope(did, wid, eid, bt_variable_studio_scope_info, link_document_id)
Set the scope the Variable Studio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**eid** | **String** |  | [required] |
**bt_variable_studio_scope_info** | [**BtVariableStudioScopeInfo**](BtVariableStudioScopeInfo.md) |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_variables

> serde_json::Value set_variables(did, wid, eid, bt_variable_params, link_document_id)
Assign variables to a Variable Studio

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**eid** | **String** |  | [required] |
**bt_variable_params** | [**Vec<models::BtVariableParams>**](BTVariableParams.md) |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

