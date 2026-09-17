# CreateBuyerLockonObserveResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lockon_id** | **String** |  | 
**status** | **String** |  | 
**identifier** | Option<[**crate::models::CreateBuyerIdentifyResponseIdentifier**](CreateBuyerIdentifyResponse_identifier.md)> |  | [optional]
**candidates** | [**Vec<crate::models::CreateBuyerLockonObserveResponseCandidates>**](CreateBuyerLockonObserveResponse_candidates.md) | Present when we could not settle it alone. Show them; a pinch on one is the cheapest, strongest disambiguation available. | 
**observation_count** | **f32** |  | 
**vision_calls** | **f32** |  | 
**vision_quota_exhausted** | **bool** |  | 
**verdict** | **String** |  | 
**crossly** | Option<[**crate::models::GetBuyerAnywhereResponseCrossly**](GetBuyerAnywhereResponse_crossly.md)> |  | [optional]
**offsite** | Option<[**crate::models::GetBuyerAnywhereResponseOffsite**](GetBuyerAnywhereResponse_offsite.md)> |  | [optional]
**alternates** | [**Vec<crate::models::GetBuyerAnywhereResponseAlternates>**](GetBuyerAnywhereResponse_alternates.md) |  | 
**saving_cents** | Option<**f32**> |  | [optional]
**shipping_unknown** | **bool** |  | 
**hud** | [**crate::models::CreateBuyerIdentifyResponseHud**](CreateBuyerIdentifyResponse_hud.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


