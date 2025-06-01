# \SketchApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sketch_bounding_boxes**](SketchApi.md#get_sketch_bounding_boxes) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/sketches/{sid}/boundingboxes | Get all bounding boxes for a sketch.
[**get_sketch_info**](SketchApi.md#get_sketch_info) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/sketches | Get information for all sketches in Part Studio.
[**get_tessellated_entities**](SketchApi.md#get_tessellated_entities) | **GET** /partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/sketches/{sid}/tessellatedentities | Get the tessellations of a sketch in a Part Studio.



## get_sketch_bounding_boxes

> models::BtBoundingBoxInfo get_sketch_bounding_boxes(did, wvm, wvmid, eid, sid, configuration, link_document_id)
Get all bounding boxes for a sketch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**sid** | **String** |  | [required] |
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


## get_sketch_info

> serde_json::Value get_sketch_info(did, wvm, wvmid, eid, configuration, sketch_id, output3_d, curve_points, include_geometry, link_document_id)
Get information for all sketches in Part Studio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. |  |
**sketch_id** | Option<[**Vec<String>**](String.md)> |  |  |
**output3_d** | Option<**bool**> |  |  |[default to false]
**curve_points** | Option<**bool**> |  |  |[default to false]
**include_geometry** | Option<**bool**> |  |  |[default to true]
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tessellated_entities

> serde_json::Value get_tessellated_entities(did, wvm, wvmid, eid, sid, configuration, entity_id, angle_tolerance, chord_tolerance, link_document_id)
Get the tessellations of a sketch in a Part Studio.

The accuracy of the tessellation to exact geometry is controlled by the `angleTolerance` and `chordTolerance` parameters. The tessellation points are computed closely enough so that neither the angle tolerance nor the chord tolerance are exceeded. For most parts, the angular tolerance is the most restrictive of the two default tolerances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**sid** | **String** |  | [required] |
**configuration** | Option<**String**> |  |  |
**entity_id** | Option<[**Vec<String>**](String.md)> |  |  |
**angle_tolerance** | Option<**f64**> |  |  |
**chord_tolerance** | Option<**f64**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

