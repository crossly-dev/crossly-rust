# ListSpatialPublicOffersItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_id** | **String** |  | 
**listing_slug** | **String** | platform_listings.platform_listing_id — what POST /cart/items takes. | 
**price_cents** | **f32** |  | 
**currency** | **String** |  | 
**available** | Option<**f32**> | Remaining stock, when the listing declares one. Null = unknown. | [optional]
**condition** | Option<**String**> | What a buyer is entitled to know before they add it. | [optional]
**grade_key** | Option<**String**> | Canonical grade key when the item is slabbed, e.g. `psa-10`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


