# \DocumentApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**copy_workspace**](DocumentApi.md#copy_workspace) | **POST** /documents/{did}/workspaces/{wid}/copy | Copy workspace by document ID and workspace ID.
[**create_document**](DocumentApi.md#create_document) | **POST** /documents | Create and upload a document.
[**create_version**](DocumentApi.md#create_version) | **POST** /documents/d/{did}/versions | Create version by document ID.
[**create_workspace**](DocumentApi.md#create_workspace) | **POST** /documents/d/{did}/workspaces | Create workspace by document ID.
[**delete_document**](DocumentApi.md#delete_document) | **DELETE** /documents/{did} | Delete document by document ID.
[**delete_workspace**](DocumentApi.md#delete_workspace) | **DELETE** /documents/d/{did}/workspaces/{wid} | Delete workspace by document ID and workspace ID.
[**download_external_data**](DocumentApi.md#download_external_data) | **GET** /documents/d/{did}/externaldata/{fid} | Download external data file(s) associated with the document.
[**export2_json**](DocumentApi.md#export2_json) | **POST** /documents/d/{did}/{wv}/{wvid}/e/{eid}/export | Export document by document ID, workspace or version ID, and tab ID.
[**get_current_microversion**](DocumentApi.md#get_current_microversion) | **GET** /documents/d/{did}/{wv}/{wvid}/currentmicroversion | Retrieve current microversion by document ID and workspace or version ID.
[**get_document**](DocumentApi.md#get_document) | **GET** /documents/{did} | Retrieve document by document ID.
[**get_document_acl**](DocumentApi.md#get_document_acl) | **GET** /documents/{did}/acl | Retrieve access control list by document ID.
[**get_document_history**](DocumentApi.md#get_document_history) | **GET** /documents/d/{did}/{wm}/{wmid}/documenthistory | Retrieve document history by document ID and workspace or microversion ID.
[**get_document_permission_set**](DocumentApi.md#get_document_permission_set) | **GET** /documents/{did}/permissionset | Retrieve Document permissions by document ID.
[**get_document_versions**](DocumentApi.md#get_document_versions) | **GET** /documents/d/{did}/versions | Retrieve versions by document ID.
[**get_document_workspaces**](DocumentApi.md#get_document_workspaces) | **GET** /documents/d/{did}/workspaces | Retrieve workspaces by document ID.
[**get_documents**](DocumentApi.md#get_documents) | **GET** /documents | Get a list of documents that meet the criteria you specify.
[**get_elements_in_document**](DocumentApi.md#get_elements_in_document) | **GET** /documents/d/{did}/{wvm}/{wvmid}/elements | Retrieve tabs by document ID and workspace or version or microversion ID.
[**get_insertables**](DocumentApi.md#get_insertables) | **GET** /documents/d/{did}/{wv}/{wvid}/insertables | Retrieve insertables by document ID and workspace or version ID.
[**get_unit_info**](DocumentApi.md#get_unit_info) | **GET** /documents/d/{did}/{wvm}/{wvmid}/unitinfo | Get the selected units and precision by document ID and workspace or version or microversion ID.
[**get_version**](DocumentApi.md#get_version) | **GET** /documents/d/{did}/versions/{vid} | Retrieve version by document ID and version ID.
[**merge_into_workspace**](DocumentApi.md#merge_into_workspace) | **POST** /documents/{did}/workspaces/{wid}/merge | Merge into workspace by document ID and workspace ID.
[**merge_preview**](DocumentApi.md#merge_preview) | **GET** /documents/{did}/w/{wid}/mergePreview | Merge preview of changes that will occur based on document ID, workspace ID and source workspace/version ID
[**move_elements_to_document**](DocumentApi.md#move_elements_to_document) | **POST** /documents/d/{did}/w/{wid}/moveelement | Move tab by document ID and workspace ID.
[**restore_from_history**](DocumentApi.md#restore_from_history) | **POST** /documents/{did}/w/{wid}/restore/{vm}/{vmid} | Restore version or microversion to workspace by document ID, workspace ID, and version or microversion ID.
[**revert_unchanged_to_revisions**](DocumentApi.md#revert_unchanged_to_revisions) | **POST** /documents/d/{did}/w/{wid}/revertunchangedtorevisions | 
[**search**](DocumentApi.md#search) | **POST** /documents/search | Search document.
[**share_document**](DocumentApi.md#share_document) | **POST** /documents/{did}/share | Share document by document ID.
[**share_with_support**](DocumentApi.md#share_with_support) | **POST** /documents/{did}/shareWithSupport | Share document by document ID with Onshape support.
[**sync_app_elements**](DocumentApi.md#sync_app_elements) | **POST** /documents/d/{did}/w/{wid}/syncAppElements | 
[**un_share_document**](DocumentApi.md#un_share_document) | **DELETE** /documents/{did}/share/{eid} | Remove document View permissions from a user or other entity.
[**unshare_from_support**](DocumentApi.md#unshare_from_support) | **DELETE** /documents/{did}/shareWithSupport | Unshare document with support.
[**update_anonymous_access**](DocumentApi.md#update_anonymous_access) | **POST** /documents/{did}/acl/anonymousAccess | Allow or deny anonymous access to a document or publication.
[**update_document_attributes**](DocumentApi.md#update_document_attributes) | **POST** /documents/{did} | Update document attributes by document ID.
[**update_external_references_to_latest_documents**](DocumentApi.md#update_external_references_to_latest_documents) | **POST** /documents/d/{did}/w/{wid}/e/{eid}/latestdocumentreferences | Update external references to latest by document ID, workspace ID, and tab ID.
[**update_public_access**](DocumentApi.md#update_public_access) | **POST** /documents/{did}/acl/public | Make a document public or private.



## copy_workspace

> models::BtCopyDocumentInfo copy_workspace(did, wid, bt_copy_document_params)
Copy workspace by document ID and workspace ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_copy_document_params** | [**BtCopyDocumentParams**](BtCopyDocumentParams.md) |  | [required] |

### Return type

[**models::BtCopyDocumentInfo**](BTCopyDocumentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_document

> models::BtDocumentInfo create_document(bt_document_params)
Create and upload a document.

The `name` field is required in the `BTDocumentParams` schema when creating a new document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_document_params** | [**BtDocumentParams**](BtDocumentParams.md) |  | [required] |

### Return type

[**models::BtDocumentInfo**](BTDocumentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_version

> models::BtVersionInfo create_version(did, bt_version_or_workspace_params)
Create version by document ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**bt_version_or_workspace_params** | [**BtVersionOrWorkspaceParams**](BtVersionOrWorkspaceParams.md) |  | [required] |

### Return type

[**models::BtVersionInfo**](BTVersionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_workspace

> models::BtWorkspaceInfo create_workspace(did, bt_version_or_workspace_params)
Create workspace by document ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**bt_version_or_workspace_params** | Option<[**BtVersionOrWorkspaceParams**](BtVersionOrWorkspaceParams.md)> |  |  |

### Return type

[**models::BtWorkspaceInfo**](BTWorkspaceInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_document

> serde_json::Value delete_document(did, forever)
Delete document by document ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**forever** | Option<**bool**> |  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workspace

> serde_json::Value delete_workspace(did, wid)
Delete workspace by document ID and workspace ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_external_data

> std::path::PathBuf download_external_data(did, fid, if_none_match)
Download external data file(s) associated with the document.

* See [API Guide: Model Translation](https://onshape-public.github.io/docs/api-adv/translation/) for more details. * If downloading an exported file, poll the `requestState` in the translation response and wait for a result of `DONE` before attempting to download the file.  * Use the `resultExternalDataIds` from the translation response as the foreign id (`{fid}`) in this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**fid** | **String** |  | [required] |
**if_none_match** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export2_json

> serde_json::Value export2_json(did, wv, wvid, eid, link_document_id, btb_export_model_params)
Export document by document ID, workspace or version ID, and tab ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wv** | **String** | Indicates which of workspace (w) or version (v) id is specified below. | [required] |
**wvid** | **String** | The id of the workspace, version in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**btb_export_model_params** | Option<[**BtbExportModelParams**](BtbExportModelParams.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/octet-stream, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_microversion

> models::BtMicroversionInfo get_current_microversion(did, wv, wvid)
Retrieve current microversion by document ID and workspace or version ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |

### Return type

[**models::BtMicroversionInfo**](BTMicroversionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document

> models::BtDocumentInfo get_document(did)
Retrieve document by document ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |

### Return type

[**models::BtDocumentInfo**](BTDocumentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document_acl

> models::BtAclInfo get_document_acl(did)
Retrieve access control list by document ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |

### Return type

[**models::BtAclInfo**](BTAclInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document_history

> Vec<models::BtDocumentHistoryInfo> get_document_history(did, wm, wmid)
Retrieve document history by document ID and workspace or microversion ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wm** | **String** |  | [required] |
**wmid** | **String** |  | [required] |

### Return type

[**Vec<models::BtDocumentHistoryInfo>**](BTDocumentHistoryInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document_permission_set

> Vec<String> get_document_permission_set(did)
Retrieve Document permissions by document ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |

### Return type

**Vec<String>**

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document_versions

> Vec<models::BtVersionInfo> get_document_versions(did, offset, limit)
Retrieve versions by document ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 0]

### Return type

[**Vec<models::BtVersionInfo>**](BTVersionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document_workspaces

> Vec<models::BtWorkspaceInfo> get_document_workspaces(did)
Retrieve workspaces by document ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |

### Return type

[**Vec<models::BtWorkspaceInfo>**](BTWorkspaceInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_documents

> models::BtGlobalTreeNodeListResponse get_documents(q, filter, owner, owner_type, sort_column, sort_order, offset, limit, label, project, parent_id)
Get a list of documents that meet the criteria you specify.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Search for documents that contain the given string in the name. Search is not case-sensitive. |  |[default to ]
**filter** | Option<**i32**> | Type of documents to search: `0: My Documents | 1: Created | 2: Shared | 3: Trash | 4: Public | 5: Recent | 6: By Owner | 7: By Company | 9: By Team` |  |
**owner** | Option<**String**> | Owner ID. Can be a user ID, company ID, or team ID, depending on `ownerType`. |  |[default to ]
**owner_type** | Option<**i32**> | Type of owner. `0: User | 1: Company | 2: Onshape`. If the owner is a teamId, leave this unspecified. |  |[default to 1]
**sort_column** | Option<**String**> | Column by which to sort search results. `name | modifiedAt | createdAt (default) | email | modifiedBy | promotedAt` |  |[default to createdAt]
**sort_order** | Option<**String**> | Sort order. `desc` (descending, default), or `asc` (ascending). |  |[default to desc]
**offset** | Option<**i32**> | Offset. Determines where search results begin. Default value is 0. |  |[default to 0]
**limit** | Option<**i32**> | Maximum number of results to return per page. Default value is 20 (also the maximum). Number of results returned can be less than this value. Use the `next` URL in the response to fetch the next page. |  |[default to 20]
**label** | Option<**String**> | Label |  |
**project** | Option<**String**> | Project |  |
**parent_id** | Option<**String**> | Parent Id |  |

### Return type

[**models::BtGlobalTreeNodeListResponse**](BTGlobalTreeNodeListResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_elements_in_document

> Vec<models::BtDocumentElementInfo> get_elements_in_document(did, wvm, wvmid, link_document_id, element_type, element_id, with_thumbnails)
Retrieve tabs by document ID and workspace or version or microversion ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**element_type** | Option<**String**> |  |  |[default to ]
**element_id** | Option<**String**> |  |  |[default to ]
**with_thumbnails** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::BtDocumentElementInfo>**](BTDocumentElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_insertables

> models::BtInsertablesListResponse get_insertables(did, wv, wvid, element_id, configuration, link_document_id, include_parts, include_surfaces, include_sketches, include_reference_features, include_assemblies, include_feature_studios, include_blobs, allowed_blob_mime_types, exclude_newer_fs_versions, max_feature_script_version, include_part_studios, include_features, include_meshes, include_wires, include_flattened_bodies, include_applications, allowed_application_mime_types, include_composite_parts, include_fs_tables, include_fs_computed_part_property_functions, include_variables, include_variable_studios, allowed_blob_extensions, is_obsoletion)
Retrieve insertables by document ID and workspace or version ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**element_id** | Option<**String**> |  |  |
**configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |
**include_parts** | Option<**bool**> |  |  |[default to false]
**include_surfaces** | Option<**bool**> |  |  |[default to false]
**include_sketches** | Option<**bool**> |  |  |[default to false]
**include_reference_features** | Option<**bool**> |  |  |[default to false]
**include_assemblies** | Option<**bool**> |  |  |[default to false]
**include_feature_studios** | Option<**bool**> |  |  |[default to false]
**include_blobs** | Option<**bool**> |  |  |[default to false]
**allowed_blob_mime_types** | Option<**String**> |  |  |[default to ]
**exclude_newer_fs_versions** | Option<**bool**> |  |  |[default to false]
**max_feature_script_version** | Option<**i32**> |  |  |
**include_part_studios** | Option<**bool**> |  |  |[default to false]
**include_features** | Option<**bool**> |  |  |[default to false]
**include_meshes** | Option<**bool**> |  |  |[default to false]
**include_wires** | Option<**bool**> |  |  |[default to false]
**include_flattened_bodies** | Option<**bool**> |  |  |[default to false]
**include_applications** | Option<**bool**> |  |  |[default to false]
**allowed_application_mime_types** | Option<**String**> |  |  |[default to ]
**include_composite_parts** | Option<**bool**> |  |  |[default to false]
**include_fs_tables** | Option<**bool**> |  |  |[default to false]
**include_fs_computed_part_property_functions** | Option<**bool**> |  |  |[default to false]
**include_variables** | Option<**bool**> |  |  |[default to false]
**include_variable_studios** | Option<**bool**> |  |  |[default to false]
**allowed_blob_extensions** | Option<**String**> |  |  |[default to ]
**is_obsoletion** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtInsertablesListResponse**](BTInsertablesListResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unit_info

> models::BtUnitInfo get_unit_info(did, wvm, wvmid, link_document_id)
Get the selected units and precision by document ID and workspace or version or microversion ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtUnitInfo**](BTUnitInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version

> models::BtVersionInfo get_version(did, vid, parents, link_document_id)
Retrieve version by document ID and version ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**vid** | **String** |  | [required] |
**parents** | Option<**bool**> |  |  |[default to false]
**link_document_id** | Option<**String**> |  |  |

### Return type

[**models::BtVersionInfo**](BTVersionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_into_workspace

> models::BtDocumentMergeInfo merge_into_workspace(did, wid, bt_version_or_workspace_merge_info)
Merge into workspace by document ID and workspace ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_version_or_workspace_merge_info** | [**BtVersionOrWorkspaceMergeInfo**](BtVersionOrWorkspaceMergeInfo.md) |  | [required] |

### Return type

[**models::BtDocumentMergeInfo**](BTDocumentMergeInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_preview

> models::BtMergePreviewInfo merge_preview(did, wid, source_type, source_id, link_document_id)
Merge preview of changes that will occur based on document ID, workspace ID and source workspace/version ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**source_type** | **String** |  | [required] |
**source_id** | **String** |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtMergePreviewInfo**](BTMergePreviewInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_elements_to_document

> models::BtMoveElementInfo move_elements_to_document(did, wid, bt_move_element_params)
Move tab by document ID and workspace ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_move_element_params** | [**BtMoveElementParams**](BtMoveElementParams.md) |  | [required] |

### Return type

[**models::BtMoveElementInfo**](BTMoveElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_from_history

> models::BtRestoreFromHistoryInfo restore_from_history(did, wid, vm, vmid, link_document_id, bt_restore_info)
Restore version or microversion to workspace by document ID, workspace ID, and version or microversion ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**vm** | **String** |  | [required] |
**vmid** | **String** |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**bt_restore_info** | Option<[**BtRestoreInfo**](BtRestoreInfo.md)> |  |  |

### Return type

[**models::BtRestoreFromHistoryInfo**](BTRestoreFromHistoryInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revert_unchanged_to_revisions

> Vec<models::BtUnchangedElementInfo> revert_unchanged_to_revisions(did, wid, bt_revert_unchanged_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_revert_unchanged_params** | Option<[**BtRevertUnchangedParams**](BtRevertUnchangedParams.md)> |  |  |

### Return type

[**Vec<models::BtUnchangedElementInfo>**](BTUnchangedElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search

> serde_json::Value search(bt_document_search_params)
Search document.

This returns list of documents based on search parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_document_search_params** | [**BtDocumentSearchParams**](BtDocumentSearchParams.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_document

> models::BtAclInfo share_document(did, bt_share_params)
Share document by document ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**bt_share_params** | [**BtShareParams**](BtShareParams.md) |  | [required] |

### Return type

[**models::BtAclInfo**](BTAclInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_with_support

> serde_json::Value share_with_support(did)
Share document by document ID with Onshape support.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_app_elements

> serde_json::Value sync_app_elements(did, wid, bt_sync_app_element_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_sync_app_element_params** | [**BtSyncAppElementParams**](BtSyncAppElementParams.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_share_document

> serde_json::Value un_share_document(did, eid, entry_type)
Remove document View permissions from a user or other entity.

Specify the ID of the entity to unshare with in the `eid` field, and specify the type of entity being identified in the `entryType` field. For example, to unshare a document with a company, you would use `1` as the `entryType` value and the `companyId` as the `entityId`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | ID of the document to unshare. | [required] |
**eid** | **String** | ID of the entity to remove permissions for. Uses `userId`, `companyId`, `teamId`, `documentId`, or `applicationId`, depending on the `entryType` value. | [required] |
**entry_type** | Option<**i32**> | `0` (user) | `1` (company) | `2` (team) | `3` (document) | `4` (application) | `5` (connection_user) |  |[default to 0]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unshare_from_support

> serde_json::Value unshare_from_support(did)
Unshare document with support.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_anonymous_access

> serde_json::Value update_anonymous_access(did, bt_acl_params)
Allow or deny anonymous access to a document or publication.

If anonymous access is allowed, you can allow or deny anonymous users the ability to export the document or publication. If `anonymousAccessAllowed=false` and `anonymousAllowsExport=true`, the call will throw an error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**bt_acl_params** | [**BtAclParams**](BtAclParams.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_document_attributes

> models::BtDocumentSummaryInfo update_document_attributes(did, bt_document_params)
Update document attributes by document ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**bt_document_params** | [**BtDocumentParams**](BtDocumentParams.md) |  | [required] |

### Return type

[**models::BtDocumentSummaryInfo**](BTDocumentSummaryInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_external_references_to_latest_documents

> models::BtLinkToLatestDocumentInfo update_external_references_to_latest_documents(did, wid, eid, bt_link_to_latest_document_params)
Update external references to latest by document ID, workspace ID, and tab ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_link_to_latest_document_params** | Option<[**BtLinkToLatestDocumentParams**](BtLinkToLatestDocumentParams.md)> |  |  |

### Return type

[**models::BtLinkToLatestDocumentInfo**](BTLinkToLatestDocumentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_public_access

> serde_json::Value update_public_access(did, bt_acl_params)
Make a document public or private.

 * Set `public=true` in the request body to make the document public. Set to `false` to make it private. Free users cannot make documents private.   * The `documentId` provided in the URL must match the one provided in the request body exactly. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**bt_acl_params** | Option<[**BtAclParams**](BtAclParams.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

