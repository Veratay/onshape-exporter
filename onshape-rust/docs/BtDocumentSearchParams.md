# BtDocumentSearchParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**document_filter** | Option<**i32**> | Type of documents to search: `0: My Documents | 1: Created | 2: Shared | 3: Trash | 4: Public | 5: Recent | 6: By Owner | 7: By Company | 9: By Team` | [optional]
**found_in** | Option<[**models::BtesVersionWorkspaceChoice**](BTESVersionWorkspaceChoice.md)> |  | [optional]
**limit** | Option<**i32**> | Number of results to return per page. Default value is 20 (also the maximum). | [optional][default to 20]
**offset** | Option<**i32**> | Offset. Determines where search results begin. Default value is 0. | [optional][default to 0]
**owner_id** | Option<**String**> | Owner ID. Can be a user ID, company ID, or team ID, depending on `ownerType`. | [optional]
**parent_id** | Option<**String**> | Search document parent Id  | [optional]
**raw_query** | Option<**String**> | Search for documents that contain the given string in the name. Search is not case-sensitive. | [optional]
**sort_column** | Option<**String**> | Column by which to sort search results. `name | modifiedAt | createdAt (default) | email | modifiedBy | promotedAt` | [optional][default to createdAt]
**sort_order** | Option<**String**> | Type of documents to search: `0: My Documents | 1: Created | 2: Shared | 3: Trash | 4: Public | 5: Recent | 6: By Owner | 7: By Company | 9: By Team` | [optional][default to desc]
**r#type** | Option<**String**> | Type of owner. `0: User | 1: Company | 2: Onshape`. If the owner is a teamId, leave this unspecified. | [optional]
**when** | Option<[**models::BtesResultsFilter**](BTESResultsFilter.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


