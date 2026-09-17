# GetPlatformLimitResponseEbay

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**platform** | **String** |  | 
**used** | **f32** |  | 
**limit** | **f32** |  | 
**remaining** | **f32** |  | 
**tier_configured** | **bool** | False if the user hasn't picked a tier (we default to 250 but flag it so the UI can prompt). | 
**respect_quota** | **bool** |  | 
**period_start** | **String** |  | 
**per_overage_fee_usd** | **f32** | Approximate cost if `used` overflows `limit` — informational. | 
**selling_cap** | Option<[**crate::models::GetPlatformLimitResponseEbaySellingCap**](GetPlatformLimitResponse_ebay_sellingCap.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


