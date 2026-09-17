# ListBuyerCatalogSearchItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**slug** | **String** |  | 
**title** | **String** |  | 
**price_cents** | **f32** |  | 
**compare_at_cents** | Option<**f32**> | MSRP above the ask, or null. Never fabricated from a stale value. | [optional]
**currency** | **String** |  | 
**condition** | Option<**String**> |  | [optional]
**brand** | Option<**String**> |  | [optional]
**category_main** | Option<**String**> |  | [optional]
**category_sub** | Option<**String**> |  | [optional]
**thumbnail** | Option<**String**> |  | [optional]
**images** | **Vec<String>** |  | 
**seller_username** | Option<**String**> |  | [optional]
**seller_display_name** | Option<**String**> |  | [optional]
**quantity_available** | **f32** | Units a buyer can actually take right now. Reserved units are excluded. | 
**listed_at** | Option<**String**> |  | [optional]
**url** | **String** | Canonical buyer-facing URL, so a client never has to build one. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


