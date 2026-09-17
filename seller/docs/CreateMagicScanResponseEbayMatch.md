# CreateMagicScanResponseEbayMatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_id** | **String** |  | 
**legacy_item_id** | Option<**String**> |  | [optional]
**title** | **String** |  | 
**brand** | Option<**String**> |  | [optional]
**price_cents** | Option<**f32**> | Normalized cents. eBay returns string + currency on `price.value`. | [optional]
**currency** | Option<**String**> |  | [optional]
**condition** | Option<**String**> |  | [optional]
**category_id** | Option<**String**> | Top-level category eBay assigned to the match (id + path). | [optional]
**category_path** | Option<**String**> |  | [optional]
**item_url** | **String** |  | 
**thumbnail_url** | Option<**String**> |  | [optional]
**aspects** | Option<[**serde_json::Value**](.md)> | Loosely-typed aspect bag — Brand, Color, Material, etc. when eBay inlines them. Always inspected defensively by the synthesizer. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


