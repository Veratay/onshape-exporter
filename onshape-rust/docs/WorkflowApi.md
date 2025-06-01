# \WorkflowApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enumerate_object_workflows**](WorkflowApi.md#enumerate_object_workflows) | **GET** /workflow/companies/{cid}/objects | Enumerate all of a company's workflowable objects.
[**get_active_workflows**](WorkflowApi.md#get_active_workflows) | **GET** /workflow/active | Get all active workflows for the currently logged in user's company.
[**get_allowed_approvers**](WorkflowApi.md#get_allowed_approvers) | **GET** /workflow/c/{companyId}/approvers | Get all identities allowed to be approvers on a workflow object.
[**get_audit_log**](WorkflowApi.md#get_audit_log) | **GET** /workflow/obj/{objectId}/auditlog | Get all audit log entries for a workflowable object.



## enumerate_object_workflows

> models::BtListResponseBtObjectWorkflowInfo enumerate_object_workflows(cid, object_types, states, limit, modified_after)
Enumerate all of a company's workflowable objects.

* For example, you can enumerate RELEASES, TASKS, etc in a company by last modified time.  * Caller must be a company admin.  * Specify `modifiedAfter` and use the `next` URI for complete enumeration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | The company or enterprise ID that owns the resource. | [required] |
**object_types** | Option<[**Vec<models::BtapiWorkflowableType>**](models::BtapiWorkflowableType.md)> | Optionally filter for specific workflowable types. Defaults to RELEASE and OBSOLETION |  |
**states** | Option<[**Vec<String>**](String.md)> | Optionally filter for specific workflow states like PENDING, RELEASED |  |
**limit** | Option<**i32**> | The number of list entries to return in a single API call. |  |[default to 20]
**modified_after** | Option<**String**> | The earliest modification date of workflowable object to find. |  |[default to 2000-01-01T00:00Z]

### Return type

[**models::BtListResponseBtObjectWorkflowInfo**](BTListResponseBTObjectWorkflowInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_active_workflows

> models::BtActiveWorkflowInfo get_active_workflows(document_id)
Get all active workflows for the currently logged in user's company.

Optionally takes a document ID to return all workflows for that document's owning company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | Option<**String**> |  |  |[default to ]

### Return type

[**models::BtActiveWorkflowInfo**](BTActiveWorkflowInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_allowed_approvers

> models::BtListResponseBtWorkflowObserverOptionInfo get_allowed_approvers(company_id, q, expand_teams, include_self)
Get all identities allowed to be approvers on a workflow object.

* Identities can be users and/or teams.  * For Enterprise accounts, also includes roles and any aliases that contain allowed users/teams.  * Not object- or property-specific.  * Used for delegation and company settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **String** |  | [required] |
**q** | Option<**String**> |  |  |[default to ]
**expand_teams** | Option<**bool**> |  |  |[default to true]
**include_self** | Option<**bool**> |  |  |[default to true]

### Return type

[**models::BtListResponseBtWorkflowObserverOptionInfo**](BTListResponseBTWorkflowObserverOptionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audit_log

> models::BtWorkflowAuditLogInfo get_audit_log(object_id)
Get all audit log entries for a workflowable object.

Get identities (users and/or teams) allowed to be approvers on a workflow object for the company. Not object- or property-specific; used for delegation and company settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** |  | [required] |

### Return type

[**models::BtWorkflowAuditLogInfo**](BTWorkflowAuditLogInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

