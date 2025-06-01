# \PartStudioApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_part_studio_feature**](PartStudioApi.md#add_part_studio_feature) | **POST** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/features | Add a feature to the Part Studio's Feature List.
[**compare_part_studios**](PartStudioApi.md#compare_part_studios) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/compare | Get the differences between two Part Studios in a single document.
[**create_part_studio**](PartStudioApi.md#create_part_studio) | **POST** /partstudios/d/{did}/w/{wid} | Create a new Part Studio in a document.
[**create_part_studio_export_step**](PartStudioApi.md#create_part_studio_export_step) | **POST** /partstudios/d/{did}/{wv}/{wvid}/e/{eid}/export/step | Asynchronously export a Part Studio to STEP.
[**create_part_studio_translation**](PartStudioApi.md#create_part_studio_translation) | **POST** /partstudios/d/{did}/{wv}/{wvid}/e/{eid}/translations | Asynchronously export a Part Studio to another format.
[**delete_part_studio_feature**](PartStudioApi.md#delete_part_studio_feature) | **DELETE** /partstudios/d/{did}/w/{wid}/e/{eid}/features/featureid/{fid} | Delete a Part Studio feature.
[**eval_feature_script**](PartStudioApi.md#eval_feature_script) | **POST** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/featurescript | Evaluate the FeatureScript snippet for a Part Studio.
[**export_parasolid**](PartStudioApi.md#export_parasolid) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/parasolid | Synchronously export a Part Studio to a Parasolid file.
[**export_part_studio_gltf**](PartStudioApi.md#export_part_studio_gltf) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/gltf | Synchronously export a Part Studio to a glTF file.
[**export_part_studio_stl**](PartStudioApi.md#export_part_studio_stl) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/stl | Synchronously export a Part Studio to an STL file.
[**get_feature_script_representation**](PartStudioApi.md#get_feature_script_representation) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/featurescriptrepresentation | Get the FeatureScript representation of a Part Studio.
[**get_feature_script_table**](PartStudioApi.md#get_feature_script_table) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/fstable | Compute and return a FeatureScript table for a Part Studio.
[**get_part_studio_body_details**](PartStudioApi.md#get_part_studio_body_details) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/bodydetails | Get the body details for a Part Studio.
[**get_part_studio_bounding_boxes**](PartStudioApi.md#get_part_studio_bounding_boxes) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/boundingboxes | Get the bounding boxes for a Part Studio.
[**get_part_studio_edges**](PartStudioApi.md#get_part_studio_edges) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/tessellatededges | Get a list of all edges in a Part Studio.
[**get_part_studio_faces**](PartStudioApi.md#get_part_studio_faces) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/tessellatedfaces | Get a list of all faces in a Part Studio.
[**get_part_studio_feature_specs**](PartStudioApi.md#get_part_studio_feature_specs) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/featurespecs | Get the specs for a Part Studio feature.
[**get_part_studio_features**](PartStudioApi.md#get_part_studio_features) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/features | Get a list of features instantiated in the Part Studio.
[**get_part_studio_mass_properties**](PartStudioApi.md#get_part_studio_mass_properties) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/massproperties | Get the mass properties for a Part Studio.
[**get_part_studio_named_views**](PartStudioApi.md#get_part_studio_named_views) | **GET** /partstudios/d/{did}/e/{eid}/namedViews | Get a list of all named views that exist in the Part Studio.
[**get_part_studio_shaded_views**](PartStudioApi.md#get_part_studio_shaded_views) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/shadedviews | Get a list of shaded views for a Part Studio.
[**translate_ids**](PartStudioApi.md#translate_ids) | **POST** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/idtranslations | Find corresponding deterministic IDs from a source document microversion at the target version.
[**update_features**](PartStudioApi.md#update_features) | **POST** /partstudios/d/{did}/w/{wid}/e/{eid}/features/updates | Update multiple features in a Part Studio
[**update_part_studio_feature**](PartStudioApi.md#update_part_studio_feature) | **POST** /partstudios/d/{did}/w/{wid}/e/{eid}/features/featureid/{fid} | Update the definition of a Part Studio feature.
[**update_rollback**](PartStudioApi.md#update_rollback) | **POST** /partstudios/d/{did}/w/{wid}/e/{eid}/features/rollback | Move the Feature List rollback bar in the Part Studio.



## add_part_studio_feature

> models::BtFeatureDefinitionResponse1617 add_part_studio_feature(did, wvm, wvmid, eid, bt_feature_definition_call1406)
Add a feature to the Part Studio's Feature List.

The feature is added immediately before the rollback bar. Any geometry IDs specified in the feature must be valid at that point in the feature tree.    See the [Features API Guide](https://onshape-public.github.io/docs/api-adv/featureaccess/) for additional information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_feature_definition_call1406** | Option<[**BtFeatureDefinitionCall1406**](BtFeatureDefinitionCall1406.md)> |  |  |

### Return type

[**models::BtFeatureDefinitionResponse1617**](BTFeatureDefinitionResponse-1617.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compare_part_studios

> models::BtRootDiffInfo compare_part_studios(did, wvm, wvmid, eid, workspace_id, version_id, microversion_id, source_configuration, target_configuration, link_document_id)
Get the differences between two Part Studios in a single document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**workspace_id** | Option<**String**> |  |  |
**version_id** | Option<**String**> |  |  |
**microversion_id** | Option<**String**> |  |  |
**source_configuration** | Option<**String**> |  |  |
**target_configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

[**models::BtRootDiffInfo**](BTRootDiffInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_part_studio

> models::BtDocumentElementInfo create_part_studio(did, wid, bt_model_element_params)
Create a new Part Studio in a document.

See the [Part Studios API Guide](https://onshape-public.github.io/docs/api-adv/partstudios/) for details and tutorials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wid** | **String** | Workspace ID. | [required] |
**bt_model_element_params** | [**BtModelElementParams**](BtModelElementParams.md) |  | [required] |

### Return type

[**models::BtDocumentElementInfo**](BTDocumentElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_part_studio_export_step

> models::BtTranslationRequestInfo create_part_studio_export_step(did, wv, wvid, eid, btb_step_export_params)
Asynchronously export a Part Studio to STEP.

Creates an asynchronous export of a Part Studio to STEP.  * See [API Guide: Asynchronous Exports](https://onshape-public.github.io/docs/api-adv/translation/#asynchronous-exports) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wv** | **String** | One of w or v corresponding to whether a workspace or version was specified. | [required] |
**wvid** | **String** | Workspace (w) or Version (v) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**btb_step_export_params** | [**BtbStepExportParams**](BtbStepExportParams.md) |  | [required] |

### Return type

[**models::BtTranslationRequestInfo**](BTTranslationRequestInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_part_studio_translation

> models::BtTranslationRequestInfo create_part_studio_translation(did, wv, wvid, eid, bt_translate_format_params)
Asynchronously export a Part Studio to another format.

Creates an asynchronous export of a Part Studio to another file format.  * Can take longer than synchronous export options, but supports more formats and provides more control on tessellation and other settings.  * Use `formatName` in the JSON request body to specify the export file type. Use [Translations/getAllTranslatorFormats](#/Translation/getAllTranslatorFormats) to get a list of valid export file formats. ` * Set `storeInDocument` to `false` to export to a data file. Set to `true` to export to a blob element in the same document.  * See [API Guide: Asynchronous Exports](https://onshape-public.github.io/docs/api-adv/translation/#asynchronous-exports) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wv** | **String** | One of w or v corresponding to whether a workspace or version was specified. | [required] |
**wvid** | **String** | Workspace (w) or Version (v) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**bt_translate_format_params** | [**BtTranslateFormatParams**](BtTranslateFormatParams.md) |  | [required] |

### Return type

[**models::BtTranslationRequestInfo**](BTTranslationRequestInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_part_studio_feature

> models::BtFeatureApiBase1430 delete_part_studio_feature(did, wid, eid, fid)
Delete a Part Studio feature.

See the [Features API Guide](https://onshape-public.github.io/docs/api-adv/featureaccess/) for additional information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wid** | **String** | Workspace ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**fid** | **String** | The id of the feature being updated. This id should be URL encoded and must match the featureId found in the serialized structure | [required] |

### Return type

[**models::BtFeatureApiBase1430**](BTFeatureApiBase-1430.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eval_feature_script

> models::BtFeatureScriptEvalResponse1859 eval_feature_script(did, wvm, wvmid, eid, link_document_id, configuration, rollback_bar_index, element_microversion_id, bt_feature_script_eval_call2377)
Evaluate the FeatureScript snippet for a Part Studio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
**bt_feature_script_eval_call2377** | Option<[**BtFeatureScriptEvalCall2377**](BtFeatureScriptEvalCall2377.md)> |  |  |

### Return type

[**models::BtFeatureScriptEvalResponse1859**](BTFeatureScriptEvalResponse-1859.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_parasolid

> export_parasolid(did, wvm, wvmid, eid, part_ids, version, include_export_ids, configuration, link_document_id, binary_export)
Synchronously export a Part Studio to a Parasolid file.

Creates a synchronous export of the Part Studio (with limited tessellation settings) to a Parasolid file.  * Returns a 307 redirect from which to download the exported file.  * Export is much faster than asynchronous endpoints at the expense of limited control on tessellation settings.  * Use the [PartStudio/createPartStudioTranslation](#/PartStudio/createPartStudioTranslation) asynchronous export for greater control.    See [API Guide: Synchronous Exports](https://onshape-public.github.io/docs/api-adv/translation/#synchronous-exports) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**part_ids** | Option<**String**> | IDs of the parts to retrieve. Use comma-separated IDs for multiple parts (example: partIds=JHK,JHD). |  |
**version** | Option<**String**> | Parasolid version |  |[default to 0]
**include_export_ids** | Option<**bool**> | Whether topology ids should be exported as parasolid attributes |  |[default to false]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |
**binary_export** | Option<**bool**> | Whether to use binary parasolid format instead of text |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_part_studio_gltf

> models::GlTf export_part_studio_gltf(did, wvm, wvmid, eid, link_document_id, configuration, rollback_bar_index, element_microversion_id, part_id, angle_tolerance, chord_tolerance, precomputed_level_of_detail, output_separate_face_nodes, face_id, output_face_appearances, max_facet_width)
Synchronously export a Part Studio to a glTF file.

Creates a synchronous export of the Part Studio (with limited tessellation settings) to a glTF file.  * Returns a 307 redirect from which to download the exported file.  * Export is much faster than asynchronous endpoints at the expense of limited control on tessellation settings.  * Use the [PartStudio/createPartStudioTranslation](#/PartStudio/createPartStudioTranslation) asynchronous export for greater control.    See [API Guide: Synchronous Exports](https://onshape-public.github.io/docs/api-adv/translation/#synchronous-exports) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
**part_id** | Option<[**Vec<String>**](String.md)> |  |  |
**angle_tolerance** | Option<**f64**> |  |  |
**chord_tolerance** | Option<**f64**> |  |  |
**precomputed_level_of_detail** | Option<**String**> |  |  |
**output_separate_face_nodes** | Option<**bool**> |  |  |[default to false]
**face_id** | Option<[**Vec<String>**](String.md)> |  |  |
**output_face_appearances** | Option<**bool**> |  |  |[default to false]
**max_facet_width** | Option<**f64**> |  |  |

### Return type

[**models::GlTf**](GlTF.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: model/gltf+json;charset=UTF-8;qs=0.08, model/gltf-binary;qs=0.08

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_part_studio_stl

> export_part_studio_stl(did, wvm, wvmid, eid, part_ids, mode, grouping, scale, units, angle_tolerance, chord_tolerance, max_facet_width, min_facet_width, configuration, link_document_id)
Synchronously export a Part Studio to an STL file.

Creates a synchronous export of the Part Studio (with limited tessellation settings) to an STL file.  * Returns a 307 redirect from which to download the exported file.  * Export is much faster than asynchronous endpoints at the expense of limited control on tessellation settings.  * Use the [PartStudio/createPartStudioTranslation](#/PartStudio/createPartStudioTranslation) asynchronous export for greater control.    See [API Guide: Synchronous Exports](https://onshape-public.github.io/docs/api-adv/translation/#synchronous-exports) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**part_ids** | Option<**String**> | IDs of the parts to retrieve. Use comma-separated IDs for multiple parts (example: partIds=JHK,JHD). |  |
**mode** | Option<**String**> | Type of file: text, binary |  |[default to text]
**grouping** | Option<**bool**> | Whether parts should be exported as a group or individually in a .zip file |  |[default to true]
**scale** | Option<**f64**> | Scale for measurements. |  |[default to 1.0]
**units** | Option<**String**> | Units for the element: `METER` | `CENTIMETER` | `MILLIMETER` | `INCH` | `FOOT` | `YARD` |  |[default to inch]
**angle_tolerance** | Option<**f64**> | Angle tolerance (in radians). This specifies the limit on the sum of the angular deviations of a tessellation chord from the tangent vectors at two chord endpoints. The specified value must be less than PI/2. This parameter currently has a default value chosen based on the complexity of the parts being tessellated. |  |
**chord_tolerance** | Option<**f64**> | Chord tolerance (in meters). This specifies the limit on the maximum deviation of a tessellation chord from the true surface/edge. This parameter currently has a default value chosen based on the size and complexity of the parts being tessellated. |  |
**max_facet_width** | Option<**f64**> | Max facet width. This specifies the limit on the size of any side of a tessellation facet. |  |
**min_facet_width** | Option<**f64**> | Max facet width. This specifies the limit on the size of any side of a tessellation facet. |  |
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feature_script_representation

> models::BtpModule234 get_feature_script_representation(did, wvm, wvmid, eid, link_document_id, configuration, rollback_bar_index, element_microversion_id)
Get the FeatureScript representation of a Part Studio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |

### Return type

[**models::BtpModule234**](BTPModule-234.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feature_script_table

> models::BtApiTableList1223 get_feature_script_table(did, wvm, wvmid, eid, table_type, configuration, table_namespace, table_parameters, part_id, link_document_id)
Compute and return a FeatureScript table for a Part Studio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**table_type** | **String** | May be any standard table type (i.e., `holeTable`) or custom table type name defined in FeatureScript.  If using a custom table type, `tableNamespace` must also be specified. | [required] |
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |
**table_namespace** | Option<**String**> | Namespace of the custom table in FS. Must be in the form of:  * `e{eid}::m{mid}` if the FS and PS tabs are in the same workspace  * `d{did}::v{vid}::e{eid}::m{mid}` if the tabs are in different workspace   Obtain the microversion id (`{mid}`) with [this endpoint](#/Document/getElementsInDocument) called on the FS tab.  Leave blank if using a standard table. Required if using a custom `tableType`. |  |
**table_parameters** | Option<**String**> | Include all parameters for the table. i.e., `customBool=false` |  |
**part_id** | Option<**String**> | ID of the part to retrieve. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

[**models::BtApiTableList1223**](BTApiTableList-1223.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_part_studio_body_details

> models::BtExportModelBodiesResponse734 get_part_studio_body_details(did, wvm, wvmid, eid, link_document_id, configuration, rollback_bar_index, element_microversion_id, part_ids, include_surfaces, include_composite_parts, include_geometric_data)
Get the body details for a Part Studio.

See the [Part Studios API Guide](https://onshape-public.github.io/docs/api-adv/partstudios/) for details and tutorials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
**part_ids** | Option<[**Vec<String>**](String.md)> | If specified, the response will only include body details for the specific parts as indicated here by their corresponding Id |  |
**include_surfaces** | Option<**bool**> | Whether or not surfaces should be included in the response. |  |[default to false]
**include_composite_parts** | Option<**bool**> | Whether or not composite parts should be included in the response. |  |[default to false]
**include_geometric_data** | Option<**bool**> | Whether or not geometric data should be included in the response. |  |[default to true]

### Return type

[**models::BtExportModelBodiesResponse734**](BTExportModelBodiesResponse-734.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_part_studio_bounding_boxes

> models::BtBoundingBoxInfo get_part_studio_bounding_boxes(did, wvm, wvmid, eid, include_hidden, include_wire_bodies, configuration, link_document_id)
Get the bounding boxes for a Part Studio.

This endpoint does not result in a tight bounding box. The values returned are meant for graphics and visualization, and are approximate.  To calculate a tight bounding box, see the [FeatureScript API Guide](https://onshape-public.github.io/docs/api-adv/fs/#calculate-a-tight-bounding-box). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**include_hidden** | Option<**bool**> | Whether or not to include bounding boxes for hidden parts. |  |[default to false]
**include_wire_bodies** | Option<**bool**> | Whether to include wire bodies in the bounding box. |  |[default to true]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

[**models::BtBoundingBoxInfo**](BTBoundingBoxInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_part_studio_edges

> models::BtExportTessellatedEdgesResponse327 get_part_studio_edges(did, wvm, wvmid, eid, link_document_id, configuration, rollback_bar_index, element_microversion_id, part_id, angle_tolerance, chord_tolerance, precomputed_level_of_detail, edge_id)
Get a list of all edges in a Part Studio.

Returns the edges as tessellated data and includes display data.  Coordinates are in meters (m).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
**part_id** | Option<[**Vec<String>**](String.md)> |  |  |
**angle_tolerance** | Option<**f64**> |  |  |
**chord_tolerance** | Option<**f64**> |  |  |
**precomputed_level_of_detail** | Option<**String**> |  |  |
**edge_id** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::BtExportTessellatedEdgesResponse327**](BTExportTessellatedEdgesResponse-327.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_part_studio_faces

> models::BtExportTessellatedFacesResponse898 get_part_studio_faces(did, wvm, wvmid, eid, link_document_id, configuration, rollback_bar_index, element_microversion_id, part_id, angle_tolerance, chord_tolerance, precomputed_level_of_detail, face_id, output_face_appearances, max_facet_width, output_vertex_normals, output_facet_normals, output_texture_coordinates, output_index_table, output_error_faces, combine_composite_part_constituents)
Get a list of all faces in a Part Studio.

Coordinates are in meters (m).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
**part_id** | Option<[**Vec<String>**](String.md)> |  |  |
**angle_tolerance** | Option<**f64**> |  |  |
**chord_tolerance** | Option<**f64**> |  |  |
**precomputed_level_of_detail** | Option<**String**> |  |  |
**face_id** | Option<[**Vec<String>**](String.md)> |  |  |
**output_face_appearances** | Option<**bool**> |  |  |[default to false]
**max_facet_width** | Option<**f64**> |  |  |
**output_vertex_normals** | Option<**bool**> |  |  |[default to false]
**output_facet_normals** | Option<**bool**> |  |  |[default to true]
**output_texture_coordinates** | Option<**bool**> |  |  |[default to false]
**output_index_table** | Option<**bool**> |  |  |[default to false]
**output_error_faces** | Option<**bool**> |  |  |[default to false]
**combine_composite_part_constituents** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtExportTessellatedFacesResponse898**](BTExportTessellatedFacesResponse-898.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_part_studio_feature_specs

> models::BtFeatureSpecsResponse664 get_part_studio_feature_specs(did, wvm, wvmid, eid)
Get the specs for a Part Studio feature.

Returns a list of feature specs available within the Part Studio. A feature spec provides a data description of the feature's interface to a feature.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |

### Return type

[**models::BtFeatureSpecsResponse664**](BTFeatureSpecsResponse-664.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_part_studio_features

> models::BtFeatureListResponse2457 get_part_studio_features(did, wvm, wvmid, eid, link_document_id, configuration, rollback_bar_index, element_microversion_id, include_geometry_ids, feature_id, no_sketch_geometry)
Get a list of features instantiated in the Part Studio.

See the [Features API Guide](https://onshape-public.github.io/docs/api-adv/featureaccess/) for additional information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
**include_geometry_ids** | Option<**bool**> | If true, include the underlying geometry IDs in the feature definition. |  |[default to true]
**feature_id** | Option<[**Vec<String>**](String.md)> | ID of a feature; repeat query param to add more than one |  |
**no_sketch_geometry** | Option<**bool**> | Whether or not to output simple sketch info without geometry |  |[default to false]

### Return type

[**models::BtFeatureListResponse2457**](BTFeatureListResponse-2457.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_part_studio_mass_properties

> models::BtMassPropertiesBulkInfo get_part_studio_mass_properties(did, wvm, wvmid, eid, link_document_id, configuration, rollback_bar_index, element_microversion_id, part_id, mass_as_group, use_mass_property_overrides)
Get the mass properties for a Part Studio.

Parts must have density. The returned schema includes the same information as in the Onshape [Mass Properties Tool](https://cad.onshape.com/help/Content/massprops-ps.htm).    When three values are returned:   * The first is the calculated value.   * The second is the minimum possible value, considering tolerance.   * The third is the maximum possible value, considering tolerance.     See the [Part Studios API Guide](https://onshape-public.github.io/docs/api-adv/partstudios/) for details and tutorials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
**part_id** | Option<[**Vec<String>**](String.md)> |  |  |
**mass_as_group** | Option<**bool**> | If true, specified parts will be evaluated as a single object instead of individually |  |[default to true]
**use_mass_property_overrides** | Option<**bool**> | If true, use the user mass property overrides when calculated mass properties |  |[default to false]

### Return type

[**models::BtMassPropertiesBulkInfo**](BTMassPropertiesBulkInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_part_studio_named_views

> models::BtNamedViewsInfo get_part_studio_named_views(did, eid, link_document_id, skip_perspective, include_section_cut_views)
Get a list of all named views that exist in the Part Studio.

Returns a map from view name to view data for the given element. See the [Part Studios API Guide](https://onshape-public.github.io/docs/api-adv/partstudios/) for details and tutorials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**eid** | **String** |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**skip_perspective** | Option<**bool**> |  |  |[default to true]
**include_section_cut_views** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtNamedViewsInfo**](BTNamedViewsInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_part_studio_shaded_views

> models::BtShadedViewsInfo get_part_studio_shaded_views(did, wvm, wvmid, eid, view_matrix, output_height, output_width, pixel_size, edges, show_all_parts, include_surfaces, use_anti_aliasing, include_wires, configuration, link_document_id)
Get a list of shaded views for a Part Studio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**view_matrix** | Option<**String**> | 12-number view matrix (comma-separated), or one of the following named views: top, bottom, front, back, left, right The 12 entries in the view matrix form three rows and four columns, which is a linear transformation applied to the model itself. The matrix's first three columns maps the coordinate axes of the model to the coordinate axes of the view, and the fourth column translates the origin (in meters). The view coordinates have x pointing right, y pointing up, and z pointing towards the viewer, while a front view of the model has x pointing right, y pointing away from the viewer, and z pointing up. For example, the identity matrix viewMatrix=1,0,0,0,0,1,0,0,0,0,1,0 corresponds to the top view, and viewMatrix=0.612,0.612,0,0,-0.354,0.354,0.707,0,0.707,-0.707,0.707,0 corresponds (approximately) to the isometric view. The first three columns of the view matrix should be orthonormal and have a positive determinant.  If this is not the case, view behavior may be undefined. |  |[default to front]
**output_height** | Option<**i32**> | Output image height (in pixels) |  |[default to 500]
**output_width** | Option<**i32**> | Output image width (in pixels) |  |[default to 500]
**pixel_size** | Option<**f64**> | Height and width represented by each pixel (in meters). If the value is 0, the display will be sized to fit the output image dimensions. |  |[default to 0.003]
**edges** | Option<**String**> | The treatment to be applied to edges in the display. Options are show: show visible edges, hide: hide visible edges. |  |[default to show]
**show_all_parts** | Option<**bool**> | Whether or not all parts should be shown in the element, regardless of user setting. If false, the visibility setting made by the user will be reflected in the image. If true, all parts will be shown. |  |[default to false]
**include_surfaces** | Option<**bool**> | Whether or not surfaces should be shown in the element. It is applicable only when showAllParts is true. If false, surfaces will be excluded. If true, all surfaces will be shown. |  |[default to false]
**use_anti_aliasing** | Option<**bool**> | If true, an anti-aliasing factor will be used to smooth model boundaries in the final image result. If false, the image will be rasterized at the given resolution. Setting to true can have negative performance implications with respect to rendering time and memory usage. If a high-resolution image is requested and anti-aliasing is turned on, the server may not be able to fulfill the request. |  |[default to false]
**include_wires** | Option<**bool**> |  |  |[default to false]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

[**models::BtShadedViewsInfo**](BTShadedViewsInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## translate_ids

> models::BtidTranslationInfo translate_ids(did, wvm, wvmid, eid, btid_translation_params)
Find corresponding deterministic IDs from a source document microversion at the target version.

* Deterministic IDs are only valid for one microversion.  * This maps deterministic IDs between microversions in an attempt to find the corresponding entities in each version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**btid_translation_params** | [**BtidTranslationParams**](BtidTranslationParams.md) |  | [required] |

### Return type

[**models::BtidTranslationInfo**](BTIdTranslationInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_features

> models::BtUpdateFeaturesResponse1333 update_features(did, wid, eid, bt_update_features_call1748)
Update multiple features in a Part Studio

This API accepts a list of features (that must already exist in the Part Studio) to update. This call does not fully redefine the features; it updates only the parameters supplied in the top-level feature structure, and optionally can update feature suppression attributes.  See the [Features API Guide](https://onshape-public.github.io/docs/api-adv/featureaccess/) for additional information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wid** | **String** | Workspace ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**bt_update_features_call1748** | Option<[**BtUpdateFeaturesCall1748**](BtUpdateFeaturesCall1748.md)> | feature The serialized feature definition |  |

### Return type

[**models::BtUpdateFeaturesResponse1333**](BTUpdateFeaturesResponse-1333.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_part_studio_feature

> models::BtFeatureDefinitionResponse1617 update_part_studio_feature(did, wid, eid, fid, bt_feature_definition_call1406)
Update the definition of a Part Studio feature.

Replaces an existing feature in the location of the existing feature. See the [Features API Guide](https://onshape-public.github.io/docs/api-adv/featureaccess/) for additional information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wid** | **String** | Workspace ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**fid** | **String** | The id of the feature being updated. This id should be URL encoded and must match the featureId found in the serialized structure | [required] |
**bt_feature_definition_call1406** | Option<[**BtFeatureDefinitionCall1406**](BtFeatureDefinitionCall1406.md)> | feature The serialized feature definition |  |

### Return type

[**models::BtFeatureDefinitionResponse1617**](BTFeatureDefinitionResponse-1617.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_rollback

> models::BtSetFeatureRollbackResponse1042 update_rollback(did, wid, eid, body)
Move the Feature List rollback bar in the Part Studio.

Replace `\"string\"` in the request body with an object that specifies the new location for the rollback bar:   `{ \"rollbackIndex\": integer }`   For example: `{ \"rollbackIndex\": 2 }`   Set to `-1` to move the rollback bar to the end of the list.   See the [Part Studios API Guide](https://onshape-public.github.io/docs/api-adv/partstudios/) for details and tutorials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wid** | **String** | Workspace ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**body** | **String** | The index at which the rollback index should be placed. Features  with entry index (0-based) higher than or equal to the value are rolled back. Value of -1 is treated  as an alias for \"end of feature list\". Otherwise the value must be in the range 0 to the number of  entries in the feature list | [required] |

### Return type

[**models::BtSetFeatureRollbackResponse1042**](BTSetFeatureRollbackResponse-1042.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

