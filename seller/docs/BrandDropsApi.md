# \BrandDropsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_brand_drop**](BrandDropsApi.md#create_brand_drop) | **POST** /v1/brand-drops | Create a brand drop as a draft (name, dropAt in the future, optional description, cover image and Crossly platform-listing ids). Approved partners only (403 drop_partner_required); counts against your monthly allowance (403 drop_quota_exhausted).
[**get_brand_drop**](BrandDropsApi.md#get_brand_drop) | **GET** /v1/brand-drops/{id} | One of your brand drops, with its status and any note from Crossly’s review.
[**get_brand_drop_partner**](BrandDropsApi.md#get_brand_drop_partner) | **GET** /v1/brand-drops/partner | Your drop-partner terms (fee, promotion tier, monthly allowance, whether you may schedule without review) and how many drops you have created this month. partner is null when you are not an approved partner — apply at crossly.net/drops/apply.
[**list_brand_drop_listings**](BrandDropsApi.md#list_brand_drop_listings) | **GET** /v1/brand-drops/{id}/listings | The listings a brand drop releases when it goes live.
[**list_brand_drops**](BrandDropsApi.md#list_brand_drops) | **GET** /v1/brand-drops | Your brand drops, newest start time first. Filter with ?status=draft|pending_review|scheduled|live|sold_out|ended|cancelled. Empty when you are not a partner.
[**update_brand_drop**](BrandDropsApi.md#update_brand_drop) | **PATCH** /v1/brand-drops/{id} | Edit a brand drop and/or change its status: draft (withdraw / unschedule), pending_review (submit for review), scheduled (only with canSelfPublish — 403 drop_review_required otherwise), cancelled. Editing name, time or description of a reviewed drop sends it back to review. Live and finished drops cannot be changed.
[**update_brand_drop_listing**](BrandDropsApi.md#update_brand_drop_listing) | **PUT** /v1/brand-drops/{id}/listings | Replace the listings a brand drop releases, by master listing id (up to 100). Each must already be listed on Crossly — otherwise 400 listing_not_on_crossly naming them. Not allowed once the drop is live or finished.



## create_brand_drop

> crate::models::BrandDropResponse create_brand_drop()
Create a brand drop as a draft (name, dropAt in the future, optional description, cover image and Crossly platform-listing ids). Approved partners only (403 drop_partner_required); counts against your monthly allowance (403 drop_quota_exhausted).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BrandDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_brand_drop

> crate::models::BrandDropResponse get_brand_drop(id)
One of your brand drops, with its status and any note from Crossly’s review.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BrandDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_brand_drop_partner

> crate::models::GetBrandDropPartnerResponse get_brand_drop_partner()
Your drop-partner terms (fee, promotion tier, monthly allowance, whether you may schedule without review) and how many drops you have created this month. partner is null when you are not an approved partner — apply at crossly.net/drops/apply.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBrandDropPartnerResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_brand_drop_listings

> crate::models::ListBrandDropListingsResponse list_brand_drop_listings(id)
The listings a brand drop releases when it goes live.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListBrandDropListingsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_brand_drops

> crate::models::ListBrandDropsResponse list_brand_drops()
Your brand drops, newest start time first. Filter with ?status=draft|pending_review|scheduled|live|sold_out|ended|cancelled. Empty when you are not a partner.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBrandDropsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_brand_drop

> crate::models::BrandDropResponse update_brand_drop(id)
Edit a brand drop and/or change its status: draft (withdraw / unschedule), pending_review (submit for review), scheduled (only with canSelfPublish — 403 drop_review_required otherwise), cancelled. Editing name, time or description of a reviewed drop sends it back to review. Live and finished drops cannot be changed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BrandDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_brand_drop_listing

> crate::models::ListBrandDropListingsResponse update_brand_drop_listing(id)
Replace the listings a brand drop releases, by master listing id (up to 100). Each must already be listed on Crossly — otherwise 400 listing_not_on_crossly naming them. Not allowed once the drop is live or finished.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListBrandDropListingsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

