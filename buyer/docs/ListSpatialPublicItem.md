# ListSpatialPublicItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**public_slug** | **String** |  | 
**category_slug** | **String** |  | 
**item_count** | **f32** | Unsold stock that lands in this room. The same predicate the room uses. | 
**for_sale_count** | **f32** | How many of those a visitor could buy right now. | 
**price_from_cents** | Option<**f32**> | The cheapest and dearest thing for sale, in cents.  A BAND, deliberately, and never a quote: `listPublicSceneOffers` is the only authority on what a given object costs. Null when nothing is for sale — zero would read as free. | [optional]
**price_to_cents** | Option<**f32**> |  | [optional]
**preview_images** | **Vec<String>** | Up to PREVIEW_IMAGES item images. Catalog art first, seller photo else. | 
**updated_at** | Option<**String**> | Last time the room itself changed. ISO, or null if the row has no date. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


