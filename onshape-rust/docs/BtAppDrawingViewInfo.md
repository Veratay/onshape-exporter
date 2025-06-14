# BtAppDrawingViewInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**associativity_change_id** | Option<**String**> |  | [optional]
**bom_reference_id** | Option<**String**> |  | [optional]
**broken_out_b_boxes** | Option<[**std::collections::HashMap<String, models::BtBoundingBox1052>**](BTBoundingBox-1052.md)> |  | [optional]
**broken_out_end_conditions** | Option<[**std::collections::HashMap<String, models::BtBrokenOutEndCondition1107>**](BTBrokenOutEndCondition-1107.md)> |  | [optional]
**broken_out_point_numbers** | Option<**Vec<i32>**> |  | [optional]
**change_id** | Option<**String**> |  | [optional]
**compute_intersection** | Option<**bool**> |  | [optional]
**cut_point** | Option<**Vec<f64>**> |  | [optional]
**depth_section_end_condition** | Option<[**models::BtBrokenOutEndCondition1107**](BTBrokenOutEndCondition-1107.md)> |  | [optional]
**display_state_id** | Option<**String**> |  | [optional]
**error_code** | Option<**i32**> | `0: OK (healthy) | 1: INFO | 2: WARNING | 3: ERROR (dangling or view generation call failed) | 4: UNKNOWN` | [optional]
**error_description** | Option<**String**> | A human-readable value for the error that occurred, if one occurred. | [optional]
**error_value** | Option<[**models::BtAppElementErrorCode**](BTAppElementErrorCode.md)> |  | [optional]
**exploded_view_id** | Option<**String**> |  | [optional]
**has_secondary_view_definition** | Option<**bool**> |  | [optional]
**hidden_lines** | Option<**String**> |  | [optional]
**ignore_faulty_parts** | Option<**bool**> |  | [optional]
**include_hidden_instances** | Option<**bool**> |  | [optional]
**include_surfaces** | Option<**bool**> |  | [optional]
**include_wires** | Option<**bool**> |  | [optional]
**is_aligned_section** | Option<**bool**> |  | [optional]
**is_broken_out_section** | Option<**bool**> |  | [optional]
**is_copied_view** | Option<**bool**> |  | [optional]
**is_crop_view** | Option<**bool**> |  | [optional]
**is_partial_section** | Option<**bool**> |  | [optional]
**is_section_of_aligned_section** | Option<**bool**> |  | [optional]
**is_section_of_section_on_base** | Option<**bool**> |  | [optional]
**is_surface** | Option<**bool**> |  | [optional]
**model_reference_id** | Option<**String**> |  | [optional]
**modification_id** | Option<**String**> |  | [optional]
**named_position_id** | Option<**String**> |  | [optional]
**occurrence_or_query_to_geometry_properties** | Option<[**std::collections::HashMap<String, models::BtAppElementViewGeometryProperties1100>**](BTAppElementViewGeometryProperties-1100.md)> |  | [optional]
**offset_section_points** | Option<**Vec<f64>**> |  | [optional]
**parent_change_id** | Option<**String**> |  | [optional]
**parent_view_id** | Option<**String**> |  | [optional]
**perspective** | Option<**bool**> |  | [optional]
**projection_angle** | Option<**String**> |  | [optional]
**quality_option** | Option<**i32**> |  | [optional]
**render_sketches** | Option<**bool**> |  | [optional]
**section_id** | Option<**String**> |  | [optional]
**section_planes** | Option<**Vec<f64>**> |  | [optional]
**show_auto_centerlines** | Option<**bool**> |  | [optional]
**show_auto_centermarks** | Option<**bool**> |  | [optional]
**show_cut_geom_only** | Option<**bool**> |  | [optional]
**show_tangent_lines** | Option<**bool**> |  | [optional]
**show_threads** | Option<**bool**> |  | [optional]
**show_viewing_plane** | Option<**bool**> |  | [optional]
**simplification_option** | Option<**i32**> |  | [optional]
**simplification_threshold** | Option<**f64**> |  | [optional]
**use_parent_view_section_data** | Option<**bool**> |  | [optional]
**view_direction** | Option<**Vec<f64>**> |  | [optional]
**view_id** | Option<**String**> |  | [optional]
**view_matrix** | Option<**Vec<f64>**> |  | [optional]
**view_version** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


