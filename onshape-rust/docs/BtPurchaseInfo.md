# BtPurchaseInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> |  | [optional]
**actual_amount_paid_cents** | Option<**i64**> |  | [optional]
**amount_cents** | Option<**i64**> |  | [optional]
**application** | Option<[**models::BtapiApplicationSummaryInfo**](BTAPIApplicationSummaryInfo.md)> |  | [optional]
**canceled_at** | Option<**String**> |  | [optional]
**card** | Option<[**models::BtCardInfo**](BTCardInfo.md)> |  | [optional]
**client_id** | Option<**String**> |  | [optional]
**coupon_amount_off** | Option<**i64**> |  | [optional]
**coupon_percent_off** | Option<**i32**> |  | [optional]
**created_by** | Option<**String**> |  | [optional]
**currency** | Option<**String**> |  | [optional]
**duration** | Option<**i32**> |  | [optional]
**duration_months** | Option<**i32**> |  | [optional]
**group** | Option<**String**> |  | [optional]
**href** | Option<**String**> | URI to fetch complete information of the resource. | [optional]
**id** | Option<**String**> | Id of the resource. | [optional]
**last_modified** | Option<**String**> |  | [optional]
**last_modified_by** | Option<**String**> |  | [optional]
**light_seats** | Option<**i64**> |  | [optional]
**name** | Option<**String**> | Name of the resource. | [optional]
**next_charge** | Option<[**models::NextCharge**](NextCharge.md)> |  | [optional]
**payment_type** | Option<**i32**> |  | [optional]
**pending_cancelation** | Option<**bool**> |  | [optional]
**plan** | Option<[**models::BtBillingPlanInfo**](BTBillingPlanInfo.md)> |  | [optional]
**plan_id** | Option<**String**> |  | [optional]
**plan_name** | Option<**String**> |  | [optional]
**plan_type** | Option<**i32**> |  | [optional]
**pre_trial_plan_id** | Option<**String**> |  | [optional]
**prorated_charges** | Option<[**Vec<models::ProratedCharges>**](ProratedCharges.md)> |  | [optional]
**prorated_total** | Option<**i64**> |  | [optional]
**purchase_date** | Option<**String**> |  | [optional]
**reseller_name** | Option<**String**> |  | [optional]
**seats** | Option<**i64**> |  | [optional]
**state** | Option<**i32**> |  | [optional]
**subscribers** | Option<[**Vec<models::BtPlanSubscriberInfo>**](BTPlanSubscriberInfo.md)> |  | [optional]
**subscription_begin_at** | Option<**String**> |  | [optional]
**subscription_end_at** | Option<**String**> |  | [optional]
**subscription_id** | Option<**String**> |  | [optional]
**subscription_type** | Option<**i32**> |  | [optional]
**tax_amount_cents** | Option<**i64**> |  | [optional]
**trial_end** | Option<**String**> |  | [optional]
**trial_initiated_by** | Option<**String**> |  | [optional]
**view_ref** | Option<**String**> | URI to visualize the resource in a webclient if applicable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


