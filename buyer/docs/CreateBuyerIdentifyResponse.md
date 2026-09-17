# CreateBuyerIdentifyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tier** | **String** |  | 
**confidence** | **f32** |  | 
**identifier** | Option<[**crate::models::CreateBuyerIdentifyResponseIdentifier**](CreateBuyerIdentifyResponse_identifier.md)> |  | [optional]
**vision_label** | Option<**String**> |  | [optional]
**vision_quota_exhausted** | **bool** | Surfaced rather than hidden: \"we could not look harder\" and \"we looked and found nothing\" are different answers, and a client that cannot tell them apart shows the wrong message on both. | 
**visual_matches** | [**Vec<crate::models::CreateBuyerIdentifyResponseVisualMatches>**](CreateBuyerIdentifyResponse_visualMatches.md) |  | 
**verdict** | **String** |  | 
**crossly** | Option<[**crate::models::GetBuyerAnywhereResponseCrossly**](GetBuyerAnywhereResponse_crossly.md)> |  | [optional]
**offsite** | Option<[**crate::models::GetBuyerAnywhereResponseOffsite**](GetBuyerAnywhereResponse_offsite.md)> |  | [optional]
**alternates** | [**Vec<crate::models::GetBuyerAnywhereResponseAlternates>**](GetBuyerAnywhereResponse_alternates.md) |  | 
**saving_cents** | Option<**f32**> |  | [optional]
**shipping_unknown** | **bool** |  | 
**hud** | [**crate::models::CreateBuyerIdentifyResponseHud**](CreateBuyerIdentifyResponse_hud.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


