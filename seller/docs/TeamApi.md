# \TeamApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_team_accept**](TeamApi.md#create_team_accept) | **POST** /v1/team/accept | Accept a pending team invitation by raw token.
[**create_team_invite**](TeamApi.md#create_team_invite) | **POST** /v1/team/invite | Mint a team invitation; returns the one-time accept URL.
[**create_team_leave**](TeamApi.md#create_team_leave) | **POST** /v1/team/leave | Leave every team this user is currently a member of.
[**create_team_revoke**](TeamApi.md#create_team_revoke) | **POST** /v1/team/revoke | Revoke a pending invite OR an active team member.
[**get_team**](TeamApi.md#get_team) | **GET** /v1/team | List pending team invitations + active members.
[**update_team**](TeamApi.md#update_team) | **PATCH** /v1/team/{memberId} | Update a team member's scopes (owner only).



## create_team_accept

> crate::models::CreateTeamAcceptResponse create_team_accept()
Accept a pending team invitation by raw token.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateTeamAcceptResponse**](CreateTeamAcceptResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_team_invite

> crate::models::CreateTeamInviteResponse create_team_invite()
Mint a team invitation; returns the one-time accept URL.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateTeamInviteResponse**](CreateTeamInviteResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_team_leave

> crate::models::CreateTeamLeaveResponse create_team_leave()
Leave every team this user is currently a member of.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateTeamLeaveResponse**](CreateTeamLeaveResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_team_revoke

> crate::models::CreateTeamRevokeResponse create_team_revoke()
Revoke a pending invite OR an active team member.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateTeamRevokeResponse**](CreateTeamRevokeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team

> crate::models::GetTeamResponse get_team()
List pending team invitations + active members.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetTeamResponse**](GetTeamResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_team

> crate::models::UpdateTeamResponse update_team(member_id)
Update a team member's scopes (owner only).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateTeamResponse**](UpdateTeamResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

