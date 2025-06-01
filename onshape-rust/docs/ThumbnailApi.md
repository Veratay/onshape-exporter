# \ThumbnailApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_application_thumbnails**](ThumbnailApi.md#delete_application_thumbnails) | **DELETE** /thumbnails/d/{did}/{wv}/{wvid}/e/{eid} | Delete an element's thumbnail.
[**get_document_thumbnail**](ThumbnailApi.md#get_document_thumbnail) | **GET** /thumbnails/d/{did}/w/{wid} | Get the thumbnail info for a workspace.
[**get_document_thumbnail_with_size**](ThumbnailApi.md#get_document_thumbnail_with_size) | **GET** /thumbnails/d/{did}/w/{wid}/s/{sz} | Get the thumbnail image with the given size for a document.
[**get_element_thumbnail**](ThumbnailApi.md#get_element_thumbnail) | **GET** /thumbnails/d/{did}/{wv}/{wvid}/e/{eid} | Get the thumbnail info structure for an element.
[**get_element_thumbnail_with_api_configuration**](ThumbnailApi.md#get_element_thumbnail_with_api_configuration) | **GET** /thumbnails/d/{did}/w/{wid}/e/{eid}/ac/{cid}/s/{sz} | Get the thumbnail image with the given configuration for an element.
[**get_element_thumbnail_with_size**](ThumbnailApi.md#get_element_thumbnail_with_size) | **GET** /thumbnails/d/{did}/{wv}/{wvid}/e/{eid}/s/{sz} | Get the thumbnail image with the given size for an element.
[**get_thumbnail_for_document**](ThumbnailApi.md#get_thumbnail_for_document) | **GET** /thumbnails/d/{did} | Get the thumbnail info for a document in the default workspace.
[**get_thumbnail_for_document_and_version**](ThumbnailApi.md#get_thumbnail_for_document_and_version) | **GET** /thumbnails/d/{did}/v/{vid} | Get the thumbnail info for a version of a document.
[**get_thumbnail_for_document_and_version_old**](ThumbnailApi.md#get_thumbnail_for_document_and_version_old) | **GET** /thumbnails/document/{did}/version/{vid} | This endpoint will be deprecated soon. Use `getThumbnailForDocumentAndVersion` instead.
[**get_thumbnail_for_document_old**](ThumbnailApi.md#get_thumbnail_for_document_old) | **GET** /thumbnails/document/{did} | This endpoint will be deprecated soon. Use `getThumbnailForDocument` instead.
[**set_application_element_thumbnail**](ThumbnailApi.md#set_application_element_thumbnail) | **POST** /thumbnails/d/{did}/{wv}/{wvid}/e/{eid} | Set the thumbnail image for an application element.



## delete_application_thumbnails

> serde_json::Value delete_application_thumbnails(did, wv, wvid, eid, link_document_id)
Delete an element's thumbnail.

Deletes an application element's thumbnail and images for the given document, workspace or version, and element combination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wv** | **String** | Indicates which of workspace (w) or version (v) id is specified below. | [required] |
**wvid** | **String** | The id of the workspace, version in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document_thumbnail

> models::BtThumbnailInfo get_document_thumbnail(did, wid)
Get the thumbnail info for a workspace.

* By default, returns thumbnail info for the element with the most-recently generated image. If you pinned an element for the document thumbnail, that element will always be used for the document-level thumbnail, if it exists in the workspace. * See also: [Tech tip on how to change a document thumbnail in onshape](https://www.onshape.com/en/resource-center/tech-tips/tech-tip-how-to-change-a-document-thumbnail-in-onshape)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |

### Return type

[**models::BtThumbnailInfo**](BTThumbnailInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document_thumbnail_with_size

> serde_json::Value get_document_thumbnail_with_size(did, wid, sz, t, skip_default_image)
Get the thumbnail image with the given size for a document.

* By default, returns thumbnail image for the element with the most-recently generated image. If you pinned an element for the document thumbnail, that element will always be used for the document-level thumbnail, if it exists in the workspace. * See also: [Tech tip on how to change a document thumbnail in onshape](https://www.onshape.com/en/resource-center/tech-tips/tech-tip-how-to-change-a-document-thumbnail-in-onshape)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**sz** | **String** | the generated thumbnail size in pixels, widthxheigth | [required] |
**t** | Option<**String**> | Cache Control key. If specified, the response header returned will tell the client to use cached thumbnails. |  |
**skip_default_image** | Option<**String**> | Controls the return of the default image, if thumbnail is not available |  |[default to ]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, image/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_element_thumbnail

> models::BtThumbnailInfo get_element_thumbnail(did, wv, wvid, eid, link_document_id)
Get the thumbnail info structure for an element.

Returns thumbnail info for the given document, workspace or version, and element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wv** | **String** | Indicates which of workspace (w) or version (v) id is specified below. | [required] |
**wvid** | **String** | The id of the workspace, version in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]

### Return type

[**models::BtThumbnailInfo**](BTThumbnailInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_element_thumbnail_with_api_configuration

> serde_json::Value get_element_thumbnail_with_api_configuration(did, wid, eid, cid, sz, link_document_id, t, skip_default_image, reject_empty, require_config_match)
Get the thumbnail image with the given configuration for an element.

Returns the thumbnail image for an element at a specified version, with the given configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wid** | **String** | The id of the workspace in which to perform the operation. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**cid** | **String** |  | [required] |
**sz** | **String** | the generated thumbnail size in pixels, widthxheigth | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**t** | Option<**String**> | Cache Control key. If specified, the response header returned will tell the client to use cached thumbnails. |  |
**skip_default_image** | Option<**String**> | Controls the return of the default image, if thumbnail is not available |  |[default to ]
**reject_empty** | Option<**bool**> |  |  |[default to false]
**require_config_match** | Option<**bool**> |  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, image/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_element_thumbnail_with_size

> serde_json::Value get_element_thumbnail_with_size(did, wv, wvid, eid, sz, link_document_id, t, skip_default_image, reject_empty)
Get the thumbnail image with the given size for an element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wv** | **String** | Indicates which of workspace (w) or version (v) id is specified below. | [required] |
**wvid** | **String** | The id of the workspace, version in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**sz** | **String** | the generated thumbnail size in pixels, widthxheigth | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**t** | Option<**String**> | Cache Control key. If specified, the response header returned will tell the client to use cached thumbnails. |  |
**skip_default_image** | Option<**String**> | Controls the return of the default image, if thumbnail is not available |  |[default to ]
**reject_empty** | Option<**bool**> |  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, image/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thumbnail_for_document

> models::BtThumbnailInfo get_thumbnail_for_document(did)
Get the thumbnail info for a document in the default workspace.

* By default, returns thumbnail info for the element with the most-recently generated image. If you pinned an element for the document thumbnail, that element will always be used for the document-level thumbnail, if it exists in the workspace. * The default workspace may vary by user; the image served depends on the signed-in user. * See also: [Tech tip on how to change a document thumbnail in onshape](https://www.onshape.com/en/resource-center/tech-tips/tech-tip-how-to-change-a-document-thumbnail-in-onshape)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |

### Return type

[**models::BtThumbnailInfo**](BTThumbnailInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thumbnail_for_document_and_version

> models::BtThumbnailInfo get_thumbnail_for_document_and_version(did, vid, link_document_id)
Get the thumbnail info for a version of a document.

* By default, returns thumbnail info for the element with the most-recently generated image. If you pinned an element for the document thumbnail, that element will always be used for the document-level thumbnail, if it exists in the workspace. * See also: [Tech tip on how to change a document thumbnail in onshape](https://www.onshape.com/en/resource-center/tech-tips/tech-tip-how-to-change-a-document-thumbnail-in-onshape)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**vid** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |

### Return type

[**models::BtThumbnailInfo**](BTThumbnailInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thumbnail_for_document_and_version_old

> models::BtThumbnailInfo get_thumbnail_for_document_and_version_old(did, vid)
This endpoint will be deprecated soon. Use `getThumbnailForDocumentAndVersion` instead.

This API exists for historical reasons. It uses `/document/` in the path, rather than the standard `/d/` to specify the document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**vid** | **String** |  | [required] |

### Return type

[**models::BtThumbnailInfo**](BTThumbnailInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thumbnail_for_document_old

> models::BtThumbnailInfo get_thumbnail_for_document_old(did)
This endpoint will be deprecated soon. Use `getThumbnailForDocument` instead.

This API exists for historical reasons. It uses `/document/` in the path, rather than the standard `/d/` to specify the document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |

### Return type

[**models::BtThumbnailInfo**](BTThumbnailInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_application_element_thumbnail

> serde_json::Value set_application_element_thumbnail(did, wv, wvid, eid, bt_application_element_thumbnail_params_array, link_document_id, overwrite)
Set the thumbnail image for an application element.

* Allows 3rd-party applications to set thumbnails for their elements.  * Application elements can have both primary and secondary thumbnails. A primary thumbnail represents the top-level of the element. A secondary thumbnail can represent sub-components of the element (e.g., a drawing sheet). * To update one or more thumbnails, you must set the overwrite query param to `true` and supply the entire set of thumbnails. All previous thumbnails will be deleted prior to updating the element with the latest images.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**wv** | **String** | Indicates which of workspace (w) or version (v) id is specified below. | [required] |
**wvid** | **String** | The id of the workspace, version in which the operation should be performed. | [required] |
**eid** | **String** | The id of the element in which to perform the operation. | [required] |
**bt_application_element_thumbnail_params_array** | [**BtApplicationElementThumbnailParamsArray**](BtApplicationElementThumbnailParamsArray.md) |  | [required] |
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. |  |[default to ]
**overwrite** | Option<**bool**> |  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

