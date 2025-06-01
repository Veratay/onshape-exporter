# \RevisionApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_revision_history**](RevisionApi.md#delete_revision_history) | **DELETE** /revisions/companies/{cid}/partnumber/{pnum}/elementType/{et} | Delete all revisions for a part number.
[**enumerate_revisions**](RevisionApi.md#enumerate_revisions) | **GET** /revisions/companies/{cid} | Enumerate all of a company's revisions.
[**get_all_in_document**](RevisionApi.md#get_all_in_document) | **GET** /revisions/d/{did} | Get all revisions for the specified document.
[**get_all_in_document_version**](RevisionApi.md#get_all_in_document_version) | **GET** /revisions/d/{did}/v/{vid} | Get all revisions for a version.
[**get_latest_in_document_or_company**](RevisionApi.md#get_latest_in_document_or_company) | **GET** /revisions/{cd}/{cdid}/p/{pnum}/latest | Get the latest revision for a part number in a document or company.
[**get_revision_by_part_number**](RevisionApi.md#get_revision_by_part_number) | **GET** /revisions/c/{cid}/partnumber/{pnum} | Get a list of revisions by part number.
[**get_revision_history_in_company_by_element_id**](RevisionApi.md#get_revision_history_in_company_by_element_id) | **GET** /revisions/companies/{cid}/d/{did}/{wv}/{wvid}/e/{eid} | Get a list of all revisions for an element in a company-owned document.
[**get_revision_history_in_company_by_part_id**](RevisionApi.md#get_revision_history_in_company_by_part_id) | **GET** /revisions/companies/{cid}/d/{did}/{wv}/{wvid}/e/{eid}/p/{pid} | Get a list of all revisions for a part in a company-owned document by part ID.
[**get_revision_history_in_company_by_part_number**](RevisionApi.md#get_revision_history_in_company_by_part_number) | **GET** /revisions/companies/{cid}/partnumber/{pnum} | Get a list of all revisions for a part or element in a company-owned document by part number.



## delete_revision_history

> serde_json::Value delete_revision_history(cid, pnum, et, ignore_linked_documents)
Delete all revisions for a part number.

Only company admins can call this API. All documents that contain or use the part number must be deleted first. This operation cannot be undone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |
**pnum** | **String** |  | [required] |
**et** | **String** |  | [required] |
**ignore_linked_documents** | Option<**bool**> |  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enumerate_revisions

> models::BtListResponseBtRevisionInfo enumerate_revisions(cid, element_type, limit, latest_only, after)
Enumerate all of a company's revisions.

* Returns a slice of all revisions per API call.  * To get the next set of results, use the `next` URL from the response body.  * This API can only be called by company admins.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | The company or enterprise ID that owns the resource. | [required] |
**element_type** | Option<**i32**> | 0: Part Studio, 1: Assembly, 2: Drawing. 4: Blob |  |
**limit** | Option<**i32**> | The number of list entries to return in a single API call. |  |[default to 20]
**latest_only** | Option<**bool**> | Whether to limit search to only latest revisions. |  |[default to false]
**after** | Option<**String**> | The earliest creation date of the revision to find. |  |[default to 2000-01-01T00:00Z]

### Return type

[**models::BtListResponseBtRevisionInfo**](BTListResponseBTRevisionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_in_document

> models::BtListResponseBtRevisionInfo get_all_in_document(did)
Get all revisions for the specified document.

Retrieve a list of all revisions that exist in a document and are owned by the document's owning company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |

### Return type

[**models::BtListResponseBtRevisionInfo**](BTListResponseBTRevisionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_in_document_version

> models::BtListResponseBtRevisionInfo get_all_in_document_version(did, vid)
Get all revisions for a version.

Retrieve a list of all revisions that exist in a document version and are owned by the document's owning company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**vid** | **String** |  | [required] |

### Return type

[**models::BtListResponseBtRevisionInfo**](BTListResponseBTRevisionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_in_document_or_company

> models::BtRevisionInfo get_latest_in_document_or_company(cd, cdid, pnum, et)
Get the latest revision for a part number in a document or company.

Returns 204 if no revisions are found.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cd** | **String** |  | [required] |
**cdid** | **String** |  | [required] |
**pnum** | **String** |  | [required] |
**et** | **String** | 0: Part Studio, 1: Assembly, 2: Drawing. 4: Blob | [required] |

### Return type

[**models::BtRevisionInfo**](BTRevisionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_revision_by_part_number

> models::BtRevisionInfo get_revision_by_part_number(cid, pnum, revision, element_type)
Get a list of revisions by part number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | Company id | [required] |
**pnum** | **String** | Part Number | [required] |
**revision** | Option<**String**> | Revision |  |
**element_type** | Option<**i32**> | 0: Part Studio, 1: Assembly, 2: Drawing. 4: Blob |  |

### Return type

[**models::BtRevisionInfo**](BTRevisionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_revision_history_in_company_by_element_id

> models::BtRevisionListResponse get_revision_history_in_company_by_element_id(cid, did, wv, wvid, eid, element_type, link_document_id, configuration, fill_approvers, fill_export_permission, support_change_type)
Get a list of all revisions for an element in a company-owned document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wv** | **String** | Indicates which of workspace (w) or version (v) id is specified below. | [required] |
**wvid** | **String** | The id of the workspace, version in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**element_type** | **String** |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**fill_approvers** | Option<**bool**> | Set to `true` to return a list of approvers. Default is `false` and will return `null`. |  |[default to false]
**fill_export_permission** | Option<**bool**> |  |  |[default to false]
**support_change_type** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtRevisionListResponse**](BTRevisionListResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_revision_history_in_company_by_part_id

> models::BtRevisionListResponse get_revision_history_in_company_by_part_id(cid, did, wv, wvid, eid, pid, configuration, link_document_id, fill_approvers, fill_export_permission, support_change_type)
Get a list of all revisions for a part in a company-owned document by part ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**pid** | **String** |  | [required] |
**configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |
**fill_approvers** | Option<**bool**> | Set to `true` to return a list of approvers. Default is `false` and will return `null`. |  |[default to false]
**fill_export_permission** | Option<**bool**> |  |  |[default to false]
**support_change_type** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtRevisionListResponse**](BTRevisionListResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_revision_history_in_company_by_part_number

> models::BtRevisionListResponse get_revision_history_in_company_by_part_number(cid, pnum, element_type, fill_approvers, fill_export_permission, support_change_type)
Get a list of all revisions for a part or element in a company-owned document by part number.

You can also request `elementType` in addition to `partNumber` since companies may or may not allow drawings to share part numbers with their parts/assemblies. To perform search without `elementType`, use `elementType` = -1 | UNKNOWN. Available element types are: -1: Unknown, 0: Part Studio, 1: Assembly, 2: Drawing, 4: Blob, 5: Application, 8: Variable Studio 10: Unknown

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |
**pnum** | **String** |  | [required] |
**element_type** | **String** |  | [required] |
**fill_approvers** | Option<**bool**> | Set to `true` to return a list of approvers. Default is `false` and will return `null`. |  |[default to false]
**fill_export_permission** | Option<**bool**> |  |  |[default to false]
**support_change_type** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtRevisionListResponse**](BTRevisionListResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

