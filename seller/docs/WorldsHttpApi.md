# \WorldsHttpApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_world_experience_http_domain**](WorldsHttpApi.md#create_world_experience_http_domain) | **POST** /v1/worlds/experiences/{experienceId}/http/domains | Declare a domain server scripts may call (example.com or *.example.com). It is reviewed by a person before it works.
[**delete_world_experience_http_domain**](WorldsHttpApi.md#delete_world_experience_http_domain) | **DELETE** /v1/worlds/experiences/{experienceId}/http/domains/{domainId} | Remove a domain: scripts lose access at once.
[**delete_world_experience_http_secret**](WorldsHttpApi.md#delete_world_experience_http_secret) | **DELETE** /v1/worlds/experiences/{experienceId}/http/secrets/{name} | Delete a secret.
[**get_world_experience_http**](WorldsHttpApi.md#get_world_experience_http) | **GET** /v1/worlds/experiences/{experienceId}/http | Http settings: declared domains with their review state, secret names (never values), limits and rules.
[**update_world_experience_http_secret**](WorldsHttpApi.md#update_world_experience_http_secret) | **PUT** /v1/worlds/experiences/{experienceId}/http/secrets/{name} | Set a secret (UPPER_SNAKE name). Scripts reference it as Http.secret(\"NAME\"); the value is never returned.



## create_world_experience_http_domain

> crate::models::CreateWorldExperienceHttpDomainResponse create_world_experience_http_domain(experience_id)
Declare a domain server scripts may call (example.com or *.example.com). It is reviewed by a person before it works.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceHttpDomainResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_http_domain

> crate::models::DeleteSpatialSceneGrantResponse delete_world_experience_http_domain(experience_id, domain_id)
Remove a domain: scripts lose access at once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**domain_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneGrantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_http_secret

> crate::models::DeleteSpatialSceneGrantResponse delete_world_experience_http_secret(experience_id, name)
Delete a secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneGrantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_http

> crate::models::GetWorldExperienceHttpResponse get_world_experience_http(experience_id)
Http settings: declared domains with their review state, secret names (never values), limits and rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceHttpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_http_secret

> crate::models::UpdateWorldExperienceHttpSecretResponse update_world_experience_http_secret(experience_id, name)
Set a secret (UPPER_SNAKE name). Scripts reference it as Http.secret(\"NAME\"); the value is never returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperienceHttpSecretResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

