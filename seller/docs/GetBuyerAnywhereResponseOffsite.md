# GetBuyerAnywhereResponseOffsite

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**store_id** | **String** |  | 
**host** | **String** | The retailer's hostname, e.g. `rei.com`. | 
**store_name** | **String** |  | 
**title** | **String** |  | 
**price_cents** | **f32** |  | 
**shipping_cents** | Option<**f32**> | Null = UNKNOWN, never free. | [optional]
**currency** | **String** |  | 
**condition** | Option<**String**> |  | [optional]
**url** | **String** |  | 
**image_url** | Option<**String**> |  | [optional]
**buyer_cashback_cents** | **f32** | What the buyer gets back, in cents, if they buy through us.  Shown because a cashback figure the buyer cannot see is a figure they have no reason to believe. Derived from the store's rate, never stored per offer — rates change and a copied one goes stale silently. | 
**delivered_cents** | **f32** | Item + shipping when known; item alone otherwise. See `shippingUnknown`. | 
**shipping_unknown** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


