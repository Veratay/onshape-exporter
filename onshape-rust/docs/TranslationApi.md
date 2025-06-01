# \TranslationApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_translation**](TranslationApi.md#create_translation) | **POST** /translations/d/{did}/w/{wid} | Import or upload a CAD file into Onshape, and translate the data into parts or assemblies.
[**delete_translation**](TranslationApi.md#delete_translation) | **DELETE** /translations/{tid} | Delete a translation request.
[**get_all_translator_formats**](TranslationApi.md#get_all_translator_formats) | **GET** /translations/translationformats | Get a list of formats this translation can use.
[**get_document_translations**](TranslationApi.md#get_document_translations) | **GET** /translations/d/{did} | Get information on an in-progress or completed translation by document ID.
[**get_translation**](TranslationApi.md#get_translation) | **GET** /translations/{tid} | Get information on an in-progress or completed translation by translation ID.



## create_translation

> models::BtTranslationRequestImportInfo create_translation(did, wid, file, allow_faulty_parts, create_composite, create_drawing_if_possible, encoded_filename, extract_assembly_hierarchy, flatten_assemblies, format_name, join_adjacent_surfaces, location_element_id, location_group_id, location_position, notify_user, owner_id, parent_id, project_id, public, one_part_per_doc, split_assemblies_into_multiple_documents, store_in_document, translate, unit, upload_id, version_string, import_appearances, import_material_density, y_axis_is_up, import_within_document, use_iges_import_post_processing, upgrade_feature_script_version, preserve_source_ids, document_id, repoint_app_element_version_refs)
Import or upload a CAD file into Onshape, and translate the data into parts or assemblies.

The API call may complete before the translation is finished. If `requestState = ACTIVE`, the translation can be polled until the state is either `DONE` or `FAILED`. Alternatively, a webhook callback can be registered for notification of translation completion (requires `Write` scope if `storeInDocument` is `true`).    See [API Guide: Import & Export](https://onshape-public.github.io/docs/api-adv/translation/) for examples.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
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

[**models::BtTranslationRequestImportInfo**](BTTranslationRequestImportInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_translation

> serde_json::Value delete_translation(tid)
Delete a translation request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_translator_formats

> Vec<models::BtModelFormatFullInfo> get_all_translator_formats()
Get a list of formats this translation can use.

Note that we don't necessarily support both import and export for any given format. See [API Guide: Model Translation](https://onshape-public.github.io/docs/api-adv/translation/) for more details.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::BtModelFormatFullInfo>**](BTModelFormatFullInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document_translations

> models::BtListResponseBtTranslationRequestInfo get_document_translations(did, offset, limit)
Get information on an in-progress or completed translation by document ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::BtListResponseBtTranslationRequestInfo**](BTListResponseBTTranslationRequestInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_translation

> models::BtTranslationRequestInfo get_translation(tid)
Get information on an in-progress or completed translation by translation ID.

When the translation is complete, `requestState` changes from `ACTIVE` to `DONE` or `FAILED`. See [API Guide: Model Translation](https://onshape-public.github.io/docs/api-adv/translation/) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** |  | [required] |

### Return type

[**models::BtTranslationRequestInfo**](BTTranslationRequestInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

