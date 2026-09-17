# GetBuyerPreferenceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**retailers** | [**Vec<crate::models::GetBuyerPreferenceResponseRetailers>**](GetBuyerPreferenceResponse_retailers.md) | Retailers they shop, most-seen first. | 
**brands** | [**Vec<crate::models::GetBuyerPreferenceResponseBrands>**](GetBuyerPreferenceResponse_brands.md) | What they look at, by catalog brand where we could resolve one. | 
**price_band** | Option<[**crate::models::GetBuyerPreferenceResponsePriceBand**](GetBuyerPreferenceResponse_priceBand.md)> |  | [optional]
**match_rate** | Option<**f32**> | How much of what they want we can actually supply. | [optional]
**observations** | **f32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


