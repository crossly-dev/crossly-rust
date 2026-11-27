# \FieldRulesApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_field_rule**](FieldRulesApi.md#create_field_rule) | **POST** /v1/field-rules | Create a per-channel field rule.
[**create_field_rule_preview**](FieldRulesApi.md#create_field_rule_preview) | **POST** /v1/field-rules/preview | Preview field-rule output for a listing on one channel.
[**create_field_rule_reorder**](FieldRulesApi.md#create_field_rule_reorder) | **POST** /v1/field-rules/reorder | Set the run order of several field rules.
[**delete_field_rule**](FieldRulesApi.md#delete_field_rule) | **DELETE** /v1/field-rules/{id} | Delete a field rule.
[**get_field_rule**](FieldRulesApi.md#get_field_rule) | **GET** /v1/field-rules/{id} | Get one field rule.
[**list_field_rules**](FieldRulesApi.md#list_field_rules) | **GET** /v1/field-rules | List the seller's per-channel field rules.
[**update_field_rule**](FieldRulesApi.md#update_field_rule) | **PATCH** /v1/field-rules/{id} | Update a field rule.



## create_field_rule

> crate::models::ListFieldRulesItem create_field_rule()
Create a per-channel field rule.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListFieldRulesItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_field_rule_preview

> crate::models::CreateFieldRulePreviewResponse create_field_rule_preview()
Preview field-rule output for a listing on one channel.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateFieldRulePreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_field_rule_reorder

> crate::models::OkResponse create_field_rule_reorder()
Set the run order of several field rules.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_field_rule

> crate::models::OkResponse delete_field_rule(id)
Delete a field rule.

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


## get_field_rule

> crate::models::ListFieldRulesItem get_field_rule(id)
Get one field rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListFieldRulesItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_field_rules

> crate::models::ListFieldRulesResponse list_field_rules(platform)
List the seller's per-channel field rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | Option<**String**> |  |  |

### Return type

**crate::models::ListFieldRulesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_field_rule

> crate::models::ListFieldRulesItem update_field_rule(id)
Update a field rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListFieldRulesItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

