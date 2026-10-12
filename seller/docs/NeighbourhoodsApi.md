# \NeighbourhoodsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_neighbourhood**](NeighbourhoodsApi.md#get_neighbourhood) | **GET** /v1/neighbourhoods/{slug} | One neighbourhood street by slug.
[**list_neighbourhoods**](NeighbourhoodsApi.md#list_neighbourhoods) | **GET** /v1/neighbourhoods | Every neighbourhood street (Sneaker Row, Card District, Vintage Alley, Vinyl Lane): open or \"opening soon\" (6 shops + open stalls, or a live street event), shops, stalls today, the next street event and its /c/<slug> URL.



## get_neighbourhood

> crate::models::GetNeighbourhoodResponse get_neighbourhood(slug)
One neighbourhood street by slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetNeighbourhoodResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_neighbourhoods

> crate::models::ListNeighbourhoodsResponse list_neighbourhoods()
Every neighbourhood street (Sneaker Row, Card District, Vintage Alley, Vinyl Lane): open or \"opening soon\" (6 shops + open stalls, or a live street event), shops, stalls today, the next street event and its /c/<slug> URL.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListNeighbourhoodsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

