# \WorldsGalleryApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_world_experience_gallery**](WorldsGalleryApi.md#create_world_experience_gallery) | **POST** /v1/worlds/experiences/{experienceId}/gallery | Add an uploaded screenshot or video (your own upload's publicUrl; a video may carry a poster image). Images are scanned at once; every item waits for a person and shows on the game's page once approved.
[**create_world_experience_gallery_upload_url**](WorldsGalleryApi.md#create_world_experience_gallery_upload_url) | **POST** /v1/worlds/experiences/{experienceId}/gallery/upload-url | A presigned upload for a gallery screenshot (JPEG/PNG/WebP/GIF, ≤ 10 MB) or video (MP4/QuickTime/WebM): PUT the file to uploadUrl with the same Content-Type and exact Content-Length, then add publicUrl to the gallery.
[**delete_world_experience_gallery**](WorldsGalleryApi.md#delete_world_experience_gallery) | **DELETE** /v1/worlds/experiences/{experienceId}/gallery/{mediaId} | Take an item off the game's page now.
[**get_world_experience_gallery**](WorldsGalleryApi.md#get_world_experience_gallery) | **GET** /v1/worlds/experiences/{experienceId}/gallery | The game's gallery: every screenshot and video with its review state (pending / approved / rejected with the reviewer's note), position and caption, plus the limits.
[**update_world_experience_gallery**](WorldsGalleryApi.md#update_world_experience_gallery) | **PATCH** /v1/worlds/experiences/{experienceId}/gallery/{mediaId} | Change an item's caption (filtered; an approved item with a new caption goes back to review).
[**update_world_experience_gallery_order**](WorldsGalleryApi.md#update_world_experience_gallery_order) | **PUT** /v1/worlds/experiences/{experienceId}/gallery/order | Set the order on the game's page: the ids you list first, the rest after in their current order.
[**update_world_experience_gallery_role**](WorldsGalleryApi.md#update_world_experience_gallery_role) | **PUT** /v1/worlds/experiences/{experienceId}/gallery/{mediaId}/role | Make an item the game's icon or thumbnail (an image), its trailer (a video), or put it back in the gallery. The previous holder of that role goes back to the gallery.
[**update_world_experience_gallery_trailer_url**](WorldsGalleryApi.md#update_world_experience_gallery_trailer_url) | **PUT** /v1/worlds/experiences/{experienceId}/gallery/trailer-url | Set the game's trailer to a YouTube or Vimeo video link (https; youtube.com, youtu.be, vimeo.com, player.vimeo.com only; any other host is 400 trailer_host_not_allowed). It waits for a person like any gallery item; the current trailer goes back to the gallery (an uploaded clip) or off the page (a link).



## create_world_experience_gallery

> crate::models::WorldExperienceGalleryResponse create_world_experience_gallery(experience_id)
Add an uploaded screenshot or video (your own upload's publicUrl; a video may carry a poster image). Images are scanned at once; every item waits for a person and shows on the game's page once approved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceGalleryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_gallery_upload_url

> crate::models::CreateWorldExperienceGalleryUploadUrlResponse create_world_experience_gallery_upload_url(experience_id)
A presigned upload for a gallery screenshot (JPEG/PNG/WebP/GIF, ≤ 10 MB) or video (MP4/QuickTime/WebM): PUT the file to uploadUrl with the same Content-Type and exact Content-Length, then add publicUrl to the gallery.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceGalleryUploadUrlResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_gallery

> crate::models::DeleteSpatialSceneGrantResponse delete_world_experience_gallery(experience_id, media_id)
Take an item off the game's page now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**media_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneGrantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_gallery

> crate::models::GetWorldExperienceGalleryResponse get_world_experience_gallery(experience_id)
The game's gallery: every screenshot and video with its review state (pending / approved / rejected with the reviewer's note), position and caption, plus the limits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceGalleryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_gallery

> crate::models::WorldExperienceGalleryResponse update_world_experience_gallery(experience_id, media_id)
Change an item's caption (filtered; an approved item with a new caption goes back to review).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**media_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceGalleryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_gallery_order

> crate::models::UpdateWorldExperienceGalleryRoleResponse update_world_experience_gallery_order(experience_id)
Set the order on the game's page: the ids you list first, the rest after in their current order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperienceGalleryRoleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_gallery_role

> crate::models::UpdateWorldExperienceGalleryRoleResponse update_world_experience_gallery_role(experience_id, media_id)
Make an item the game's icon or thumbnail (an image), its trailer (a video), or put it back in the gallery. The previous holder of that role goes back to the gallery.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**media_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperienceGalleryRoleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_gallery_trailer_url

> crate::models::WorldExperienceGalleryResponse update_world_experience_gallery_trailer_url(experience_id)
Set the game's trailer to a YouTube or Vimeo video link (https; youtube.com, youtu.be, vimeo.com, player.vimeo.com only; any other host is 400 trailer_host_not_allowed). It waits for a person like any gallery item; the current trailer goes back to the gallery (an uploaded clip) or off the page (a link).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceGalleryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

