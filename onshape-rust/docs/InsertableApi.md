# \InsertableApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_latest_in_document**](InsertableApi.md#get_latest_in_document) | **GET** /insertables/d/{did}/latest | Get a list of things in this document that can be inserted elsewhere.



## get_latest_in_document

> models::BtListResponseBtInsertableInfo get_latest_in_document(did, include_parts, include_surfaces, include_sketches, include_reference_features, include_assemblies, include_feature_studios, include_blobs, allowed_blob_mime_types, exclude_newer_fs_versions, max_feature_script_version, include_part_studios, include_features, include_meshes, include_wires, include_flattened_bodies, include_applications, allowed_application_mime_types, include_composite_parts, include_fs_tables, include_fs_computed_part_property_functions, include_variables, include_variable_studios, allowed_blob_extensions, is_obsoletion)
Get a list of things in this document that can be inserted elsewhere.

* Returns only the latest revision of released insertables.  * Use the document ID (`did`) parameter to specify the source document, not the insertion target.  * For example, you can insert a custom Feature library into another custom Feature library, insert Parts into an Assembly or a Drawing, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
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

[**models::BtListResponseBtInsertableInfo**](BTListResponseBTInsertableInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

