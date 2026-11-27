# \WorldsPackagesApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_world_experience_place_package**](WorldsPackagesApi.md#create_world_experience_place_package) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/packages | Insert a copy of a package (default: the newest version, auto-update on).
[**create_world_experience_place_package_update**](WorldsPackagesApi.md#create_world_experience_place_package_update) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/packages/{nodeId}/update | Update a copy to a version (default newest). A copy edited in the place needs force: true (its edits are replaced).
[**create_world_package**](WorldsPackagesApi.md#create_world_package) | **POST** /v1/worlds/packages | Create a package (v1) from one node (place JSON) or from a node in a place — which then becomes copy v1, auto-updating.
[**create_world_package_toolbox**](WorldsPackagesApi.md#create_world_package_toolbox) | **POST** /v1/worlds/packages/{packageId}/toolbox | List the latest version in the Toolbox as an asset of kind package (Toolbox review and pricing rules apply).
[**create_world_package_version**](WorldsPackagesApi.md#create_world_package_version) | **POST** /v1/worlds/packages/{packageId}/versions | Publish a new version; every auto-updating, unmodified copy in your places is updated through its co-editing session. Returns what happened per place.
[**delete_world_package**](WorldsPackagesApi.md#delete_world_package) | **DELETE** /v1/worlds/packages/{packageId} | Archive a package. Copies in places keep working; they just stop receiving updates.
[**get_world_package**](WorldsPackagesApi.md#get_world_package) | **GET** /v1/worlds/packages/{packageId} | A package with its versions.
[**get_world_package_version**](WorldsPackagesApi.md#get_world_package_version) | **GET** /v1/worlds/packages/{packageId}/versions/{number} | One version, with its payload (one root node in place JSON).
[**list_world_experience_place_packages**](WorldsPackagesApi.md#list_world_experience_place_packages) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/packages | Package copies in a place: version, newest version, modified, auto-update.
[**list_world_packages**](WorldsPackagesApi.md#list_world_packages) | **GET** /v1/worlds/packages | Your package library: reusable models and modules, newest first, with how many places hold copies.
[**update_world_experience_place_package**](WorldsPackagesApi.md#update_world_experience_place_package) | **PATCH** /v1/worlds/experiences/{experienceId}/places/{placeId}/packages/{nodeId} | Turn auto-update on or off for one copy.
[**update_world_package**](WorldsPackagesApi.md#update_world_package) | **PATCH** /v1/worlds/packages/{packageId} | Rename a package or change its description.



## create_world_experience_place_package

> crate::models::CreateWorldExperiencePlacePackageResponse create_world_experience_place_package(experience_id, place_id)
Insert a copy of a package (default: the newest version, auto-update on).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlacePackageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_package_update

> crate::models::CreateWorldExperiencePlacePackageUpdateResponse create_world_experience_place_package_update(experience_id, place_id, node_id)
Update a copy to a version (default newest). A copy edited in the place needs force: true (its edits are replaced).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlacePackageUpdateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_package

> crate::models::GetWorldPackageResponse create_world_package()
Create a package (v1) from one node (place JSON) or from a node in a place — which then becomes copy v1, auto-updating.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldPackageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_package_toolbox

> crate::models::CreateWorldPackageToolboxResponse create_world_package_toolbox(package_id)
List the latest version in the Toolbox as an asset of kind package (Toolbox review and pricing rules apply).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPackageToolboxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_package_version

> crate::models::CreateWorldPackageVersionResponse create_world_package_version(package_id)
Publish a new version; every auto-updating, unmodified copy in your places is updated through its co-editing session. Returns what happened per place.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPackageVersionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_package

> crate::models::DeleteWorldPackageResponse delete_world_package(package_id)
Archive a package. Copies in places keep working; they just stop receiving updates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldPackageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_package

> crate::models::GetWorldPackageResponse get_world_package(package_id)
A package with its versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldPackageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_package_version

> crate::models::GetWorldPackageVersionResponse get_world_package_version(package_id, number)
One version, with its payload (one root node in place JSON).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_id** | **String** |  | [required] |
**number** | **String** |  | [required] |

### Return type

**crate::models::GetWorldPackageVersionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_place_packages

> crate::models::ListWorldExperiencePlacePackagesResponse list_world_experience_place_packages(experience_id, place_id)
Package copies in a place: version, newest version, modified, auto-update.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperiencePlacePackagesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_packages

> crate::models::ListWorldPackagesResponse list_world_packages()
Your package library: reusable models and modules, newest first, with how many places hold copies.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldPackagesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_place_package

> crate::models::UpdateWorldExperiencePlacePackageResponse update_world_experience_place_package(experience_id, place_id, node_id)
Turn auto-update on or off for one copy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperiencePlacePackageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_package

> crate::models::DeleteWorldPackageResponse update_world_package(package_id)
Rename a package or change its description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldPackageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

