# UpdateOrderResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**user_id** | **String** |  | 
**quantity** | **f32** |  | 
**notes** | Option<**String**> |  | [optional]
**status** | **String** |  | 
**platform** | **String** |  | 
**cancelled_at** | Option<**String**> |  | [optional]
**listing_id** | Option<**String**> |  | [optional]
**inventory_item_id** | Option<**String**> |  | [optional]
**platform_listing_id** | Option<**String**> |  | [optional]
**handling_time_days** | Option<**f32**> |  | [optional]
**platform_order_id** | Option<**String**> |  | [optional]
**buyer_username** | Option<**String**> |  | [optional]
**sales_channel** | **String** |  | 
**channel_location_id** | Option<**String**> |  | [optional]
**package_preset_id** | Option<**String**> |  | [optional]
**carrier** | Option<**String**> |  | [optional]
**service** | Option<**String**> |  | [optional]
**tracking_number** | Option<**String**> |  | [optional]
**easypost_shipment_id** | Option<**String**> |  | [optional]
**easypost_tracker_id** | Option<**String**> |  | [optional]
**easypost_rate_id** | Option<**String**> |  | [optional]
**shipping_label_url** | Option<**String**> |  | [optional]
**label_purchased_at** | Option<**String**> |  | [optional]
**shipped_at** | Option<**String**> |  | [optional]
**estimated_delivery** | Option<**String**> |  | [optional]
**delivered_at** | Option<**String**> |  | [optional]
**carrier_status** | Option<**String**> |  | [optional]
**carrier_status_detail** | Option<**String**> |  | [optional]
**tracking_history** | Option<[**Vec<crate::models::ListOrdersItemTrackingHistory>**](ListOrdersItem_trackingHistory.md)> |  | [optional]
**delivery_location** | Option<**String**> |  | [optional]
**delivery_signature** | Option<**String**> |  | [optional]
**tracking_submitted_at** | Option<**String**> |  | [optional]
**tracking_submit_status** | Option<**String**> |  | [optional]
**label_cost** | Option<**String**> |  | [optional]
**cost_of_goods** | Option<**String**> |  | [optional]
**requested_carrier** | Option<**String**> |  | [optional]
**requested_service** | Option<**String**> |  | [optional]
**ship_by_at** | Option<**String**> |  | [optional]
**ship_by_alerted_at** | Option<**String**> |  | [optional]
**oversold_by** | **f32** |  | 
**last_status_check_at** | Option<**String**> |  | [optional]
**last_chat_check_at** | Option<**String**> |  | [optional]
**is_disputed** | **bool** |  | 
**dispute_reason** | Option<**String**> |  | [optional]
**dispute_platform_case_id** | Option<**String**> |  | [optional]
**dispute_resolved_at** | Option<**String**> |  | [optional]
**refund_amount** | Option<**String**> |  | [optional]
**refund_reason** | Option<**String**> |  | [optional]
**refund_platform_id** | Option<**String**> |  | [optional]
**refunded_at** | Option<**String**> |  | [optional]
**cancellation_reason** | Option<**String**> |  | [optional]
**arrival_condition_requested_at** | Option<**String**> |  | [optional]
**arrival_condition_submitted_at** | Option<**String**> |  | [optional]
**arrival_condition_declined_at** | Option<**String**> |  | [optional]
**delivery_photo_url** | Option<**String**> |  | [optional]
**purchase_order_ref** | Option<**String**> |  | [optional]
**arrival_condition_photos** | **Vec<String>** |  | 
**buyer_email** | Option<**String**> |  | [optional]
**fulfillment_method** | **String** |  | 
**deleted_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


