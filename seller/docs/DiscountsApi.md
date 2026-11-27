# \DiscountsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_automatic_discount**](DiscountsApi.md#create_automatic_discount) | **POST** /v1/automatic-discounts | Create an automatic discount.
[**create_automatic_discount_preview**](DiscountsApi.md#create_automatic_discount_preview) | **POST** /v1/automatic-discounts/preview | Preview a hypothetical cart of your listings against your automatic discounts (or an unsaved draft): what each line gets.
[**create_discount_code**](DiscountsApi.md#create_discount_code) | **POST** /v1/discount-codes | Create a discount code: percent, fixed, free shipping, buy X get Y, spend tiers or a set price, with requirements, item scope, limits (total, per buyer, per day, budget, regions), schedule and stacking.
[**create_discount_code_generate**](DiscountsApi.md#create_discount_code_generate) | **POST** /v1/discount-codes/{id}/generate | Generate up to 5,000 unique single-use codes with this code’s rules (their uses count toward its limits).
[**create_discount_code_status**](DiscountsApi.md#create_discount_code_status) | **POST** /v1/discount-codes/{id}/status | Pause, resume (active) or archive a discount code.
[**create_discount_holder**](DiscountsApi.md#create_discount_holder) | **POST** /v1/discounts/holders | Create a Collection holders deal in the store (X-Storefront-Id; default store when omitted).
[**delete_automatic_discount**](DiscountsApi.md#delete_automatic_discount) | **DELETE** /v1/automatic-discounts/{id} | Delete an automatic discount.
[**delete_discount_holder**](DiscountsApi.md#delete_discount_holder) | **DELETE** /v1/discounts/holders/{id} | Delete a Collection holders deal. Returns `{ ok }`.
[**get_discount_code**](DiscountsApi.md#get_discount_code) | **GET** /v1/discount-codes/{id} | One discount code with its rules and usage.
[**get_discount_code_share**](DiscountsApi.md#get_discount_code_share) | **GET** /v1/discount-codes/{id}/share | The share link that pre-applies the code at checkout (the store page with ?code=) and a QR code of it as a PNG data URL.
[**get_discount_holder**](DiscountsApi.md#get_discount_holder) | **GET** /v1/discounts/holders/{id} | One Collection holders deal with its usage (uses, cents given).
[**list_automatic_discounts**](DiscountsApi.md#list_automatic_discounts) | **GET** /v1/automatic-discounts | Your automatic discounts (buy more, save more): quantity and mixed-item tiers, scope, channels and schedule.
[**list_discount_code_redemptions**](DiscountsApi.md#list_discount_code_redemptions) | **GET** /v1/discount-codes/{id}/redemptions | A code’s usage history: orders, masked buyers, region, channel and the amount given, newest first.
[**list_discount_codes**](DiscountsApi.md#list_discount_codes) | **GET** /v1/discount-codes | Your discount codes: kind, status (active, scheduled, paused, expired, exhausted, archived), every rule, and usage.
[**list_discount_holders**](DiscountsApi.md#list_discount_holders) | **GET** /v1/discounts/holders | The store’s Collection holders deals: a percent off for buyers holding any token of a collection (by chain + contract), with scope, channels, schedule, stacking and limits.
[**update_automatic_discount**](DiscountsApi.md#update_automatic_discount) | **PUT** /v1/automatic-discounts/{id} | Replace an automatic discount.
[**update_discount_code**](DiscountsApi.md#update_discount_code) | **PATCH** /v1/discount-codes/{id} | Update a discount code. The code itself can be renamed only until it is first used.
[**update_discount_holder**](DiscountsApi.md#update_discount_holder) | **PUT** /v1/discounts/holders/{id} | Replace a Collection holders deal.



## create_automatic_discount

> crate::models::CreateAutomaticDiscountResponse create_automatic_discount()
Create an automatic discount.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateAutomaticDiscountResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_automatic_discount_preview

> crate::models::CreateAutomaticDiscountPreviewResponse create_automatic_discount_preview()
Preview a hypothetical cart of your listings against your automatic discounts (or an unsaved draft): what each line gets.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateAutomaticDiscountPreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_discount_code

> crate::models::DiscountCodeResponse create_discount_code()
Create a discount code: percent, fixed, free shipping, buy X get Y, spend tiers or a set price, with requirements, item scope, limits (total, per buyer, per day, budget, regions), schedule and stacking.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::DiscountCodeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_discount_code_generate

> crate::models::CreateDiscountCodeGenerateResponse create_discount_code_generate(id)
Generate up to 5,000 unique single-use codes with this code’s rules (their uses count toward its limits).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateDiscountCodeGenerateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_discount_code_status

> crate::models::DiscountCodeResponse create_discount_code_status(id)
Pause, resume (active) or archive a discount code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DiscountCodeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_discount_holder

> crate::models::CreateDiscountHolderResponse create_discount_holder()
Create a Collection holders deal in the store (X-Storefront-Id; default store when omitted).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateDiscountHolderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_automatic_discount

> crate::models::UpdateGroupBuyResponse delete_automatic_discount(id)
Delete an automatic discount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateGroupBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_discount_holder

> crate::models::OkResponse delete_discount_holder(id)
Delete a Collection holders deal. Returns `{ ok }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_discount_code

> crate::models::GetDiscountCodeResponse get_discount_code(id)
One discount code with its rules and usage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetDiscountCodeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_discount_code_share

> crate::models::GetDiscountCodeShareResponse get_discount_code_share(id)
The share link that pre-applies the code at checkout (the store page with ?code=) and a QR code of it as a PNG data URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetDiscountCodeShareResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_discount_holder

> crate::models::GetDiscountHolderResponse get_discount_holder(id)
One Collection holders deal with its usage (uses, cents given).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetDiscountHolderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_automatic_discounts

> crate::models::ListAutomaticDiscountsResponse list_automatic_discounts()
Your automatic discounts (buy more, save more): quantity and mixed-item tiers, scope, channels and schedule.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListAutomaticDiscountsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_discount_code_redemptions

> crate::models::ListDiscountCodeRedemptionsResponse list_discount_code_redemptions(id)
A code’s usage history: orders, masked buyers, region, channel and the amount given, newest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListDiscountCodeRedemptionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_discount_codes

> crate::models::ListDiscountCodesResponse list_discount_codes()
Your discount codes: kind, status (active, scheduled, paused, expired, exhausted, archived), every rule, and usage.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListDiscountCodesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_discount_holders

> crate::models::ListDiscountHoldersResponse list_discount_holders()
The store’s Collection holders deals: a percent off for buyers holding any token of a collection (by chain + contract), with scope, channels, schedule, stacking and limits.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListDiscountHoldersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_automatic_discount

> crate::models::CreateAutomaticDiscountResponse update_automatic_discount(id)
Replace an automatic discount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateAutomaticDiscountResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_discount_code

> crate::models::DiscountCodeResponse update_discount_code(id)
Update a discount code. The code itself can be renamed only until it is first used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DiscountCodeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_discount_holder

> crate::models::CreateDiscountHolderResponse update_discount_holder(id)
Replace a Collection holders deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateDiscountHolderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

