# BtMassPropertiesInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**centroid** | Option<**Vec<f64>**> | Centroid, center of gravity, center of mass | [optional]
**has_mass** | Option<**bool**> | `true` if the part has mass. | [optional]
**inertia** | Option<**Vec<f64>**> | Mass moments of inertia | [optional]
**mass** | Option<**Vec<f64>**> | Mass | [optional]
**mass_missing_count** | Option<**i32**> | Number of parts without mass. | [optional]
**periphery** | Option<**Vec<f64>**> | Surface area | [optional]
**principal_axes** | Option<[**Vec<models::BtVector3d389>**](BTVector3d-389.md)> | Vector coordinates of the principal axes. Use `BTVector3d-389` as the `btType`. | [optional]
**principal_inertia** | Option<**Vec<f64>**> | Principal moments of inertia | [optional]
**volume** | Option<**Vec<f64>**> | Volume | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


