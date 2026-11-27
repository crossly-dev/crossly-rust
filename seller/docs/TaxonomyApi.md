# \TaxonomyApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_category_by_id**](TaxonomyApi.md#get_category_by_id) | **GET** /v1/categories/{id} | One canonical category and the category it maps to on each marketplace.
[**get_taxonomy_category**](TaxonomyApi.md#get_taxonomy_category) | **GET** /v1/taxonomy/{platform}/categories | Categories for a platform. Default is top-level; pass `?parent=<categoryId>` to drill down one level (supported on cookie platforms whose recipe returns flat parent_id-linked rows).
[**get_taxonomy_category_aspect**](TaxonomyApi.md#get_taxonomy_category_aspect) | **GET** /v1/taxonomy/{platform}/categories/{id}/aspects | Item-specific aspects (eBay) / properties (Etsy) / hard-coded enums (cookie platforms) for a category.
[**get_taxonomy_category_children**](TaxonomyApi.md#get_taxonomy_category_children) | **GET** /v1/taxonomy/{platform}/categories/{id}/children | Direct children of a category node.
[**get_taxonomy_required_field**](TaxonomyApi.md#get_taxonomy_required_field) | **GET** /v1/taxonomy/{platform}/required-fields | Normalized field schema the seller needs to fill before crossposting to this platform. Combines master fields (title/description/price/condition) with platform-specific overrides.
[**list_category_aspects**](TaxonomyApi.md#list_category_aspects) | **GET** /v1/categories/{id}/aspects | Item specifics for a canonical category: required/optional, known values, per-marketplace names.
[**list_category_search**](TaxonomyApi.md#list_category_search) | **GET** /v1/categories/search | Search or browse Crossly's canonical categories (the ids listings take as categoryId).
[**list_taxonomy_ebay_charities**](TaxonomyApi.md#list_taxonomy_ebay_charities) | **GET** /v1/taxonomy/ebay/charities | Search eBay for Charity organisations (the charityId an eBay listing donates to).
[**list_taxonomy_suggest**](TaxonomyApi.md#list_taxonomy_suggest) | **GET** /v1/taxonomy/{platform}/suggest | Reverse lookup — suggest categories matching a search phrase: eBay live; Poshmark, Mercari, Depop, Grailed, Vinted, Whatnot, Vestiaire, OfferUp, Facebook and Curtsy over their harvested trees. Etsy answers an empty list (browse its tree instead).



## get_category_by_id

> crate::models::GetCategoryByIdResponse get_category_by_id(id)
One canonical category and the category it maps to on each marketplace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetCategoryByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_taxonomy_category

> crate::models::GetTaxonomyCategoryResponse get_taxonomy_category(platform, parent)
Categories for a platform. Default is top-level; pass `?parent=<categoryId>` to drill down one level (supported on cookie platforms whose recipe returns flat parent_id-linked rows).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**parent** | Option<**String**> |  |  |

### Return type

**crate::models::GetTaxonomyCategoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_taxonomy_category_aspect

> crate::models::GetTaxonomyCategoryAspectResponse get_taxonomy_category_aspect(platform, id)
Item-specific aspects (eBay) / properties (Etsy) / hard-coded enums (cookie platforms) for a category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

**crate::models::GetTaxonomyCategoryAspectResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_taxonomy_category_children

> crate::models::GetTaxonomyCategoryChildrenResponse get_taxonomy_category_children(platform, id)
Direct children of a category node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

**crate::models::GetTaxonomyCategoryChildrenResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_taxonomy_required_field

> crate::models::GetTaxonomyRequiredFieldResponse get_taxonomy_required_field(platform, category_id)
Normalized field schema the seller needs to fill before crossposting to this platform. Combines master fields (title/description/price/condition) with platform-specific overrides.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**category_id** | Option<**String**> | Optional — used to inline aspects when present. |  |

### Return type

**crate::models::GetTaxonomyRequiredFieldResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_category_aspects

> crate::models::ListCategoryAspectsResponse list_category_aspects(id, values_limit)
Item specifics for a canonical category: required/optional, known values, per-marketplace names.

Required-ness and cardinality come from the eBay category the concept maps to. `freeText` = any text is accepted; otherwise `values` are the known values (up to `valuesLimit`). `marketplaces` gives the attribute name each marketplace uses for the aspect. Send them as `itemSpecifics` on POST /v1/listings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**values_limit** | Option<**i32**> |  |  |[default to 50]

### Return type

**crate::models::ListCategoryAspectsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_category_search

> crate::models::ListCategorySearchResponse list_category_search(q, path, limit)
Search or browse Crossly's canonical categories (the ids listings take as categoryId).

`q` matches the category name and path. `path` (e.g. \"Clothing & Shoes > Men\") limits to one branch. Without `q`, `meta.branches` lists the next level under `path` (the root when omitted) so a client can walk the tree down to a leaf.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> |  |  |
**path** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |[default to 25]

### Return type

**crate::models::ListCategorySearchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_taxonomy_ebay_charities

> crate::models::ListTaxonomyEbayCharitiesResponse list_taxonomy_ebay_charities(q, limit, marketplace)
Search eBay for Charity organisations (the charityId an eBay listing donates to).

Send the chosen `charityOrgId` as `overrides.ebay.charityId` with `overrides.ebay.donationPercentage` (10-100 in steps of 5) on a listing to donate that share of the eBay sale. `q` needs at least 2 characters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |[default to 20]
**marketplace** | Option<**String**> |  |  |[default to EBAY_US]

### Return type

**crate::models::ListTaxonomyEbayCharitiesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_taxonomy_suggest

> crate::models::ListTaxonomySuggestResponse list_taxonomy_suggest(platform, q)
Reverse lookup — suggest categories matching a search phrase: eBay live; Poshmark, Mercari, Depop, Grailed, Vinted, Whatnot, Vestiaire, OfferUp, Facebook and Curtsy over their harvested trees. Etsy answers an empty list (browse its tree instead).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**q** | Option<**String**> |  |  |

### Return type

**crate::models::ListTaxonomySuggestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

