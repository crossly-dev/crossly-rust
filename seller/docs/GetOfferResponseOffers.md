# GetOfferResponseOffers

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**status** | **String** |  | 
**amount_cents** | **f32** |  | 
**listed_total_cents** | Option<**f32**> | Asking total when the offer was made — what the buyer responded to. | [optional]
**is_bundle** | **bool** |  | 
**items** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**message** | Option<**String**> |  | [optional]
**parent_offer_id** | Option<**String**> |  | [optional]
**expires_at** | **String** |  | 
**decided_at** | Option<**String**> |  | [optional]
**consumed_at** | Option<**String**> | Set once an accepted offer has actually been paid for. | [optional]
**created_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


