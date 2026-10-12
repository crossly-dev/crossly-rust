# \DropsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_drop**](DropsApi.md#create_drop) | **POST** /v1/drops | Set up a drop: { title, listingIds (1-10 of your live Crossly listings), units (at most their stock), opensLineAt, releaseAt (ISO), eligibility: \"signed_in\" | \"verified\" }. The line opens at opensLineAt with a published seed commitment; the draw runs at releaseAt.
[**delete_drop**](DropsApi.md#delete_drop) | **DELETE** /v1/drops/{id} | Call a drop off before its line opens.
[**get_drop**](DropsApi.md#get_drop) | **GET** /v1/drops/{id} | A drop: its listings, units, times, eligibility, the sha256 seed commitment and the entry count. After release also the revealed seed, the sorted entry ids and the draw order (recompute it with drawWinners(seed, dropId, entryIds, units)).
[**list_drop_mine**](DropsApi.md#list_drop_mine) | **GET** /v1/drops/mine | Your drops, latest release first, with entries, winners, claims, the waitlist and expired holds.
[**list_street_drops**](DropsApi.md#list_street_drops) | **GET** /v1/streets/{slug}/drops | Upcoming, open and just-released drops on a Crossly World street, one per shop, soonest first, with the plot (or live stall spot) each line forms at.
[**update_drop**](DropsApi.md#update_drop) | **PATCH** /v1/drops/{id} | Change a drop before its line opens (any of the set-up fields).



## create_drop

> crate::models::CreateDropResponse create_drop()
Set up a drop: { title, listingIds (1-10 of your live Crossly listings), units (at most their stock), opensLineAt, releaseAt (ISO), eligibility: \"signed_in\" | \"verified\" }. The line opens at opensLineAt with a published seed commitment; the draw runs at releaseAt.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_drop

> crate::models::DeleteDropResponse delete_drop(id)
Call a drop off before its line opens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_drop

> crate::models::GetDropResponse get_drop(id)
A drop: its listings, units, times, eligibility, the sha256 seed commitment and the entry count. After release also the revealed seed, the sorted entry ids and the draw order (recompute it with drawWinners(seed, dropId, entryIds, units)).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_drop_mine

> crate::models::ListDropMineResponse list_drop_mine()
Your drops, latest release first, with entries, winners, claims, the waitlist and expired holds.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListDropMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_street_drops

> crate::models::ListStreetDropsResponse list_street_drops(slug)
Upcoming, open and just-released drops on a Crossly World street, one per shop, soonest first, with the plot (or live stall spot) each line forms at.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListStreetDropsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_drop

> crate::models::CreateDropResponse update_drop(id)
Change a drop before its line opens (any of the set-up fields).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

