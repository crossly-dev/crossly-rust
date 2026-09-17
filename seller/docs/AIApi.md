# \AIApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ai_categorize**](AIApi.md#create_ai_categorize) | **POST** /v1/ai/categorize | Taxonomy guess from a single image URL.
[**create_ai_categorize_from_image**](AIApi.md#create_ai_categorize_from_image) | **POST** /v1/ai/categorize-from-image | Taxonomy guess from a single base64 image.
[**create_ai_enhance_description**](AIApi.md#create_ai_enhance_description) | **POST** /v1/ai/enhance-description | SEO-rewrite a listing description.
[**create_ai_enhance_listing**](AIApi.md#create_ai_enhance_listing) | **POST** /v1/ai/enhance-listing | Rewrite title + description + tags in one call.
[**create_ai_enhance_title**](AIApi.md#create_ai_enhance_title) | **POST** /v1/ai/enhance-title | SEO-rewrite a listing title.
[**create_ai_extract_receipt**](AIApi.md#create_ai_extract_receipt) | **POST** /v1/ai/extract-receipt | Structured data extraction from a receipt photo.
[**create_ai_generate_listing**](AIApi.md#create_ai_generate_listing) | **POST** /v1/ai/generate-listing | Generate full listing fields from up to 4 image URLs.
[**create_ai_help**](AIApi.md#create_ai_help) | **POST** /v1/ai/help | In-app help Q&A grounded in supplied docs.
[**create_ai_magic_listing**](AIApi.md#create_ai_magic_listing) | **POST** /v1/ai/magic-listing | Generate full listing fields from base64 photos.
[**create_ai_test_key**](AIApi.md#create_ai_test_key) | **POST** /v1/ai/test-key | Live-ping a candidate BYO-key.
[**delete_ai_key**](AIApi.md#delete_ai_key) | **DELETE** /v1/ai/key | Remove the BYO-key for a provider.
[**get_ai_provider**](AIApi.md#get_ai_provider) | **GET** /v1/ai/providers | Static catalog of supported AI providers.
[**get_ai_status**](AIApi.md#get_ai_status) | **GET** /v1/ai/status | BYO-key state for the calling user.
[**update_ai_key**](AIApi.md#update_ai_key) | **PUT** /v1/ai/key | Save an encrypted BYO-key for an AI provider.



## create_ai_categorize

> crate::models::CreateAiCategorizeResponse create_ai_categorize()
Taxonomy guess from a single image URL.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAiCategorizeResponse**](CreateAiCategorizeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ai_categorize_from_image

> crate::models::CreateAiCategorizeFromImageResponse create_ai_categorize_from_image()
Taxonomy guess from a single base64 image.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAiCategorizeFromImageResponse**](CreateAiCategorizeFromImageResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ai_enhance_description

> crate::models::CreateAiEnhanceDescriptionResponse create_ai_enhance_description()
SEO-rewrite a listing description.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAiEnhanceDescriptionResponse**](CreateAiEnhanceDescriptionResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ai_enhance_listing

> crate::models::CreateAiEnhanceListingResponse create_ai_enhance_listing()
Rewrite title + description + tags in one call.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAiEnhanceListingResponse**](CreateAiEnhanceListingResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ai_enhance_title

> crate::models::CreateAiEnhanceTitleResponse create_ai_enhance_title()
SEO-rewrite a listing title.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAiEnhanceTitleResponse**](CreateAiEnhanceTitleResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ai_extract_receipt

> crate::models::CreateAiExtractReceiptResponse create_ai_extract_receipt()
Structured data extraction from a receipt photo.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAiExtractReceiptResponse**](CreateAiExtractReceiptResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ai_generate_listing

> crate::models::CreateAiGenerateListingResponse create_ai_generate_listing()
Generate full listing fields from up to 4 image URLs.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAiGenerateListingResponse**](CreateAiGenerateListingResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ai_help

> crate::models::CreateAiHelpResponse create_ai_help()
In-app help Q&A grounded in supplied docs.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAiHelpResponse**](CreateAiHelpResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ai_magic_listing

> crate::models::CreateAiMagicListingResponse create_ai_magic_listing()
Generate full listing fields from base64 photos.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAiMagicListingResponse**](CreateAiMagicListingResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ai_test_key

> crate::models::CreateAiTestKeyResponse create_ai_test_key()
Live-ping a candidate BYO-key.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAiTestKeyResponse**](CreateAiTestKeyResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ai_key

> crate::models::DeleteAiKeyResponse delete_ai_key()
Remove the BYO-key for a provider.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DeleteAiKeyResponse**](DeleteAiKeyResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ai_provider

> crate::models::GetAiProviderResponse get_ai_provider()
Static catalog of supported AI providers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAiProviderResponse**](GetAiProviderResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ai_status

> crate::models::GetAiStatusResponse get_ai_status()
BYO-key state for the calling user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAiStatusResponse**](GetAiStatusResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ai_key

> crate::models::UpdateAiKeyResponse update_ai_key()
Save an encrypted BYO-key for an AI provider.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UpdateAiKeyResponse**](UpdateAiKeyResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

