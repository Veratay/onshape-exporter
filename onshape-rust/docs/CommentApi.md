# \CommentApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_attachment**](CommentApi.md#add_attachment) | **POST** /comments/{cid}/attachment | Add an attachment to a comment.
[**create_comment**](CommentApi.md#create_comment) | **POST** /comments | Update a document with a new comment.
[**delete_attachments**](CommentApi.md#delete_attachments) | **DELETE** /comments/{cid}/attachment | Delete all attachments from a comment.
[**delete_comment**](CommentApi.md#delete_comment) | **DELETE** /comments/{cid} | Delete a comment from a document.
[**get_attachment**](CommentApi.md#get_attachment) | **GET** /comments/{cid}/attachment/{fdid}.{ext} | Get the attachment with the specified file extension that is associated with the specified comment.
[**get_comment**](CommentApi.md#get_comment) | **GET** /comments/{cid} | Get details for a comment.
[**get_comments**](CommentApi.md#get_comments) | **GET** /comments | Get a list of comments in a document.
[**reopen**](CommentApi.md#reopen) | **POST** /comments/{cid}/reopen | Reopen a resolved comment.
[**resolve**](CommentApi.md#resolve) | **POST** /comments/{cid}/resolve | Resolve a comment.
[**update_comment**](CommentApi.md#update_comment) | **POST** /comments/{cid} | Update the content of an existing comment.



## add_attachment

> models::BtCommentInfo add_attachment(cid, file, is_markup)
Add an attachment to a comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |
**file** | [**serde_json::Value**](serde_json::Value.md) | The file to upload. | [required] |
**is_markup** | **bool** |  | [required] |

### Return type

[**models::BtCommentInfo**](BTCommentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_comment

> models::BtCommentInfo create_comment(bt_comment_params)
Update a document with a new comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_comment_params** | [**BtCommentParams**](BtCommentParams.md) |  | [required] |

### Return type

[**models::BtCommentInfo**](BTCommentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_attachments

> serde_json::Value delete_attachments(cid)
Delete all attachments from a comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment

> serde_json::Value delete_comment(cid)
Delete a comment from a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attachment

> serde_json::Value get_attachment(cid, fdid, ext)
Get the attachment with the specified file extension that is associated with the specified comment.

Returns only a single attachment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |
**fdid** | **String** |  | [required] |
**ext** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, image/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment

> models::BtCommentInfo get_comment(cid)
Get details for a comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |

### Return type

[**models::BtCommentInfo**](BTCommentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments

> models::BtListResponseBtCommentInfo get_comments(did, object_type, pid, eid, filter, resolved, sort_column, sort_order, offset, limit)
Get a list of comments in a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | Option<**String**> |  |  |[default to ]
**object_type** | Option<**i32**> |  |  |[default to 6]
**pid** | Option<**String**> |  |  |[default to ]
**eid** | Option<**String**> |  |  |[default to ]
**filter** | Option<**i32**> |  |  |[default to 0]
**resolved** | Option<**bool**> |  |  |[default to true]
**sort_column** | Option<**String**> |  |  |[default to createdAt]
**sort_order** | Option<**String**> |  |  |[default to asc]
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::BtListResponseBtCommentInfo**](BTListResponseBTCommentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reopen

> models::BtCommentInfo reopen(cid)
Reopen a resolved comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |

### Return type

[**models::BtCommentInfo**](BTCommentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resolve

> models::BtCommentInfo resolve(cid)
Resolve a comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |

### Return type

[**models::BtCommentInfo**](BTCommentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_comment

> models::BtCommentInfo update_comment(cid, bt_comment_params)
Update the content of an existing comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |
**bt_comment_params** | [**BtCommentParams**](BtCommentParams.md) |  | [required] |

### Return type

[**models::BtCommentInfo**](BTCommentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

