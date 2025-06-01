# \BlobElementApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_blob_translation**](BlobElementApi.md#create_blob_translation) | **POST** /blobelements/d/{did}/{wv}/{wvid}/e/{eid}/translations | Export a blob element to another format.
[**download_file_workspace**](BlobElementApi.md#download_file_workspace) | **GET** /blobelements/d/{did}/w/{wid}/e/{eid} | Download a file from a blob element for the specified workspace/version/microversion.
[**update_units**](BlobElementApi.md#update_units) | **POST** /blobelements/d/{did}/w/{wid}/e/{eid}/units | Change the measurement units for the blob element.
[**upload_file_create_element**](BlobElementApi.md#upload_file_create_element) | **POST** /blobelements/d/{did}/w/{wid} | Upload a file and create a blob element from it.
[**upload_file_update_element**](BlobElementApi.md#upload_file_update_element) | **POST** /blobelements/d/{did}/w/{wid}/e/{eid} | Update a blob element by uploading a file.



## create_blob_translation

> models::BtTranslationRequestInfo create_blob_translation(did, wv, wvid, eid, bt_translate_format_params, link_document_id)
Export a blob element to another format.

* Use `formatName` in the JSON request body to specify the export file type. Use [Translations/getAllTranslatorFormats](https://cad.onshape.com/glassworks/explorer/#/Translation/getAllTranslatorFormats) to get a list of valid export file formats.  * Set `storeInDocument` to `false` to export to a data file. Set to `true` to export to a blob element in the same document.  * See [API Guide: Model Translation](https://onshape-public.github.io/docs/api-adv/translation/) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wv** | **String** | Indicates which of workspace (w) or version (v) id is specified below. | [required] |
**wvid** | **String** | The id of the workspace, version in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**bt_translate_format_params** | [**BtTranslateFormatParams**](BtTranslateFormatParams.md) |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtTranslationRequestInfo**](BTTranslationRequestInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_file_workspace

> std::path::PathBuf download_file_workspace(did, wid, eid, link_document_id, content_disposition, if_none_match)
Download a file from a blob element for the specified workspace/version/microversion.

See [API Guide: Model Translation](https://onshape-public.github.io/docs/api-adv/translation/) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**content_disposition** | Option<**String**> | If \"attachment\", includes a Content-Disposition return header with the filename. |  |
**if_none_match** | Option<**String**> | Entity tag; an md5 checksum of the data in double quotes. If the data to download has the same checksum as this entity tag, a 304 'Not Modified' status will be returned. The entity tag is returned in the response headers as ETag. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_units

> models::BtDocumentElementProcessingInfo update_units(did, wid, eid, bt_update_mesh_units_params, link_document_id)
Change the measurement units for the blob element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**bt_update_mesh_units_params** | [**BtUpdateMeshUnitsParams**](BtUpdateMeshUnitsParams.md) |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtDocumentElementProcessingInfo**](BTDocumentElementProcessingInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file_create_element

> models::BtDocumentElementProcessingInfo upload_file_create_element(did, wid, link_document_id, file, allow_faulty_parts, create_composite, create_drawing_if_possible, encoded_filename, extract_assembly_hierarchy, flatten_assemblies, format_name, join_adjacent_surfaces, location_element_id, location_group_id, location_position, notify_user, owner_id, parent_id, project_id, public, one_part_per_doc, split_assemblies_into_multiple_documents, store_in_document, translate, unit, upload_id, version_string, import_appearances, import_material_density, y_axis_is_up, import_within_document, use_iges_import_post_processing, upgrade_feature_script_version, preserve_source_ids, document_id, repoint_app_element_version_refs)
Upload a file and create a blob element from it.

Request body parameters are multipart fields, so you must use `\"Content-Type\":\"multipart/form-data\"` in the request header.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**file** | Option<[**serde_json::Value**](serde_json::Value.md)> | The file to upload. |  |
**allow_faulty_parts** | Option<**bool**> | If true, and a part doesn't pass Onshape validation, it will be imported with faults. |  |
**create_composite** | Option<**bool**> | Not supported for importing into a single part studio. |  |
**create_drawing_if_possible** | Option<**bool**> |  |  |
**encoded_filename** | Option<**String**> | If the filename contains non-ASCII characters. Use this field to store the filename. |  |
**extract_assembly_hierarchy** | Option<**bool**> |  |  |
**flatten_assemblies** | Option<**bool**> | If the file is an assembly, or contains an assembly, setting this to True will import it as a Part Studio. In this case the assembly will be flattened to a set of parts in a Part Studio. There will be duplicate parts created whenever a part is instanced more than once. If False, it will be imported as an Assembly. |  |
**format_name** | Option<**String**> |  |  |
**join_adjacent_surfaces** | Option<**bool**> |  |  |
**location_element_id** | Option<**String**> |  |  |
**location_group_id** | Option<**String**> |  |  |
**location_position** | Option<**i32**> |  |  |[default to -1]
**notify_user** | Option<**bool**> |  |  |[default to true]
**owner_id** | Option<**String**> |  |  |
**parent_id** | Option<**String**> |  |  |
**project_id** | Option<**String**> |  |  |
**public** | Option<**bool**> |  |  |
**one_part_per_doc** | Option<**bool**> |  |  |[default to false]
**split_assemblies_into_multiple_documents** | Option<**bool**> |  |  |[default to false]
**store_in_document** | Option<**bool**> |  |  |[default to true]
**translate** | Option<**bool**> |  |  |[default to true]
**unit** | Option<**String**> |  |  |[default to ]
**upload_id** | Option<**String**> |  |  |
**version_string** | Option<**String**> |  |  |
**import_appearances** | Option<**bool**> | Face appearances defined on models will be imported. |  |[default to true]
**import_material_density** | Option<**bool**> | Material density defined on models will be imported. |  |[default to false]
**y_axis_is_up** | Option<**bool**> | If the file was created in a system that orients with Y Axis Up, the models would by default be brought into Onshape (a Z Axis Up system) with a flipped coordinate system. Toggle this value to reorient the axis system to match Onshape and display the model with the coordinates you expect. |  |
**import_within_document** | Option<**bool**> |  |  |
**use_iges_import_post_processing** | Option<**bool**> | Try getting optimized topology from IGES model. |  |[default to false]
**upgrade_feature_script_version** | Option<**bool**> |  |  |[default to false]
**preserve_source_ids** | Option<**bool**> |  |  |[default to false]
**document_id** | Option<**String**> |  |  |
**repoint_app_element_version_refs** | Option<**bool**> | Re-point the version references in APP elements to initial version in the new document |  |[default to false]

### Return type

[**models::BtDocumentElementProcessingInfo**](BTDocumentElementProcessingInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file_update_element

> models::BtDocumentElementProcessingInfo upload_file_update_element(did, wid, eid, link_document_id, parent_change_id, file, allow_faulty_parts, create_composite, create_drawing_if_possible, encoded_filename, extract_assembly_hierarchy, flatten_assemblies, format_name, join_adjacent_surfaces, location_element_id, location_group_id, location_position, notify_user, owner_id, parent_id, project_id, public, one_part_per_doc, split_assemblies_into_multiple_documents, store_in_document, translate, unit, upload_id, version_string, import_appearances, import_material_density, y_axis_is_up, import_within_document, use_iges_import_post_processing, upgrade_feature_script_version, preserve_source_ids, document_id, repoint_app_element_version_refs)
Update a blob element by uploading a file.

Request body parameters are multipart fields, so you must use `\"Content-Type\":\"multipart/form-data\"` in the request header.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**parent_change_id** | Option<**String**> | The id of the last change made to this application element. This can be retrieved from the response for any app element modification endpoint. |  |
**file** | Option<[**serde_json::Value**](serde_json::Value.md)> | The file to upload. |  |
**allow_faulty_parts** | Option<**bool**> | If true, and a part doesn't pass Onshape validation, it will be imported with faults. |  |
**create_composite** | Option<**bool**> | Not supported for importing into a single part studio. |  |
**create_drawing_if_possible** | Option<**bool**> |  |  |
**encoded_filename** | Option<**String**> | If the filename contains non-ASCII characters. Use this field to store the filename. |  |
**extract_assembly_hierarchy** | Option<**bool**> |  |  |
**flatten_assemblies** | Option<**bool**> | If the file is an assembly, or contains an assembly, setting this to True will import it as a Part Studio. In this case the assembly will be flattened to a set of parts in a Part Studio. There will be duplicate parts created whenever a part is instanced more than once. If False, it will be imported as an Assembly. |  |
**format_name** | Option<**String**> |  |  |
**join_adjacent_surfaces** | Option<**bool**> |  |  |
**location_element_id** | Option<**String**> |  |  |
**location_group_id** | Option<**String**> |  |  |
**location_position** | Option<**i32**> |  |  |[default to -1]
**notify_user** | Option<**bool**> |  |  |[default to true]
**owner_id** | Option<**String**> |  |  |
**parent_id** | Option<**String**> |  |  |
**project_id** | Option<**String**> |  |  |
**public** | Option<**bool**> |  |  |
**one_part_per_doc** | Option<**bool**> |  |  |[default to false]
**split_assemblies_into_multiple_documents** | Option<**bool**> |  |  |[default to false]
**store_in_document** | Option<**bool**> |  |  |[default to true]
**translate** | Option<**bool**> |  |  |[default to true]
**unit** | Option<**String**> |  |  |[default to ]
**upload_id** | Option<**String**> |  |  |
**version_string** | Option<**String**> |  |  |
**import_appearances** | Option<**bool**> | Face appearances defined on models will be imported. |  |[default to true]
**import_material_density** | Option<**bool**> | Material density defined on models will be imported. |  |[default to false]
**y_axis_is_up** | Option<**bool**> | If the file was created in a system that orients with Y Axis Up, the models would by default be brought into Onshape (a Z Axis Up system) with a flipped coordinate system. Toggle this value to reorient the axis system to match Onshape and display the model with the coordinates you expect. |  |
**import_within_document** | Option<**bool**> |  |  |
**use_iges_import_post_processing** | Option<**bool**> | Try getting optimized topology from IGES model. |  |[default to false]
**upgrade_feature_script_version** | Option<**bool**> |  |  |[default to false]
**preserve_source_ids** | Option<**bool**> |  |  |[default to false]
**document_id** | Option<**String**> |  |  |
**repoint_app_element_version_refs** | Option<**bool**> | Re-point the version references in APP elements to initial version in the new document |  |[default to false]

### Return type

[**models::BtDocumentElementProcessingInfo**](BTDocumentElementProcessingInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

