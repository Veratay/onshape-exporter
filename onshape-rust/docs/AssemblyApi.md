# \AssemblyApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_feature**](AssemblyApi.md#add_feature) | **POST** /assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/features | Add a feature to the assembly feature list.
[**create_assembly**](AssemblyApi.md#create_assembly) | **POST** /assemblies/d/{did}/w/{wid} | Create a new assembly tab in the document.
[**create_instance**](AssemblyApi.md#create_instance) | **POST** /assemblies/d/{did}/w/{wid}/e/{eid}/instances | Insert an instance of a part, sketch, assembly, or Part Studio into an assembly.
[**delete_feature**](AssemblyApi.md#delete_feature) | **DELETE** /assemblies/d/{did}/w/{wid}/e/{eid}/features/featureid/{fid} | Delete a feature from an assembly.
[**delete_instance**](AssemblyApi.md#delete_instance) | **DELETE** /assemblies/d/{did}/w/{wid}/e/{eid}/instance/nodeid/{nid} | Delete an instance of an assembly.
[**get_assembly_bounding_boxes**](AssemblyApi.md#get_assembly_bounding_boxes) | **GET** /assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/boundingboxes | Get bounding box information for the specified assembly.
[**get_assembly_definition**](AssemblyApi.md#get_assembly_definition) | **GET** /assemblies/d/{did}/{wvm}/{wvmid}/e/{eid} | Get definition information for the specified assembly.
[**get_assembly_mass_properties**](AssemblyApi.md#get_assembly_mass_properties) | **GET** /assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/massproperties | Get the mass properties for the assembly.
[**get_assembly_shaded_views**](AssemblyApi.md#get_assembly_shaded_views) | **GET** /assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/shadedviews | Get an array of shaded view images for the document.
[**get_bill_of_materials**](AssemblyApi.md#get_bill_of_materials) | **GET** /assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/bom | Get the Bill Of Materials (BOM) content for the specified assembly.
[**get_display_states**](AssemblyApi.md#get_display_states) | **GET** /assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/displaystates | Get a list of display states for the specified assembly.
[**get_exploded_views**](AssemblyApi.md#get_exploded_views) | **GET** /assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/explodedviews | Get a list of exploded views for the specified assembly.
[**get_feature_specs**](AssemblyApi.md#get_feature_specs) | **GET** /assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/featurespecs | Get the feature spec definitions for an assembly.
[**get_features**](AssemblyApi.md#get_features) | **GET** /assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/features | Get the definitions of the specified features in an assembly.
[**get_mate_values**](AssemblyApi.md#get_mate_values) | **GET** /assemblies/d/{did}/{wv}/{wvid}/e/{eid}/matevalues | Get a list of mate values in the specified assembly.
[**get_named_positions**](AssemblyApi.md#get_named_positions) | **GET** /assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/namedpositions | Get a list of all named positions for the assembly.
[**get_named_views**](AssemblyApi.md#get_named_views) | **GET** /assemblies/d/{did}/e/{eid}/namedViews | Get the view data for all named views for the specified element.
[**get_or_create_bill_of_materials_element**](AssemblyApi.md#get_or_create_bill_of_materials_element) | **POST** /assemblies/d/{did}/w/{wid}/e/{eid}/bomelement | Gets the Bill Of Materials (BOM) for the specified assembly, or creates a BOM if none exist.
[**insert_transformed_instances**](AssemblyApi.md#insert_transformed_instances) | **POST** /assemblies/d/{did}/w/{wid}/e/{eid}/transformedinstances | Create new instances with transformation.
[**modify**](AssemblyApi.md#modify) | **POST** /assemblies/d/{did}/w/{wid}/e/{eid}/modify | Modify an assembly.
[**transform_occurrences**](AssemblyApi.md#transform_occurrences) | **POST** /assemblies/d/{did}/w/{wid}/e/{eid}/occurrencetransforms | Transform a list of assembly occurrences.
[**translate_format**](AssemblyApi.md#translate_format) | **POST** /assemblies/d/{did}/{wv}/{wvid}/e/{eid}/translations | Export the assembly to another format.
[**update_feature**](AssemblyApi.md#update_feature) | **POST** /assemblies/d/{did}/w/{wid}/e/{eid}/features/featureid/{fid} | Update an existing feature for an Assembly.
[**update_mate_values**](AssemblyApi.md#update_mate_values) | **POST** /assemblies/d/{did}/w/{wid}/e/{eid}/matevalues | Update mate values for the given mates in the specified assembly.



## add_feature

> models::BtFeatureDefinitionResponse1617 add_feature(did, wvm, wvmid, eid, bt_feature_definition_call1406)
Add a feature to the assembly feature list.

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


## create_assembly

> models::BtDocumentElementInfo create_assembly(did, wid, bt_model_element_params)
Create a new assembly tab in the document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_model_element_params** | [**BtModelElementParams**](BtModelElementParams.md) |  | [required] |

### Return type

[**models::BtDocumentElementInfo**](BTDocumentElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_instance

> serde_json::Value create_instance(did, wid, eid, bt_assembly_instance_definition_params)
Insert an instance of a part, sketch, assembly, or Part Studio into an assembly.

Part Studio instances may include multiple parts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_assembly_instance_definition_params** | Option<[**BtAssemblyInstanceDefinitionParams**](BtAssemblyInstanceDefinitionParams.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_feature

> models::BtFeatureApiBase1430 delete_feature(did, wid, eid, fid)
Delete a feature from an assembly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**fid** | **String** |  | [required] |

### Return type

[**models::BtFeatureApiBase1430**](BTFeatureApiBase-1430.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance

> serde_json::Value delete_instance(did, eid, wid, nid)
Delete an instance of an assembly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**nid** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_assembly_bounding_boxes

> models::BtBoundingBoxInfo get_assembly_bounding_boxes(did, wvm, wvmid, eid, link_document_id, configuration, exploded_view_id, include_hidden, display_state_id, named_position_id, include_sketches)
Get bounding box information for the specified assembly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**exploded_view_id** | Option<**String**> |  |  |
**include_hidden** | Option<**bool**> |  |  |
**display_state_id** | Option<**String**> | Call the [getDisplayStates](https://cad.onshape.com/glassworks/explorer/#/Assembly/getDisplayStates) endpoint to get display state ID(s). |  |
**named_position_id** | Option<**String**> |  |  |
**include_sketches** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtBoundingBoxInfo**](BTBoundingBoxInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_assembly_definition

> models::BtAssemblyDefinitionInfo get_assembly_definition(did, wvm, wvmid, eid, link_document_id, configuration, exploded_view_id, include_mate_features, include_non_solids, include_mate_connectors, exclude_suppressed)
Get definition information for the specified assembly.

All coordinates and translation matrix components are in meters (m).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**exploded_view_id** | Option<**String**> |  |  |
**include_mate_features** | Option<**bool**> |  |  |[default to false]
**include_non_solids** | Option<**bool**> |  |  |[default to false]
**include_mate_connectors** | Option<**bool**> |  |  |[default to false]
**exclude_suppressed** | Option<**bool**> | Whether or not to exclude suppressed instances/mate features in response |  |[default to false]

### Return type

[**models::BtAssemblyDefinitionInfo**](BTAssemblyDefinitionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_assembly_mass_properties

> models::BtMassPropertiesInfo get_assembly_mass_properties(did, wvm, wvmid, eid, link_document_id, configuration)
Get the mass properties for the assembly.

The assembly must contain parts that have density. The returned schema includes the same information as in the Onshape [Mass Properties Tool](https://cad.onshape.com/help/Content/massprops-asmb.htm).  When three values are returned:   * The first is the calculated value.   * The second is the minimum possible value, considering tolerance.   * The third is the maximum possible value, considering tolerance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]

### Return type

[**models::BtMassPropertiesInfo**](BTMassPropertiesInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_assembly_shaded_views

> models::BtShadedViewsInfo get_assembly_shaded_views(did, wvm, wvmid, eid, link_document_id, configuration, exploded_view_id, view_matrix, output_height, output_width, pixel_size, edges, show_all_parts, include_surfaces, use_anti_aliasing, include_wires, display_state_id, named_position_id)
Get an array of shaded view images for the document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**exploded_view_id** | Option<**String**> |  |  |
**view_matrix** | Option<**String**> |  |  |[default to front]
**output_height** | Option<**i32**> |  |  |[default to 500]
**output_width** | Option<**i32**> |  |  |[default to 500]
**pixel_size** | Option<**f64**> |  |  |[default to 0.003]
**edges** | Option<**String**> |  |  |[default to show]
**show_all_parts** | Option<**bool**> |  |  |[default to false]
**include_surfaces** | Option<**bool**> |  |  |[default to true]
**use_anti_aliasing** | Option<**bool**> |  |  |[default to false]
**include_wires** | Option<**bool**> |  |  |[default to false]
**display_state_id** | Option<**String**> | Call the [getDisplayStates](https://cad.onshape.com/glassworks/explorer/#/Assembly/getDisplayStates) endpoint to get display state ID(s). |  |
**named_position_id** | Option<**String**> |  |  |

### Return type

[**models::BtShadedViewsInfo**](BTShadedViewsInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bill_of_materials

> models::BtBillOfMaterialsInfo get_bill_of_materials(did, wvm, wvmid, eid, link_document_id, configuration, bom_column_ids, indented, multi_level, generate_if_absent, template_id, include_excluded, only_visible_columns, ignore_subassembly_bom_behavior, include_item_microversions, include_top_level_assembly_row, thumbnail)
Get the Bill Of Materials (BOM) content for the specified assembly.

Returns the BOM in JSON in the Onshape BOM Standard format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**bom_column_ids** | Option<[**Vec<String>**](String.md)> | Ids of the columns to include, or all columns if empty. BOM column ids correspond to metadata property ids. |  |
**indented** | Option<**bool**> | Return the Structured BOM table with all rows collapsed, otherwise returns the Flattened BOM. |  |[default to true]
**multi_level** | Option<**bool**> | Return the Structured BOM table with all rows expanded. Ignored if indented is false. |  |[default to false]
**generate_if_absent** | Option<**bool**> | Return the BOM table data even if the BOM does not exist. If this is false and the BOM does not exist, a 404 status code will be returned. This option is highly recommended. |  |[default to false]
**template_id** | Option<**String**> | The id of the BOM table template to use when generating the table. |  |
**include_excluded** | Option<**bool**> | Include items that have been excluded from the BOM table. |  |
**only_visible_columns** | Option<**bool**> | Only return data for visible columns, instead of all possible columns. |  |
**ignore_subassembly_bom_behavior** | Option<**bool**> | Ignore the 'Subassembly BOM behavior' property when constructing the BOM table. |  |
**include_item_microversions** | Option<**bool**> | Include element microversions and version metadata microversions in the JSON. |  |[default to false]
**include_top_level_assembly_row** | Option<**bool**> | Include top-level assembly row when constructing the BOM table. |  |[default to false]
**thumbnail** | Option<**bool**> | Return thumbnail info |  |[default to false]

### Return type

[**models::BtBillOfMaterialsInfo**](BTBillOfMaterialsInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_display_states

> Vec<models::BtDisplayStateInfo> get_display_states(did, wvm, wvmid, eid, link_document_id)
Get a list of display states for the specified assembly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**Vec<models::BtDisplayStateInfo>**](BTDisplayStateInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exploded_views

> Vec<models::BtViewFeatureBaseInfo> get_exploded_views(did, wvm, wvmid, eid, link_document_id, configuration, exploded_view_id)
Get a list of exploded views for the specified assembly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**exploded_view_id** | Option<**String**> |  |  |

### Return type

[**Vec<models::BtViewFeatureBaseInfo>**](BTViewFeatureBaseInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feature_specs

> models::BtFeatureSpecsResponse664 get_feature_specs(did, wvm, wvmid, eid)
Get the feature spec definitions for an assembly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

[**models::BtFeatureSpecsResponse664**](BTFeatureSpecsResponse-664.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_features

> models::BtAssemblyFeatureListResponse1174 get_features(did, wvm, wvmid, eid, link_document_id, configuration, exploded_view_id, feature_id)
Get the definitions of the specified features in an assembly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**exploded_view_id** | Option<**String**> |  |  |
**feature_id** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::BtAssemblyFeatureListResponse1174**](BTAssemblyFeatureListResponse-1174.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mate_values

> models::BtAssemblyMateValuesInfo get_mate_values(did, wv, wvid, eid)
Get a list of mate values in the specified assembly.

Describes the relative position of the first mate connector with respect to the second along the designated degrees of freedom (DOF) for mates in the specified assembly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

[**models::BtAssemblyMateValuesInfo**](BTAssemblyMateValuesInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_named_positions

> Vec<models::BtViewFeatureBaseInfo> get_named_positions(did, wvm, wvmid, eid, link_document_id, configuration, exploded_view_id)
Get a list of all named positions for the assembly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |[default to ]
**exploded_view_id** | Option<**String**> |  |  |

### Return type

[**Vec<models::BtViewFeatureBaseInfo>**](BTViewFeatureBaseInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_named_views

> models::BtNamedViewsInfo get_named_views(did, eid, link_document_id, skip_perspective, include_section_cut_views)
Get the view data for all named views for the specified element.

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


## get_or_create_bill_of_materials_element

> models::BtDocumentElementInfo get_or_create_bill_of_materials_element(did, wid, eid)
Gets the Bill Of Materials (BOM) for the specified assembly, or creates a BOM if none exist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

[**models::BtDocumentElementInfo**](BTDocumentElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insert_transformed_instances

> models::BtAssemblyInsertTransformedInstancesResponse insert_transformed_instances(did, eid, wid, bt_assembly_transformed_instances_definition_params)
Create new instances with transformation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_assembly_transformed_instances_definition_params** | [**BtAssemblyTransformedInstancesDefinitionParams**](BtAssemblyTransformedInstancesDefinitionParams.md) |  | [required] |

### Return type

[**models::BtAssemblyInsertTransformedInstancesResponse**](BTAssemblyInsertTransformedInstancesResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify

> serde_json::Value modify(did, wid, eid, link_document_id, bt_assembly_modification_params)
Modify an assembly.

This endpoint can include multiple modifications to an assembly with one change. For example, it can delete/suppress/unsuppress/transform multiple instances. It creates one history entry in the document history list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**bt_assembly_modification_params** | Option<[**BtAssemblyModificationParams**](BtAssemblyModificationParams.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transform_occurrences

> serde_json::Value transform_occurrences(did, eid, wid, bt_assembly_transform_definition_params)
Transform a list of assembly occurrences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_assembly_transform_definition_params** | Option<[**BtAssemblyTransformDefinitionParams**](BtAssemblyTransformDefinitionParams.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## translate_format

> models::BtTranslationRequestInfo translate_format(did, wv, wvid, eid, bt_translate_format_params)
Export the assembly to another format.

* Use `formatName` in the JSON request body to specify the export file type. Use [Translations/getAllTranslatorFormats](https://cad.onshape.com/glassworks/explorer/#/Translation/getAllTranslatorFormats) to get a list of valid export file formats. Confirm that `couldBeAssembly=true.` * Set `storeInDocument` to `false` to export to a data file. Set to `true` to export to a blob element in the same document.  * See [API Guide: Model Translation](https://onshape-public.github.io/docs/api-adv/translation/) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_translate_format_params** | [**BtTranslateFormatParams**](BtTranslateFormatParams.md) |  | [required] |

### Return type

[**models::BtTranslationRequestInfo**](BTTranslationRequestInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feature

> models::BtFeatureDefinitionResponse1617 update_feature(did, wid, eid, fid, bt_feature_definition_call1406)
Update an existing feature for an Assembly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**fid** | **String** |  | [required] |
**bt_feature_definition_call1406** | Option<[**BtFeatureDefinitionCall1406**](BtFeatureDefinitionCall1406.md)> |  |  |

### Return type

[**models::BtFeatureDefinitionResponse1617**](BTFeatureDefinitionResponse-1617.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_mate_values

> models::BtAssemblyMateValuesInfo update_mate_values(did, wid, eid, bt_assembly_mate_values_info)
Update mate values for the given mates in the specified assembly.

* The input mates must support motion along the provided input degrees of freedom; otherwise, the input mate value will be ignored.  * Values associated with multiple allowed degrees of freedom for a mate can be updated simultaneously.  * Values associated with multiple mate features can be updated simultaneously.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_assembly_mate_values_info** | [**BtAssemblyMateValuesInfo**](BtAssemblyMateValuesInfo.md) |  | [required] |

### Return type

[**models::BtAssemblyMateValuesInfo**](BTAssemblyMateValuesInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

