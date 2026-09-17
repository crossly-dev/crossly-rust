# CreateBuyerActivityResponseMatchOffer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | **String** |  | 
**price_cents** | **f32** |  | 
**condition** | Option<**String**> | Free-text on listings, an order-book grade ('DS') on asks. Never null on asks; frequently null on listings, which is itself informative. | [optional]
**title** | Option<**String**> |  | [optional]
**image_url** | Option<**String**> |  | [optional]
**url** | **String** |  | 
**slug** | Option<**String**> | The public slug, when this offer is a marketplace listing.  Present so Scout can buy it without parsing the URL it was given back. Null on an order-book ask, which is deliberate rather than an omission: an ask is a price in a book, not a thing with a checkout, and a Buy button on one would be a promise the market side cannot keep. | [optional]
**available** | Option<**f32**> | How many the seller has. Caps the quantity stepper honestly. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


