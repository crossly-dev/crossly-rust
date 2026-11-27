# \PriceRulesApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_price_rule_preview**](PriceRulesApi.md#create_price_rule_preview) | **POST** /v1/price-rules/preview | Compute the price each marketplace would list a listing (or a price) at.
[**delete_price_rule**](PriceRulesApi.md#delete_price_rule) | **DELETE** /v1/price-rules/{id} | Delete a price rule.
[**list_price_rules**](PriceRulesApi.md#list_price_rules) | **GET** /v1/price-rules | List the seller's per-channel price rules.
[**update_price_rule**](PriceRulesApi.md#update_price_rule) | **PUT** /v1/price-rules | Create or replace the price rule for a platform (and optional account / category).
[**update_price_rule_by_id**](PriceRulesApi.md#update_price_rule_by_id) | **PATCH** /v1/price-rules/{id} | Update a price rule.



## create_price_rule_preview

> crate::models::CreatePriceRulePreviewResponse create_price_rule_preview()
Compute the price each marketplace would list a listing (or a price) at.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreatePriceRulePreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_price_rule

> crate::models::OkResponse delete_price_rule(id)
Delete a price rule.

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


## list_price_rules

> crate::models::ListPriceRulesResponse list_price_rules(platform)
List the seller's per-channel price rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | Option<**String**> |  |  |

### Return type

**crate::models::ListPriceRulesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_price_rule

> crate::models::ListPriceRulesItem update_price_rule()
Create or replace the price rule for a platform (and optional account / category).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListPriceRulesItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_price_rule_by_id

> crate::models::ListPriceRulesItem update_price_rule_by_id(id)
Update a price rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListPriceRulesItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

