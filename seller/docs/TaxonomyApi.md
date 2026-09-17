# \TaxonomyApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_taxonomy_category**](TaxonomyApi.md#get_taxonomy_category) | **GET** /v1/taxonomy/{platform}/categories | Categories for a platform. Default is top-level; pass `?parent=<categoryId>` to drill down one level (supported on cookie platforms whose recipe returns flat parent_id-linked rows).
[**get_taxonomy_category_aspect**](TaxonomyApi.md#get_taxonomy_category_aspect) | **GET** /v1/taxonomy/{platform}/categories/{id}/aspects | Item-specific aspects (eBay) / properties (Etsy) / hard-coded enums (cookie platforms) for a category.
[**get_taxonomy_category_children**](TaxonomyApi.md#get_taxonomy_category_children) | **GET** /v1/taxonomy/{platform}/categories/{id}/children | Direct children of a category node.
[**get_taxonomy_required_field**](TaxonomyApi.md#get_taxonomy_required_field) | **GET** /v1/taxonomy/{platform}/required-fields | Normalized field schema the seller needs to fill before crossposting to this platform. Combines master fields (title/description/price/condition) with platform-specific overrides.
[**list_taxonomy_suggest**](TaxonomyApi.md#list_taxonomy_suggest) | **GET** /v1/taxonomy/{platform}/suggest | Reverse lookup — suggest categories matching a search phrase. eBay-only today.



## get_taxonomy_category

> crate::models::GetTaxonomyCategoryResponse get_taxonomy_category(platform, parent)
Categories for a platform. Default is top-level; pass `?parent=<categoryId>` to drill down one level (supported on cookie platforms whose recipe returns flat parent_id-linked rows).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**parent** | Option<**String**> |  |  |

### Return type

[**crate::models::GetTaxonomyCategoryResponse**](GetTaxonomyCategoryResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_taxonomy_category_aspect

> crate::models::GetTaxonomyCategoryAspectResponse get_taxonomy_category_aspect(platform, id)
Item-specific aspects (eBay) / properties (Etsy) / hard-coded enums (cookie platforms) for a category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetTaxonomyCategoryAspectResponse**](GetTaxonomyCategoryAspectResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_taxonomy_category_children

> crate::models::GetTaxonomyCategoryChildrenResponse get_taxonomy_category_children(platform, id)
Direct children of a category node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetTaxonomyCategoryChildrenResponse**](GetTaxonomyCategoryChildrenResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_taxonomy_required_field

> crate::models::GetTaxonomyRequiredFieldResponse get_taxonomy_required_field(platform, category_id)
Normalized field schema the seller needs to fill before crossposting to this platform. Combines master fields (title/description/price/condition) with platform-specific overrides.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**category_id** | Option<**String**> | Optional — used to inline aspects when present. |  |

### Return type

[**crate::models::GetTaxonomyRequiredFieldResponse**](GetTaxonomyRequiredFieldResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_taxonomy_suggest

> crate::models::V1List list_taxonomy_suggest(platform, q)
Reverse lookup — suggest categories matching a search phrase. eBay-only today.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**q** | Option<**String**> |  |  |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

