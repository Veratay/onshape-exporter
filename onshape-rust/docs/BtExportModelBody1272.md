# BtExportModelBody1272

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bt_type** | Option<**String**> | Type of JSON object. | [optional]
**closed** | Option<**bool**> | If type == COMPOSITE, indicates whether it is open or closed. | [optional]
**constituent_body_ids** | Option<**Vec<String>**> |  | [optional]
**consumed_by_composite** | Option<**bool**> | Indicates if there is a closed composite that consumes this body. | [optional]
**edges** | Option<[**Vec<models::BtExportModelEdge1782>**](BTExportModelEdge-1782.md)> |  | [optional]
**faces** | Option<[**Vec<models::BtExportModelFace1363>**](BTExportModelFace-1363.md)> |  | [optional]
**id** | Option<**String**> |  | [optional]
**properties** | Option<[**models::BtExportBodyProperties3559**](BTExportBodyProperties-3559.md)> |  | [optional]
**r#type** | Option<[**models::GbtBodyType**](GBTBodyType.md)> |  | [optional]
**vertices** | Option<[**Vec<models::BtExportModelVertex858>**](BTExportModelVertex-858.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


