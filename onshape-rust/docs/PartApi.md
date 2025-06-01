# \PartApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**export_part_gltf**](PartApi.md#export_part_gltf) | **GET** /parts/d/{did}/{wvm}/{wvmid}/e/{eid}/partid/{partid}/gltf | Synchronously export a part to a glTF file.
[**export_ps**](PartApi.md#export_ps) | **GET** /parts/d/{did}/{wvm}/{wvmid}/e/{eid}/partid/{partid}/parasolid | Synchronously export a part to a Parasolid file.
[**export_stl**](PartApi.md#export_stl) | **GET** /parts/d/{did}/{wvm}/{wvmid}/e/{eid}/partid/{partid}/stl | Synchronously export a part to an STL file.
[**get_bend_table**](PartApi.md#get_bend_table) | **GET** /parts/d/{did}/{wvm}/{wvmid}/e/{eid}/partid/{partid}/sheetmetal/bendtable | Get a part's sheet metal bend table.
[**get_body_details**](PartApi.md#get_body_details) | **GET** /parts/d/{did}/{wvm}/{wvmid}/e/{eid}/partid/{partid}/bodydetails | Get a part's body details.
[**get_bounding_boxes**](PartApi.md#get_bounding_boxes) | **GET** /parts/d/{did}/{wvm}/{wvmid}/e/{eid}/partid/{partid}/boundingboxes | Get a part's bounding box details.
[**get_edges**](PartApi.md#get_edges) | **GET** /parts/d/{did}/{wvm}/{wvmid}/e/{eid}/partid/{partid}/tessellatededges | Get a list of a part's tessellation edges.
[**get_faces1**](PartApi.md#get_faces1) | **GET** /parts/d/{did}/{wvm}/{wvmid}/e/{eid}/partid/{partid}/tessellatedfaces | Get a list of a part's tessellation faces.
[**get_mass_properties**](PartApi.md#get_mass_properties) | **GET** /parts/d/{did}/{wvm}/{wvmid}/e/{eid}/partid/{partid}/massproperties | Get a part's mass properties.
[**get_part_shaded_views**](PartApi.md#get_part_shaded_views) | **GET** /parts/d/{did}/{wvm}/{wvmid}/e/{eid}/partid/{partid}/shadedviews | Get a part's shaded views.
[**get_parts_wmv**](PartApi.md#get_parts_wmv) | **GET** /parts/d/{did}/{wvm}/{wvmid} | Get all parts in a workspace, version, or microversion.
[**get_parts_wmve**](PartApi.md#get_parts_wmve) | **GET** /parts/d/{did}/{wvm}/{wvmid}/e/{eid} | Get all parts in an element.



## export_part_gltf

> models::GlTf export_part_gltf(did, wvm, wvmid, eid, partid, link_document_id, configuration, rollback_bar_index, element_microversion_id, angle_tolerance, chord_tolerance, precomputed_level_of_detail, output_separate_face_nodes, face_id, output_face_appearances, max_facet_width)
Synchronously export a part to a glTF file.

Creates a synchronous export of the part (with limited tessellation settings) to a glTF file.  * Returns a 307 redirect from which to download the exported file.  * Export is much faster than asynchronous endpoints at the expense of limited control on tessellation settings.  * Use the [PartStudio/createPartStudioTranslation](#/PartStudio/createPartStudioTranslation) asynchronous export for greater control.    See [API Guide: Synchronous Exports](https://onshape-public.github.io/docs/api-adv/translation/#synchronous-exports) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**partid** | **String** |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
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


## export_ps

> export_ps(did, wvm, wvmid, eid, partid, version, configuration, link_document_id)
Synchronously export a part to a Parasolid file.

Creates a synchronous export of the part (with limited tessellation settings) to a Parasolid file.  * Returns a 307 redirect from which to download the exported file.  * Export is much faster than asynchronous endpoints at the expense of limited control on tessellation settings.  * Use the [PartStudio/createPartStudioTranslation](#/PartStudio/createPartStudioTranslation) asynchronous export for greater control.    See [API Guide: Synchronous Exports](https://onshape-public.github.io/docs/api-adv/translation/#synchronous-exports) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**partid** | **String** |  | [required] |
**version** | Option<**String**> |  |  |[default to 0]
**configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_stl

> serde_json::Value export_stl(did, wvm, wvmid, eid, partid, mode, grouping, scale, units, angle_tolerance, chord_tolerance, max_facet_width, min_facet_width, configuration, link_document_id)
Synchronously export a part to an STL file.

Creates a synchronous export of the part (with limited tessellation settings) to an STL file.  * Returns a 307 redirect from which to download the exported file.  * Export is much faster than asynchronous endpoints at the expense of limited control on tessellation settings.  * Use the [PartStudio/createPartStudioTranslation](#/PartStudio/createPartStudioTranslation) asynchronous export for greater control.    See [API Guide: Synchronous Exports](https://onshape-public.github.io/docs/api-adv/translation/#synchronous-exports) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**partid** | **String** |  | [required] |
**mode** | Option<**String**> |  |  |[default to text]
**grouping** | Option<**bool**> |  |  |[default to true]
**scale** | Option<**f64**> |  |  |[default to 1.0]
**units** | Option<**String**> |  |  |[default to inch]
**angle_tolerance** | Option<**f64**> |  |  |
**chord_tolerance** | Option<**f64**> |  |  |
**max_facet_width** | Option<**f64**> |  |  |
**min_facet_width** | Option<**f64**> |  |  |
**configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bend_table

> models::BtTableResponse1546 get_bend_table(did, wvm, wvmid, eid, partid, link_document_id)
Get a part's sheet metal bend table.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**partid** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |

### Return type

[**models::BtTableResponse1546**](BTTableResponse-1546.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_body_details

> models::BtExportModelBodiesResponse734 get_body_details(did, wvm, wvmid, eid, partid, link_document_id, configuration, rollback_bar_index, element_microversion_id, include_geometric_data)
Get a part's body details.

All coordinates are in meters (m).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**partid** | **String** |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
**include_geometric_data** | Option<**bool**> | Whether or not geometric data should be included in the response. |  |[default to true]

### Return type

[**models::BtExportModelBodiesResponse734**](BTExportModelBodiesResponse-734.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bounding_boxes

> models::BtBoundingBoxInfo get_bounding_boxes(did, wvm, wvmid, eid, partid, include_hidden, configuration, link_document_id)
Get a part's bounding box details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**partid** | **String** |  | [required] |
**include_hidden** | Option<**bool**> |  |  |[default to false]
**configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

[**models::BtBoundingBoxInfo**](BTBoundingBoxInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_edges

> models::BtExportTessellatedEdgesResponse327 get_edges(did, wvm, wvmid, eid, partid, link_document_id, configuration, rollback_bar_index, element_microversion_id, angle_tolerance, chord_tolerance, precomputed_level_of_detail, edge_id)
Get a list of a part's tessellation edges.

Returns the coordinates (in meters) of each edge's endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**partid** | **String** |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
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


## get_faces1

> models::BtExportTessellatedFacesResponse898 get_faces1(did, wvm, wvmid, eid, partid, link_document_id, configuration, rollback_bar_index, element_microversion_id, angle_tolerance, chord_tolerance, precomputed_level_of_detail, face_id, output_face_appearances, max_facet_width, output_vertex_normals, output_facet_normals, output_texture_coordinates, output_index_table, output_error_faces, combine_composite_part_constituents)
Get a list of a part's tessellation faces.

Coordinates are in meters (m).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**partid** | **String** |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
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


## get_mass_properties

> models::BtMassPropertiesBulkInfo get_mass_properties(did, wvm, wvmid, eid, partid, link_document_id, configuration, rollback_bar_index, element_microversion_id, infer_metadata_owner, use_mass_property_overrides)
Get a part's mass properties.

Parts must have density. The returned schema includes the same information as in the Onshape [Mass Properties Tool](https://cad.onshape.com/help/Content/massprops-ps.htm).  When three values are returned:   * The first is the calculated value.   * The second is the minimum possible value, considering tolerance.   * The third is the maximum possible value, considering tolerance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**partid** | **String** |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**rollback_bar_index** | Option<**i32**> | Index specifying the location of the rollback bar when the call is evaluated. A -1 indicates that it should be at the end of the featurelist. |  |[default to -1]
**element_microversion_id** | Option<**String**> | A specific element microversion in which to evaluate the request. |  |
**infer_metadata_owner** | Option<**bool**> |  |  |[default to true]
**use_mass_property_overrides** | Option<**bool**> | If true, use the user mass property overrides when calculated mass properties |  |[default to false]

### Return type

[**models::BtMassPropertiesBulkInfo**](BTMassPropertiesBulkInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_part_shaded_views

> models::BtShadedViewsInfo get_part_shaded_views(did, wvm, wvmid, eid, partid, view_matrix, output_height, output_width, pixel_size, edges, use_anti_aliasing, configuration, link_document_id)
Get a part's shaded views.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**partid** | **String** |  | [required] |
**view_matrix** | Option<**String**> |  |  |[default to front]
**output_height** | Option<**i32**> |  |  |[default to 500]
**output_width** | Option<**i32**> |  |  |[default to 500]
**pixel_size** | Option<**f64**> |  |  |[default to 0.003]
**edges** | Option<**String**> |  |  |[default to show]
**use_anti_aliasing** | Option<**bool**> |  |  |[default to false]
**configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

[**models::BtShadedViewsInfo**](BTShadedViewsInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_parts_wmv

> Vec<models::BtPartMetadataInfo> get_parts_wmv(did, wvm, wvmid, element_id, link_document_id, configuration, with_thumbnails, include_property_defaults, include_flat_parts)
Get all parts in a workspace, version, or microversion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**element_id** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> |  |  |[default to ]
**with_thumbnails** | Option<**bool**> | Whether or not to include thumbnails (not supported for microversion) |  |[default to false]
**include_property_defaults** | Option<**bool**> | If true, include metadata schema property defaults in response |  |[default to false]
**include_flat_parts** | Option<**bool**> |  |  |

### Return type

[**Vec<models::BtPartMetadataInfo>**](BTPartMetadataInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_parts_wmve

> Vec<models::BtPartMetadataInfo> get_parts_wmve(did, wvm, wvmid, eid, with_thumbnails, include_property_defaults, include_flat_parts, configuration, link_document_id)
Get all parts in an element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**with_thumbnails** | Option<**bool**> | Whether or not to include thumbnails (not supported for microversion) |  |[default to false]
**include_property_defaults** | Option<**bool**> | If true, include metadata schema property defaults in response |  |[default to false]
**include_flat_parts** | Option<**bool**> |  |  |
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

[**Vec<models::BtPartMetadataInfo>**](BTPartMetadataInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

