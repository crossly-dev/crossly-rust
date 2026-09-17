# \TemplatesApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_me_template**](TemplatesApi.md#create_me_template) | **POST** /v1/me/templates | Create a template.
[**create_me_template_import**](TemplatesApi.md#create_me_template_import) | **POST** /v1/me/templates/import | Bulk-create templates from an exported payload.
[**create_me_template_render**](TemplatesApi.md#create_me_template_render) | **POST** /v1/me/templates/{id}/render | Render a template's title + description against a context.
[**create_me_template_share**](TemplatesApi.md#create_me_template_share) | **POST** /v1/me/templates/{id}/share | Mint or return an existing share token for a template.
[**delete_me_template**](TemplatesApi.md#delete_me_template) | **DELETE** /v1/me/templates/{id} | Delete a template.
[**delete_me_template_share**](TemplatesApi.md#delete_me_template_share) | **DELETE** /v1/me/templates/{id}/share | Revoke a template share link.
[**get_me_template**](TemplatesApi.md#get_me_template) | **GET** /v1/me/templates/{id} | Fetch one template in full.
[**get_me_template_suggest**](TemplatesApi.md#get_me_template_suggest) | **GET** /v1/me/templates/suggest | The seller's default template for a category.
[**list_me_templates**](TemplatesApi.md#list_me_templates) | **GET** /v1/me/templates | List the seller's templates.
[**update_me_template**](TemplatesApi.md#update_me_template) | **PATCH** /v1/me/templates/{id} | Patch a template.



## create_me_template

> crate::models::CreateMeTemplateResponse create_me_template()
Create a template.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateMeTemplateResponse**](CreateMeTemplateResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_me_template_import

> crate::models::CreateMeTemplateImportResponse create_me_template_import()
Bulk-create templates from an exported payload.

Accepts one template object or an array of them. Entries that fail validation are SKIPPED rather than failing the batch — an import is usually someone else's export, and rejecting fifty good templates over one bad row helps nobody. The response reports how many landed, so a short count is visible rather than silent.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateMeTemplateImportResponse**](CreateMeTemplateImportResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_me_template_render

> crate::models::CreateMeTemplateRenderResponse create_me_template_render(id)
Render a template's title + description against a context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateMeTemplateRenderResponse**](CreateMeTemplateRenderResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_me_template_share

> crate::models::CreateMeTemplateShareResponse create_me_template_share(id)
Mint or return an existing share token for a template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateMeTemplateShareResponse**](CreateMeTemplateShareResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_me_template

> crate::models::DeleteMeTemplateResponse delete_me_template(id)
Delete a template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteMeTemplateResponse**](DeleteMeTemplateResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_me_template_share

> crate::models::DeleteMeTemplateShareResponse delete_me_template_share(id)
Revoke a template share link.

Clears the share token, so the public link stops resolving. The template itself is untouched. Sharing again mints a NEW token — the old link is dead for good, which is the point of revoking.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteMeTemplateShareResponse**](DeleteMeTemplateShareResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_me_template

> crate::models::GetMeTemplateResponse get_me_template(id)
Fetch one template in full.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetMeTemplateResponse**](GetMeTemplateResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_me_template_suggest

> crate::models::GetMeTemplateSuggestResponse get_me_template_suggest()
The seller's default template for a category.

Returns { template: null } rather than 404 when the seller has no default for that category — \"no default\" is a normal answer, not an error, and callers hydrate a form from it.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetMeTemplateSuggestResponse**](GetMeTemplateSuggestResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_me_templates

> crate::models::V1List list_me_templates()
List the seller's templates.

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


## update_me_template

> crate::models::UpdateMeTemplateResponse update_me_template(id)
Patch a template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateMeTemplateResponse**](UpdateMeTemplateResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

