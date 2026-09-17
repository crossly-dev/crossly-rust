# CreateBuyerScanResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**verdict** | **String** |  | 
**crossly** | Option<[**crate::models::GetBuyerAnywhereResponseCrossly**](GetBuyerAnywhereResponse_crossly.md)> |  | [optional]
**offsite** | Option<[**crate::models::GetBuyerAnywhereResponseOffsite**](GetBuyerAnywhereResponse_offsite.md)> |  | [optional]
**alternates** | [**Vec<crate::models::GetBuyerAnywhereResponseAlternates>**](GetBuyerAnywhereResponse_alternates.md) |  | 
**saving_cents** | Option<**f32**> |  | [optional]
**shipping_unknown** | **bool** |  | 
**match_method** | **String** |  | 
**comparable** | **bool** |  | 
**confidence** | **f32** |  | 
**identifier** | [**crate::models::CreateBuyerScanResponseIdentifier**](CreateBuyerScanResponse_identifier.md) |  | 
**visual_matches** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


