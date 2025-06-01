# \DrawingApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_drawing_app_element**](DrawingApi.md#create_drawing_app_element) | **POST** /drawings/d/{did}/w/{wid}/create | Create a new drawing in a document.
[**create_drawing_translation**](DrawingApi.md#create_drawing_translation) | **POST** /drawings/d/{did}/{wv}/{wvid}/e/{eid}/translations | Translate (export) a drawing to a different format.
[**get_drawing_translator_formats**](DrawingApi.md#get_drawing_translator_formats) | **GET** /drawings/d/{did}/w/{wid}/e/{eid}/translationformats | Get a list of all valid formats the drawing can be translated (exported) to.
[**get_drawing_view_json_geometry1**](DrawingApi.md#get_drawing_view_json_geometry1) | **GET** /drawings/d/{did}/{wvm}/{wvmid}/e/{eid}/views/{viewid}/jsongeometry | Get view geometry of a drawing view in JSON format.
[**get_drawing_views1**](DrawingApi.md#get_drawing_views1) | **GET** /drawings/d/{did}/{wvm}/{wvmid}/e/{eid}/views | Get details of all drawing views.
[**get_modification_status**](DrawingApi.md#get_modification_status) | **GET** /drawings/modify/status/{mrid} | Get the status of a drawing modification operation.
[**modify_drawing**](DrawingApi.md#modify_drawing) | **POST** /drawings/d/{did}/w/{wid}/e/{eid}/modify | Modify a drawing via JSON payload.



## create_drawing_app_element

> models::BtDocumentElementInfo create_drawing_app_element(did, wid, bt_drawing_params)
Create a new drawing in a document.

This endpoint takes a JSON Schema as input. See the schema docs below for details, and see [API Guide: Drawings](https://onshape-public.github.io/docs/api-adv/drawings/) for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | ID of the document in which to create the drawing. | [required] |
**wid** | **String** | ID of the workspace in which to create the drawing. | [required] |
**bt_drawing_params** | [**BtDrawingParams**](BtDrawingParams.md) |  | [required] |

### Return type

[**models::BtDocumentElementInfo**](BTDocumentElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_drawing_translation

> models::BtTranslationRequestInfo create_drawing_translation(did, wv, wvid, eid, bt_translate_format_params)
Translate (export) a drawing to a different format.

Export a drawing to a different format within a document. Use `getDrawingTranslatorFormats` for a list of supported translation (i.e., import/export) formats. See [API Guide: Translations](https://onshape-public.github.io/docs/api-adv/translation/#export-a-drawing-as-a-json) for more information.

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


## get_drawing_translator_formats

> Vec<models::BtModelFormatInfo> get_drawing_translator_formats(did, wid, eid)
Get a list of all valid formats the drawing can be translated (exported) to.

See [API Guide: Translations](https://onshape-public.github.io/docs/api-adv/translation/#export-a-drawing-as-a-json) for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

[**Vec<models::BtModelFormatInfo>**](BTModelFormatInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_drawing_view_json_geometry1

> serde_json::Value get_drawing_view_json_geometry1(did, wvm, wvmid, eid, viewid, link_document_id, transaction_id, change_id, scale)
Get view geometry of a drawing view in JSON format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**viewid** | **String** | The id of the view in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**transaction_id** | Option<**String**> | The id of the transaction in which this operation should take place. Transaction ids can be generated through the AppElement startTransaction API. |  |[default to ]
**change_id** | Option<**String**> | The id of the last change made to this application element. This can be retrieved from the response for any app element modification endpoint. |  |[default to ]
**scale** | Option<**f64**> | Scale for measurements. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_drawing_views1

> models::BtAppArrayInfoBtAppDrawingViewInfo get_drawing_views1(did, wvm, wvmid, eid, link_document_id, transaction_id, change_id)
Get details of all drawing views.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wvm** | **String** | Indicates which of workspace (w), version (v), or document microversion (m) id is specified below. | [required] |
**wvmid** | **String** | The id of the workspace, version or document microversion in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**transaction_id** | Option<**String**> | The id of the transaction in which this operation should take place. Transaction ids can be generated through the AppElement startTransaction API. |  |[default to ]
**change_id** | Option<**String**> | The id of the last change made to this application element. This can be retrieved from the response for any app element modification endpoint. |  |[default to ]

### Return type

[**models::BtAppArrayInfoBtAppDrawingViewInfo**](BTAppArrayInfoBTAppDrawingViewInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_modification_status

> models::BtAppModificationRequestInfo get_modification_status(mrid)
Get the status of a drawing modification operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mrid** | **String** |  | [required] |

### Return type

[**models::BtAppModificationRequestInfo**](BTAppModificationRequestInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_drawing

> models::BtAppModificationRequestInfo modify_drawing(did, wid, eid, bt_drawing_modification_params, link_document_id)
Modify a drawing via JSON payload.

See [API Guide: Drawings](https://onshape-public.github.io/docs/api-adv/drawings/) for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**bt_drawing_modification_params** | [**BtDrawingModificationParams**](BtDrawingModificationParams.md) |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtAppModificationRequestInfo**](BTAppModificationRequestInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

