# \CustomersApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_customer_bulk_delete**](CustomersApi.md#create_customer_bulk_delete) | **POST** /v1/customers/bulk-delete | Bulk blocklist customer handles.
[**create_customer_bulk_export**](CustomersApi.md#create_customer_bulk_export) | **POST** /v1/customers/bulk-export | Bulk export aggregated customers as CSV.
[**create_customer_refresh_profile**](CustomersApi.md#create_customer_refresh_profile) | **POST** /v1/customers/{handle}/refresh-profile | Queue a fresh read of a customer's buyer profiles on every platform they ordered on.
[**get_customer**](CustomersApi.md#get_customer) | **GET** /v1/customers/{handle} | Get one customer: recent 50 orders, buyer profiles per platform, and stats over all of their orders.
[**get_loyalty_club**](CustomersApi.md#get_loyalty_club) | **GET** /v1/loyalty/club | Your loyalty club: tiers and how many members are in each.
[**list_customers**](CustomersApi.md#list_customers) | **GET** /v1/customers | List aggregated customers (group-by lower(buyer_username)), each with its best avatar, display name and star rating.
[**list_loyalty_members**](CustomersApi.md#list_loyalty_members) | **GET** /v1/loyalty/members | Your loyalty club’s members, by spend with you.
[**update_loyalty_club**](CustomersApi.md#update_loyalty_club) | **PUT** /v1/loyalty/club | Create or update your loyalty club: name, on/off, and up to 6 spend tiers each with a discount (≤ 30%) and perks.



## create_customer_bulk_delete

> crate::models::CreateCustomerBulkDeleteResponse create_customer_bulk_delete()
Bulk blocklist customer handles.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateCustomerBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_customer_bulk_export

> String create_customer_bulk_export()
Bulk export aggregated customers as CSV.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_customer_refresh_profile

> crate::models::CreateCustomerRefreshProfileResponse create_customer_refresh_profile(handle)
Queue a fresh read of a customer's buyer profiles on every platform they ordered on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |

### Return type

**crate::models::CreateCustomerRefreshProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_customer

> crate::models::GetCustomerResponse get_customer(handle)
Get one customer: recent 50 orders, buyer profiles per platform, and stats over all of their orders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |

### Return type

**crate::models::GetCustomerResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_loyalty_club

> crate::models::GetLoyaltyClubResponse get_loyalty_club()
Your loyalty club: tiers and how many members are in each.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetLoyaltyClubResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_customers

> crate::models::ListCustomersResponse list_customers()
List aggregated customers (group-by lower(buyer_username)), each with its best avatar, display name and star rating.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListCustomersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_loyalty_members

> crate::models::ListLoyaltyMembersResponse list_loyalty_members()
Your loyalty club’s members, by spend with you.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListLoyaltyMembersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_loyalty_club

> crate::models::GetLoyaltyClubResponse update_loyalty_club()
Create or update your loyalty club: name, on/off, and up to 6 spend tiers each with a discount (≤ 30%) and perks.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetLoyaltyClubResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

