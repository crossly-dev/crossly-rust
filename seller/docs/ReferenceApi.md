# \ReferenceApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_brand**](ReferenceApi.md#get_brand) | **GET** /v1/brands | Search the Crossly brand index. Returns up to 50 matches.
[**get_category**](ReferenceApi.md#get_category) | **GET** /v1/categories | List Crossly's canonical category tree.
[**get_department**](ReferenceApi.md#get_department) | **GET** /v1/departments | Search the eBay-sourced \"Department\" item-specific values.
[**get_gender**](ReferenceApi.md#get_gender) | **GET** /v1/genders | Search the eBay-sourced \"Gender\" item-specific values.
[**get_pattern**](ReferenceApi.md#get_pattern) | **GET** /v1/patterns | Search the eBay-sourced \"Pattern\" item-specific values.
[**get_size_system**](ReferenceApi.md#get_size_system) | **GET** /v1/size-systems | Search the Poshmark + Vestiaire size-system union (US/UK/EU/AU/FR/KR).
[**get_style**](ReferenceApi.md#get_style) | **GET** /v1/styles | Search the eBay-sourced \"Style\" item-specific values.
[**get_type**](ReferenceApi.md#get_type) | **GET** /v1/types | Search the eBay-sourced \"Type\" item-specific values.
[**list_reference_countries**](ReferenceApi.md#list_reference_countries) | **GET** /v1/reference/countries | Every country with its delivery-address rules.



## get_brand

> crate::models::GetBrandResponse get_brand()
Search the Crossly brand index. Returns up to 50 matches.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBrandResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_category

> crate::models::GetCategoryResponse get_category()
List Crossly's canonical category tree.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetCategoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_department

> crate::models::GetDepartmentResponse get_department()
Search the eBay-sourced \"Department\" item-specific values.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetDepartmentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_gender

> crate::models::GetGenderResponse get_gender()
Search the eBay-sourced \"Gender\" item-specific values.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetGenderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_pattern

> crate::models::GetPatternResponse get_pattern()
Search the eBay-sourced \"Pattern\" item-specific values.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPatternResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_size_system

> crate::models::GetSizeSystemResponse get_size_system()
Search the Poshmark + Vestiaire size-system union (US/UK/EU/AU/FR/KR).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSizeSystemResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_style

> crate::models::GetStyleResponse get_style()
Search the eBay-sourced \"Style\" item-specific values.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetStyleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_type

> crate::models::GetTypeResponse get_type()
Search the eBay-sourced \"Type\" item-specific values.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetTypeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_reference_countries

> crate::models::ListReferenceCountriesResponse list_reference_countries()
Every country with its delivery-address rules.

ISO 3166-1 alpha-2 code, name, main currency and region tags, plus what a delivery address there needs: whether a postal code / state is required, what they are called locally, and the fixed state list where carriers want one. `embargoed` marks destinations Crossly cannot deliver to.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListReferenceCountriesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

