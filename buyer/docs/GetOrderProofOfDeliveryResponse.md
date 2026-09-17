# GetOrderProofOfDeliveryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | **String** |  | 
**platform** | **String** |  | 
**platform_order_id** | Option<**String**> |  | [optional]
**item_title** | Option<**String**> |  | [optional]
**buyer_username** | Option<**String**> |  | [optional]
**ship_to_postal_code** | Option<**String**> | The ZIP we shipped to, for comparison against the delivery scan. | [optional]
**ship_to_city_state** | Option<**String**> |  | [optional]
**carrier** | Option<**String**> |  | [optional]
**tracking_number** | Option<**String**> |  | [optional]
**tracking_url** | Option<**String**> |  | [optional]
**shipped_at** | Option<**String**> |  | [optional]
**delivered_at** | Option<**String**> |  | [optional]
**delivery_location** | Option<**String**> |  | [optional]
**signature** | Option<**String**> | Null means the carrier captured none — NOT that delivery is unproven. | [optional]
**scans** | [**Vec<crate::models::GetOrderProofOfDeliveryResponseScans>**](GetOrderProofOfDeliveryResponse_scans.md) |  | 
**gaps** | **Vec<String>** | Why this document is weak, stated plainly so the seller isn't surprised  by the marketplace's response. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


