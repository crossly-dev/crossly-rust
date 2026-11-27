# \CustomersApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_customer_bulk_delete**](CustomersApi.md#create_customer_bulk_delete) | **POST** /v1/customers/bulk-delete | Bulk blocklist customer handles.
[**create_customer_bulk_export**](CustomersApi.md#create_customer_bulk_export) | **POST** /v1/customers/bulk-export | Bulk export aggregated customers as CSV.
[**create_customer_refresh_profile**](CustomersApi.md#create_customer_refresh_profile) | **POST** /v1/customers/{handle}/refresh-profile | Queue a fresh read of a customer's buyer profiles on every platform they ordered on.
[**create_loyalty_airdrop_cancel**](CustomersApi.md#create_loyalty_airdrop_cancel) | **POST** /v1/loyalty/airdrops/{id}/cancel | Cancel a queued or failed airdrop (one already sent is the member’s).
[**create_loyalty_airdrop_check**](CustomersApi.md#create_loyalty_airdrop_check) | **POST** /v1/loyalty/airdrops/{id}/check | Read a submitted airdrop’s receipt again now.
[**create_loyalty_airdrop_prepare**](CustomersApi.md#create_loyalty_airdrop_prepare) | **POST** /v1/loyalty/airdrops/{id}/prepare | The mint transaction for your launch owner wallet to send: chain, from, to (your launch contract) and calldata, to the member’s verified wallet. Refused airdrop_not_sendable with a reason when it cannot be built.
[**create_loyalty_airdrop_submit**](CustomersApi.md#create_loyalty_airdrop_submit) | **POST** /v1/loyalty/airdrops/{id}/submit | Record the hash your wallet broadcast; Crossly reads the receipt (now, then on a schedule) and marks it sent or failed.
[**create_loyalty_program_invite**](CustomersApi.md#create_loyalty_program_invite) | **POST** /v1/loyalty/program/invite | Invite Crossly buyers to your loyalty program by user id (up to 500); returns how many were newly invited.
[**create_loyalty_program_member_override**](CustomersApi.md#create_loyalty_program_member_override) | **POST** /v1/loyalty/program/members/{id}/override | Pin a member to a tier (tierId null clears the pin), with a reason.
[**create_loyalty_tier**](CustomersApi.md#create_loyalty_tier) | **POST** /v1/loyalty/tiers | Add a tier: name, look, threshold and typed perks (% off, free shipping, early access, members-only listings, wholesale access, token gates, airdrops, Bucks, World items, event tickets, …).
[**delete_loyalty_tier**](CustomersApi.md#delete_loyalty_tier) | **DELETE** /v1/loyalty/tiers/{id} | Delete a tier; its members are re-evaluated.
[**get_customer**](CustomersApi.md#get_customer) | **GET** /v1/customers/{handle} | Get one customer: recent 50 orders, buyer profiles per platform, and stats over all of their orders.
[**get_customer_note**](CustomersApi.md#get_customer_note) | **GET** /v1/customers/{handle}/notes | Your private notes on a customer (empty when none). Works before their first order.
[**get_loyalty_airdrop**](CustomersApi.md#get_loyalty_airdrop) | **GET** /v1/loyalty/airdrops/{id} | One loyalty NFT airdrop: member, collection, recipient wallet, transaction hash and status.
[**get_loyalty_club**](CustomersApi.md#get_loyalty_club) | **GET** /v1/loyalty/club | Your loyalty club: tiers and how many members are in each.
[**get_loyalty_program**](CustomersApi.md#get_loyalty_program) | **GET** /v1/loyalty/program | Your loyalty program: qualification (spend, orders, points), window, joining, downgrade and grace, and its tiers with typed perks.
[**get_loyalty_program_member**](CustomersApi.md#get_loyalty_program_member) | **GET** /v1/loyalty/program/members/{id} | One member: standing, tier, progress, perks held, history and any override.
[**list_customers**](CustomersApi.md#list_customers) | **GET** /v1/customers | List aggregated customers (group-by lower(buyer_username)), each with its best avatar, display name and star rating.
[**list_loyalty_airdrops**](CustomersApi.md#list_loyalty_airdrops) | **GET** /v1/loyalty/airdrops | NFT airdrops your loyalty tiers earned members: queued to send, submitted (waiting on the receipt), sent, failed (with why) or cancelled.
[**list_loyalty_members**](CustomersApi.md#list_loyalty_members) | **GET** /v1/loyalty/members | Your loyalty club’s members, by spend with you.
[**list_loyalty_program_invite_candidates**](CustomersApi.md#list_loyalty_program_invite_candidates) | **GET** /v1/loyalty/program/invite-candidates | Crossly buyers who bought from you and are not loyalty members yet (q searches by name): who you can invite.
[**list_loyalty_program_members**](CustomersApi.md#list_loyalty_program_members) | **GET** /v1/loyalty/program/members | Your loyalty members with tier, standing, grace and overrides; search, tier, status and sort filters.
[**update_customer_note**](CustomersApi.md#update_customer_note) | **PUT** /v1/customers/{handle}/notes | Save your private notes on a customer (an empty string clears them).
[**update_loyalty_club**](CustomersApi.md#update_loyalty_club) | **PUT** /v1/loyalty/club | Create or update your loyalty club: name, on/off, and up to 6 spend tiers each with a discount (≤ 30%) and perks.
[**update_loyalty_program**](CustomersApi.md#update_loyalty_program) | **PUT** /v1/loyalty/program | Create or update your loyalty program settings.
[**update_loyalty_tier**](CustomersApi.md#update_loyalty_tier) | **PUT** /v1/loyalty/tiers/{id} | Replace a tier. Members holding it get the edited perks.



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


## create_loyalty_airdrop_cancel

> crate::models::LoyaltyAirdropResponse create_loyalty_airdrop_cancel(id)
Cancel a queued or failed airdrop (one already sent is the member’s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::LoyaltyAirdropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_loyalty_airdrop_check

> crate::models::LoyaltyAirdropResponse create_loyalty_airdrop_check(id)
Read a submitted airdrop’s receipt again now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::LoyaltyAirdropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_loyalty_airdrop_prepare

> crate::models::CreateLoyaltyAirdropPrepareResponse create_loyalty_airdrop_prepare(id)
The mint transaction for your launch owner wallet to send: chain, from, to (your launch contract) and calldata, to the member’s verified wallet. Refused airdrop_not_sendable with a reason when it cannot be built.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateLoyaltyAirdropPrepareResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_loyalty_airdrop_submit

> crate::models::LoyaltyAirdropResponse create_loyalty_airdrop_submit(id)
Record the hash your wallet broadcast; Crossly reads the receipt (now, then on a schedule) and marks it sent or failed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::LoyaltyAirdropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_loyalty_program_invite

> crate::models::CreateLoyaltyProgramInviteResponse create_loyalty_program_invite()
Invite Crossly buyers to your loyalty program by user id (up to 500); returns how many were newly invited.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateLoyaltyProgramInviteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_loyalty_program_member_override

> crate::models::GetLoyaltyProgramMemberResponse create_loyalty_program_member_override(id)
Pin a member to a tier (tierId null clears the pin), with a reason.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetLoyaltyProgramMemberResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_loyalty_tier

> crate::models::CreateLoyaltyTierResponse create_loyalty_tier()
Add a tier: name, look, threshold and typed perks (% off, free shipping, early access, members-only listings, wholesale access, token gates, airdrops, Bucks, World items, event tickets, …).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateLoyaltyTierResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_loyalty_tier

> crate::models::UpdateGroupBuyResponse delete_loyalty_tier(id)
Delete a tier; its members are re-evaluated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateGroupBuyResponse**

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


## get_customer_note

> crate::models::GetCustomerNoteResponse get_customer_note(handle)
Your private notes on a customer (empty when none). Works before their first order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |

### Return type

**crate::models::GetCustomerNoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_loyalty_airdrop

> crate::models::LoyaltyAirdropResponse get_loyalty_airdrop(id)
One loyalty NFT airdrop: member, collection, recipient wallet, transaction hash and status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::LoyaltyAirdropResponse**

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


## get_loyalty_program

> crate::models::GetLoyaltyProgramResponse get_loyalty_program()
Your loyalty program: qualification (spend, orders, points), window, joining, downgrade and grace, and its tiers with typed perks.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetLoyaltyProgramResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_loyalty_program_member

> crate::models::GetLoyaltyProgramMemberResponse get_loyalty_program_member(id)
One member: standing, tier, progress, perks held, history and any override.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetLoyaltyProgramMemberResponse**

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


## list_loyalty_airdrops

> crate::models::ListLoyaltyAirdropsResponse list_loyalty_airdrops()
NFT airdrops your loyalty tiers earned members: queued to send, submitted (waiting on the receipt), sent, failed (with why) or cancelled.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListLoyaltyAirdropsResponse**

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


## list_loyalty_program_invite_candidates

> crate::models::ListLoyaltyProgramInviteCandidatesResponse list_loyalty_program_invite_candidates()
Crossly buyers who bought from you and are not loyalty members yet (q searches by name): who you can invite.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListLoyaltyProgramInviteCandidatesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_loyalty_program_members

> crate::models::ListLoyaltyProgramMembersResponse list_loyalty_program_members()
Your loyalty members with tier, standing, grace and overrides; search, tier, status and sort filters.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListLoyaltyProgramMembersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_customer_note

> crate::models::GetCustomerNoteResponse update_customer_note(handle, update_customer_note_request)
Save your private notes on a customer (an empty string clears them).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |
**update_customer_note_request** | **UpdateCustomerNoteRequest** |  | [required] |

### Return type

**crate::models::GetCustomerNoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
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


## update_loyalty_program

> crate::models::UpdateLoyaltyProgramResponse update_loyalty_program()
Create or update your loyalty program settings.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateLoyaltyProgramResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_loyalty_tier

> crate::models::CreateLoyaltyTierResponse update_loyalty_tier(id)
Replace a tier. Members holding it get the edited perks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateLoyaltyTierResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

