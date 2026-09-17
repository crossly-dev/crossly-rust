# \BuyerApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_buyer_activity**](BuyerApi.md#create_buyer_activity) | **POST** /v1/buyer/activity | Report an item your user is looking at, and get our answer.
[**create_buyer_cart_item**](BuyerApi.md#create_buyer_cart_item) | **POST** /v1/buyer/cart/items | Add a listing to your cart.
[**create_buyer_cart_quote**](BuyerApi.md#create_buyer_cart_quote) | **POST** /v1/buyer/cart/quote | Price the cart, delivered — item, shipping, tax, total.
[**create_buyer_offer**](BuyerApi.md#create_buyer_offer) | **POST** /v1/buyer/offers | Offer a price on a listing.
[**create_buyer_wishlist**](BuyerApi.md#create_buyer_wishlist) | **POST** /v1/buyer/wishlists | Create a wishlist.
[**create_buyer_wishlist_item**](BuyerApi.md#create_buyer_wishlist_item) | **POST** /v1/buyer/wishlists/{id}/items | Add a listing to a wishlist.
[**delete_buyer_cart_item**](BuyerApi.md#delete_buyer_cart_item) | **DELETE** /v1/buyer/cart/items/{id} | Remove a line from your cart.
[**get_buyer_preference**](BuyerApi.md#get_buyer_preference) | **GET** /v1/buyer/preferences | The shopping profile derived from that activity.
[**get_buyer_profile**](BuyerApi.md#get_buyer_profile) | **GET** /v1/buyer/profile | Your Crossly shopping profile — name, email, saved address, Bucks balance.
[**list_buyer_activity**](BuyerApi.md#list_buyer_activity) | **GET** /v1/buyer/activity | What this buyer has compared lately.
[**list_buyer_cart**](BuyerApi.md#list_buyer_cart) | **GET** /v1/buyer/cart | What is in your Crossly cart.
[**list_buyer_cashback**](BuyerApi.md#list_buyer_cashback) | **GET** /v1/buyer/cashback | Your Scout cashback — pending, confirmed, paid.
[**list_buyer_orders**](BuyerApi.md#list_buyer_orders) | **GET** /v1/buyer/orders | What you have bought on Crossly, newest first.
[**list_buyer_wishlist_items**](BuyerApi.md#list_buyer_wishlist_items) | **GET** /v1/buyer/wishlists/{id}/items | What is on one wishlist.
[**list_buyer_wishlists**](BuyerApi.md#list_buyer_wishlists) | **GET** /v1/buyer/wishlists | Your wishlists.



## create_buyer_activity

> crate::models::CreateBuyerActivityResponse create_buyer_activity(inline_object4)
Report an item your user is looking at, and get our answer.

Records the look and returns whether Crossly has the item and at what price. Send an identifier and a store DOMAIN — a full URL is refused. Nothing about the page itself is stored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object4** | [**InlineObject4**](InlineObject4.md) |  | [required] |

### Return type

[**crate::models::CreateBuyerActivityResponse**](CreateBuyerActivityResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_buyer_cart_item

> crate::models::CreateBuyerCartItemResponse create_buyer_cart_item(inline_object2)
Add a listing to your cart.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object2** | [**InlineObject2**](InlineObject2.md) |  | [required] |

### Return type

[**crate::models::CreateBuyerCartItemResponse**](CreateBuyerCartItemResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_buyer_cart_quote

> crate::models::CreateBuyerCartQuoteResponse create_buyer_cart_quote()
Price the cart, delivered — item, shipping, tax, total.

Runs the real checkout cascade and returns the totals instead of charging. Nothing is purchased. `taxComplete: false` means there is no saved delivery address, so the total is a floor rather than a final figure.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateBuyerCartQuoteResponse**](CreateBuyerCartQuoteResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_buyer_offer

> crate::models::CreateBuyerOfferResponse create_buyer_offer(inline_object3)
Offer a price on a listing.

Sends an offer to the seller. Spends nothing — a seller accepting still leaves you to complete checkout. Offers at or above the asking price are refused; buy it instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object3** | [**InlineObject3**](InlineObject3.md) |  | [required] |

### Return type

[**crate::models::CreateBuyerOfferResponse**](CreateBuyerOfferResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_buyer_wishlist

> crate::models::CreateBuyerWishlistResponse create_buyer_wishlist(inline_object)
Create a wishlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object** | [**InlineObject**](InlineObject.md) |  | [required] |

### Return type

[**crate::models::CreateBuyerWishlistResponse**](CreateBuyerWishlistResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_buyer_wishlist_item

> crate::models::CreateBuyerWishlistItemResponse create_buyer_wishlist_item(id, inline_object1)
Add a listing to a wishlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**inline_object1** | [**InlineObject1**](InlineObject1.md) |  | [required] |

### Return type

[**crate::models::CreateBuyerWishlistItemResponse**](CreateBuyerWishlistItemResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_buyer_cart_item

> delete_buyer_cart_item(id)
Remove a line from your cart.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_buyer_preference

> crate::models::GetBuyerPreferenceResponse get_buyer_preference()
The shopping profile derived from that activity.

Derived, never declared — there is no preferences form anywhere. `matchRate` is the share of this person's searches Crossly could answer; a low number is an inventory problem, not a personalisation one, which is why it is here.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetBuyerPreferenceResponse**](GetBuyerPreferenceResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_buyer_profile

> crate::models::GetBuyerProfileResponse get_buyer_profile()
Your Crossly shopping profile — name, email, saved address, Bucks balance.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetBuyerProfileResponse**](GetBuyerProfileResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_buyer_activity

> crate::models::V1List list_buyer_activity()
What this buyer has compared lately.

Newest first, and only as far back as the retention window — the link between a person and a comparison is dropped after 180 days, so this thins out rather than growing forever.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_buyer_cart

> crate::models::V1List list_buyer_cart()
What is in your Crossly cart.

Line items with the price captured when each was added. This is NOT a quote — shipping, tax and any discounts are computed at checkout against a delivery address, and the sum of these lines is not what you will be charged.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_buyer_cashback

> crate::models::V1List list_buyer_cashback(status, page, limit)
Your Scout cashback — pending, confirmed, paid.

Newest first. `pending` means an order was reported and the retailer's return window has not closed; nothing is paid until it does. `expired` means a click was never reported as converting, which is the ordinary outcome for most clicks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_buyer_orders

> crate::models::V1List list_buyer_orders(page, limit)
What you have bought on Crossly, newest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_buyer_wishlist_items

> crate::models::V1List list_buyer_wishlist_items(id)
What is on one wishlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_buyer_wishlists

> crate::models::V1List list_buyer_wishlists()
Your wishlists.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

