# CreateBuyerActivityResponseMatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**verdict** | **String** |  | 
**offer** | [**crate::models::CreateBuyerActivityResponseMatchOffer**](CreateBuyerActivityResponse_match_offer.md) |  | 
**saving_cents** | Option<**f32**> | Positive when cheaper. Null when no page price was supplied. | [optional]
**condition_comparable** | **bool** | TRUE means the offer's condition and the page's are comparable. False means we matched the item but not its state — a used Crossly copy against a retailer's new one — and the UI must say so rather than claim a saving. | 
**shipping_known** | **bool** | Always false today. Crossly shipping is computed at checkout from the buyer's address, which Scout does not have and should not send. Present so the surface that renders \"before shipping\" is reading a fact rather than hard-coding an assumption that stops being true when we add it. | 
**catalog** | Option<[**crate::models::GetCatalogLookupResponseCatalog**](GetCatalogLookupResponse_catalog.md)> |  | [optional]
**alternates** | [**Vec<crate::models::CreateBuyerActivityResponseMatchOffer>**](CreateBuyerActivityResponse_match_offer.md) | Other buyable offers for the SAME item, cheapest first, best excluded.  Deliberately not \"you might also like\". We have no behavioural data to build that from, and inventing it would put unrelated items under a badge whose entire value is that it only ever appears when we have the thing the buyer is actually looking at.  What these ARE is the same product from other sellers, in other conditions, at other prices — which is the choice a buyer on a product page genuinely wants, and the one a single \"cheapest\" result hides. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


