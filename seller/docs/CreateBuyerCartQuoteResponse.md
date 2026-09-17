# CreateBuyerCartQuoteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | **String** |  | 
**tax_cents** | **f32** |  | 
**shipping_cents** | **f32** |  | 
**total_cents** | **f32** |  | 
**currency** | **String** |  | 
**items_total_cents** | **f32** |  | 
**pickup_cart_item_ids** | **Vec<String>** | Lines being collected in person, so a summary can name what ships free. | 
**tax_complete** | **bool** | False means there is no saved delivery address, so `taxCents` is a floor rather than a final figure — not that tax is zero. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


