# BtbExportAdvancedParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assembly_export_params** | Option<[**models::BtbAssemblyExportParams**](BTBAssemblyExportParams.md)> |  | [optional]
**configuration** | Option<**String**> | URL-encoded string of configuration values (separated by `;`). See the [Configurations API Guide](https://onshape-public.github.io/docs/api-adv/configs/) for details. | [optional]
**element_ids** | Option<**Vec<String>**> | An array of element ids for multi-element export. | [optional]
**evaluate_export_rule** | Option<**bool**> | Set to `true` to evaluate the export rule for the given `formatName` and to include an `exportRuleFileName` value in the response. | [optional][default to false]
**ignore_export_rules_for_contents** | Option<**bool**> | For multiple elements export, use 'true' if export rule shouldn't be applied for all elements. | [optional][default to false]
**link_document_id** | Option<**String**> | The id of the document through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. | [optional]
**link_document_workspace_id** | Option<**String**> | The id of the workspace through which the above document should be accessed; only applicable when accessing a version of the document. This allows a user who has access to document a to see data from document b, as long as document b has been linked to document a by a user who has permission to both. | [optional]
**part_ids** | Option<**String**> | IDs of the parts to retrieve. Use comma-separated IDs for multiple parts (example: partIds=JHK,JHD). | [optional]
**parts_export_filter** | Option<[**models::BtPartsExportFilter4308**](BTPartsExportFilter-4308.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


