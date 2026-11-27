# \ProductSetsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_product_set**](ProductSetsApi.md#create_product_set) | **POST** /v1/product-sets | Create a product set (filters, explicit picks, or both).
[**create_product_set_bulk**](ProductSetsApi.md#create_product_set_bulk) | **POST** /v1/product-sets/{id}/bulk | Run a bulk action (delist / relist / check-status) against the set's current members.
[**create_product_set_member**](ProductSetsApi.md#create_product_set_member) | **POST** /v1/product-sets/{id}/members | Add / exclude / remove explicit picks for a set.
[**delete_product_set**](ProductSetsApi.md#delete_product_set) | **DELETE** /v1/product-sets/{id} | Delete a product set (its id is also dropped from any channel exclusion lists).
[**get_product_set**](ProductSetsApi.md#get_product_set) | **GET** /v1/product-sets/{id} | Get one product set (its filters and pick counts).
[**list_product_set_members**](ProductSetsApi.md#list_product_set_members) | **GET** /v1/product-sets/{id}/members | Evaluate a set now and return a page of its member listings.
[**list_product_sets**](ProductSetsApi.md#list_product_sets) | **GET** /v1/product-sets | List your saved product sets with their live member counts.
[**update_product_set**](ProductSetsApi.md#update_product_set) | **PATCH** /v1/product-sets/{id} | Update a set: rename, description, or replace its filters.



## create_product_set

> crate::models::ListProductSetsItem create_product_set()
Create a product set (filters, explicit picks, or both).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListProductSetsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_product_set_bulk

> crate::models::CreateProductSetBulkResponse create_product_set_bulk(id)
Run a bulk action (delist / relist / check-status) against the set's current members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateProductSetBulkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_product_set_member

> crate::models::ListProductSetsItem create_product_set_member(id)
Add / exclude / remove explicit picks for a set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListProductSetsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_product_set

> crate::models::OkResponse delete_product_set(id)
Delete a product set (its id is also dropped from any channel exclusion lists).

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


## get_product_set

> crate::models::ListProductSetsItem get_product_set(id)
Get one product set (its filters and pick counts).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListProductSetsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_product_set_members

> crate::models::ListProductSetMembersResponse list_product_set_members(id, page, limit)
Evaluate a set now and return a page of its member listings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

**crate::models::ListProductSetMembersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_product_sets

> crate::models::ListProductSetsResponse list_product_sets()
List your saved product sets with their live member counts.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListProductSetsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_product_set

> crate::models::ListProductSetsItem update_product_set(id)
Update a set: rename, description, or replace its filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListProductSetsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

