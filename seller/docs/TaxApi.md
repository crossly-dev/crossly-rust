# \TaxApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_mileage**](TaxApi.md#create_mileage) | **POST** /v1/mileage | Create a mileage entry.
[**delete_mileage**](TaxApi.md#delete_mileage) | **DELETE** /v1/mileage/{id} | Delete a mileage entry.
[**get_mileage_summary**](TaxApi.md#get_mileage_summary) | **GET** /v1/mileage/summary | Annual mileage totals + IRS deduction.
[**get_tax_schedule_c**](TaxApi.md#get_tax_schedule_c) | **GET** /v1/tax/schedule-c | Schedule C JSON for a tax year.
[**list_mileage**](TaxApi.md#list_mileage) | **GET** /v1/mileage | List mileage entries.
[**update_mileage**](TaxApi.md#update_mileage) | **PATCH** /v1/mileage/{id} | Update a mileage entry.



## create_mileage

> crate::models::CreateMileageResponse create_mileage()
Create a mileage entry.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateMileageResponse**](CreateMileageResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_mileage

> crate::models::DeleteMileageResponse delete_mileage(id)
Delete a mileage entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteMileageResponse**](DeleteMileageResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mileage_summary

> crate::models::GetMileageSummaryResponse get_mileage_summary()
Annual mileage totals + IRS deduction.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetMileageSummaryResponse**](GetMileageSummaryResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tax_schedule_c

> crate::models::GetTaxScheduleCResponse get_tax_schedule_c()
Schedule C JSON for a tax year.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetTaxScheduleCResponse**](GetTaxScheduleCResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_mileage

> crate::models::V1List list_mileage()
List mileage entries.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_mileage

> crate::models::UpdateMileageResponse update_mileage(id)
Update a mileage entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateMileageResponse**](UpdateMileageResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

