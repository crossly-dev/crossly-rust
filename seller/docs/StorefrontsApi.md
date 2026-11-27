# \StorefrontsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_storefront**](StorefrontsApi.md#create_storefront) | **POST** /v1/storefronts | Create a store: name (1-60) and optional slug (3-40 lowercase letters, numbers, hyphens; derived from the name when omitted). Your first store becomes the default.
[**create_storefront_listing_move**](StorefrontsApi.md#create_storefront_listing_move) | **POST** /v1/storefronts/{id}/listings/{platformListingId}/move | Move a Crossly listing (by its platform_listing id) into this store.
[**create_storefront_onboarding_skip**](StorefrontsApi.md#create_storefront_onboarding_skip) | **POST** /v1/storefronts/onboarding/skip | Skip (or dismiss) the setup wizard: ensures your default store exists, with a generated name you can rename any time. Idempotent.
[**create_storefront_onboarding_suggest**](StorefrontsApi.md#create_storefront_onboarding_suggest) | **POST** /v1/storefronts/onboarding/suggest | Suggest another generated store name with a free matching slug.
[**create_storefront_policy_from_term**](StorefrontsApi.md#create_storefront_policy_from_term) | **POST** /v1/storefronts/{id}/policies/from-terms | Create a policy preset from this store's current terms (body: { kind: 'shipping' | 'return' | 'payment' }) and assign it to the store. Returns { preset, policies }.
[**create_storefront_policy_use_shared**](StorefrontsApi.md#create_storefront_policy_use_shared) | **POST** /v1/storefronts/{id}/policies/use-shared | Make a store use your shared policies: a non-default store drops its own return/shipping/payment policy copies and shows your default policies from then on. No change on the default store. Returns the store settings (policySource per kind, usesOwnPolicy).
[**create_storefront_publish**](StorefrontsApi.md#create_storefront_publish) | **POST** /v1/storefronts/{id}/publish | Flip a store live. Fails with storefront_requirements_missing (and the missing list) when ship-from, shipping policy, return policy or payouts are not set.
[**create_storefront_section_by_id**](StorefrontsApi.md#create_storefront_section_by_id) | **POST** /v1/storefronts/{id}/sections | Add a section to a store (kind hero | listings | collections | about | policies | social | announcement | faq | creations).
[**delete_storefront**](StorefrontsApi.md#delete_storefront) | **DELETE** /v1/storefronts/{id} | Archive a store. If it was the default, your oldest remaining store becomes the default.
[**delete_storefront_section_by_section_id**](StorefrontsApi.md#delete_storefront_section_by_section_id) | **DELETE** /v1/storefronts/{id}/sections/{sectionId} | Remove a section from a store. Listings on a removed shelf stay live.
[**delete_storefront_team_access**](StorefrontsApi.md#delete_storefront_team_access) | **DELETE** /v1/storefronts/team-access | Revoke a team member's access to one store (or their all-stores grant).
[**get_hub_setup**](StorefrontsApi.md#get_hub_setup) | **GET** /v1/hub/setup/{section} | A Hub section's setup checklist: each step, whether it is done and the page that completes it. Derived from what exists, never stored.
[**get_storefront_by_id**](StorefrontsApi.md#get_storefront_by_id) | **GET** /v1/storefronts/{id} | One store: name, slug, status (draft|live|paused|archived), isDefault, nameAuto, onboardedAt.
[**get_storefront_onboarding**](StorefrontsApi.md#get_storefront_onboarding) | **GET** /v1/storefronts/onboarding | Crossly Shop setup state: whether you have a store, the generated name/slug a skip would use, each optional step and whether it is done, the listing-gate requirements, payouts readiness and the public URL once live.
[**get_storefront_policy**](StorefrontsApi.md#get_storefront_policy) | **GET** /v1/storefronts/{id}/policies | A store's policies: per kind (shipping, return, payment) the preset the store picked (presetId, null = your default) and what the store resolves to (resolvedPresetId, resolvedPresetName, level: store | seller_default | legacy | default, body).
[**get_storefront_requirement**](StorefrontsApi.md#get_storefront_requirement) | **GET** /v1/storefronts/{id}/requirements | What this store still needs before a NEW Crossly listing can publish into it: ship_from, shipping_policy, return_policy, payouts — with where to fix each.
[**get_storefront_setting**](StorefrontsApi.md#get_storefront_setting) | **GET** /v1/storefronts/{id}/settings | A store's settings: design (banner, logo, theme, SEO, social, published), ship-from, return address, policies (storePolicies: the store's preset picks + what each kind resolves to), custom-order flag, featured listings, bundle discount, custom domain. DEPRECATED aliases, resolved through the policy cascade: offersFreeShipping, freeShippingThresholdCents, offersLocalPickup, localCourierEnabled, acceptsLayaway, sellerProtections, requestArrivalCondition, arrivalConditionWindowHours — use /v1/storefronts/:id/policies and the preset fields.
[**get_storefront_slug_available**](StorefrontsApi.md#get_storefront_slug_available) | **GET** /v1/storefronts/slug-available | Is this store slug free? Checks shape, reserved words, current slugs and retired ones.
[**list_storefront_sections_by_id**](StorefrontsApi.md#list_storefront_sections_by_id) | **GET** /v1/storefronts/{id}/sections | A store's sections in display order.
[**list_storefront_team_access**](StorefrontsApi.md#list_storefront_team_access) | **GET** /v1/storefronts/team-access | Every active per-store grant you have made to team members (storefrontId null = all stores).
[**list_storefronts**](StorefrontsApi.md#list_storefronts) | **GET** /v1/storefronts | Your stores (default first). includeArchived=true adds archived ones.
[**update_storefront_by_id**](StorefrontsApi.md#update_storefront_by_id) | **PATCH** /v1/storefronts/{id} | Rename a store. Never changes the slug or your @handle.
[**update_storefront_default**](StorefrontsApi.md#update_storefront_default) | **PUT** /v1/storefronts/{id}/default | Make this store your default (where a Crossly publish with no storefrontId lands).
[**update_storefront_policy**](StorefrontsApi.md#update_storefront_policy) | **PUT** /v1/storefronts/{id}/policies | Set a store's policies: shippingPresetId, returnPresetId, paymentPresetId (a policy preset of that kind you own, or null for your default). Omitted = unchanged. Listings in the store with no preset of their own follow these.
[**update_storefront_section_by_section_id**](StorefrontsApi.md#update_storefront_section_by_section_id) | **PATCH** /v1/storefronts/{id}/sections/{sectionId} | Rename, show/hide, refill (listings shelves) or reconfigure one section of a store.
[**update_storefront_section_reorder_by_id**](StorefrontsApi.md#update_storefront_section_reorder_by_id) | **PUT** /v1/storefronts/{id}/sections/reorder | Set a store's section order in one atomic call: every section id, top to bottom.
[**update_storefront_setting**](StorefrontsApi.md#update_storefront_setting) | **PUT** /v1/storefronts/{id}/settings | Update a store's settings. Omitted fields are unchanged, null clears. Policies replace wholesale. shippingPresetId / returnPresetId / paymentPresetId pick the store's presets. The checkout-term fields (offersFreeShipping … arrivalConditionWindowHours) are DEPRECATED aliases: they are written into the store's preset.
[**update_storefront_slug**](StorefrontsApi.md#update_storefront_slug) | **PUT** /v1/storefronts/{id}/slug | Change a store's slug. The old slug keeps redirecting to the store.
[**update_storefront_status**](StorefrontsApi.md#update_storefront_status) | **PUT** /v1/storefronts/{id}/status | Set a store's status: draft | live | paused | archived. A live store is public once it has a live listing.
[**update_storefront_team_access**](StorefrontsApi.md#update_storefront_team_access) | **PUT** /v1/storefronts/team-access | Grant (or replace) a team member's access to one store, or to every store (storefrontId null), with scopes.



## create_storefront

> crate::models::StorefrontResponse create_storefront()
Create a store: name (1-60) and optional slug (3-40 lowercase letters, numbers, hyphens; derived from the name when omitted). Your first store becomes the default.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_listing_move

> crate::models::OkResponse create_storefront_listing_move(id, platform_listing_id)
Move a Crossly listing (by its platform_listing id) into this store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platform_listing_id** | **String** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_onboarding_skip

> crate::models::StorefrontResponse create_storefront_onboarding_skip()
Skip (or dismiss) the setup wizard: ensures your default store exists, with a generated name you can rename any time. Idempotent.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_onboarding_suggest

> crate::models::CreateStorefrontOnboardingSuggestResponse create_storefront_onboarding_suggest()
Suggest another generated store name with a free matching slug.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateStorefrontOnboardingSuggestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_policy_from_term

> crate::models::CreateStorefrontPolicyFromTermResponse create_storefront_policy_from_term(id)
Create a policy preset from this store's current terms (body: { kind: 'shipping' | 'return' | 'payment' }) and assign it to the store. Returns { preset, policies }.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateStorefrontPolicyFromTermResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_policy_use_shared

> crate::models::UpdateStorefrontSettingResponse create_storefront_policy_use_shared(id)
Make a store use your shared policies: a non-default store drops its own return/shipping/payment policy copies and shows your default policies from then on. No change on the default store. Returns the store settings (policySource per kind, usesOwnPolicy).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateStorefrontSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_publish

> crate::models::StorefrontResponse create_storefront_publish(id)
Flip a store live. Fails with storefront_requirements_missing (and the missing list) when ship-from, shipping policy, return policy or payouts are not set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_section_by_id

> crate::models::StorefrontSectionResponse create_storefront_section_by_id(id)
Add a section to a store (kind hero | listings | collections | about | policies | social | announcement | faq | creations).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontSectionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_storefront

> crate::models::StorefrontResponse delete_storefront(id)
Archive a store. If it was the default, your oldest remaining store becomes the default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_storefront_section_by_section_id

> crate::models::OkResponse delete_storefront_section_by_section_id(id, section_id)
Remove a section from a store. Listings on a removed shelf stay live.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**section_id** | **String** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_storefront_team_access

> crate::models::OkResponse delete_storefront_team_access()
Revoke a team member's access to one store (or their all-stores grant).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_hub_setup

> crate::models::GetHubSetupResponse get_hub_setup(section)
A Hub section's setup checklist: each step, whether it is done and the page that completes it. Derived from what exists, never stored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section** | **String** |  | [required] |

### Return type

**crate::models::GetHubSetupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_storefront_by_id

> crate::models::StorefrontResponse get_storefront_by_id(id)
One store: name, slug, status (draft|live|paused|archived), isDefault, nameAuto, onboardedAt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_storefront_onboarding

> crate::models::GetStorefrontOnboardingResponse get_storefront_onboarding()
Crossly Shop setup state: whether you have a store, the generated name/slug a skip would use, each optional step and whether it is done, the listing-gate requirements, payouts readiness and the public URL once live.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetStorefrontOnboardingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_storefront_policy

> crate::models::GetStorefrontPolicyResponse get_storefront_policy(id)
A store's policies: per kind (shipping, return, payment) the preset the store picked (presetId, null = your default) and what the store resolves to (resolvedPresetId, resolvedPresetName, level: store | seller_default | legacy | default, body).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetStorefrontPolicyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_storefront_requirement

> crate::models::GetStorefrontRequirementResponse get_storefront_requirement(id)
What this store still needs before a NEW Crossly listing can publish into it: ship_from, shipping_policy, return_policy, payouts — with where to fix each.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetStorefrontRequirementResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_storefront_setting

> crate::models::GetStorefrontSettingResponse get_storefront_setting(id)
A store's settings: design (banner, logo, theme, SEO, social, published), ship-from, return address, policies (storePolicies: the store's preset picks + what each kind resolves to), custom-order flag, featured listings, bundle discount, custom domain. DEPRECATED aliases, resolved through the policy cascade: offersFreeShipping, freeShippingThresholdCents, offersLocalPickup, localCourierEnabled, acceptsLayaway, sellerProtections, requestArrivalCondition, arrivalConditionWindowHours — use /v1/storefronts/:id/policies and the preset fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetStorefrontSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_storefront_slug_available

> crate::models::GetStorefrontSlugAvailableResponse get_storefront_slug_available()
Is this store slug free? Checks shape, reserved words, current slugs and retired ones.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetStorefrontSlugAvailableResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_storefront_sections_by_id

> crate::models::StorefrontResponse4 list_storefront_sections_by_id(id)
A store's sections in display order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontResponse4**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_storefront_team_access

> crate::models::ListStorefrontTeamAccessResponse list_storefront_team_access()
Every active per-store grant you have made to team members (storefrontId null = all stores).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListStorefrontTeamAccessResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_storefronts

> crate::models::ListStorefrontsResponse list_storefronts()
Your stores (default first). includeArchived=true adds archived ones.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListStorefrontsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_by_id

> crate::models::StorefrontResponse update_storefront_by_id(id)
Rename a store. Never changes the slug or your @handle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_default

> crate::models::StorefrontResponse update_storefront_default(id)
Make this store your default (where a Crossly publish with no storefrontId lands).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_policy

> crate::models::GetStorefrontPolicyResponse update_storefront_policy(id)
Set a store's policies: shippingPresetId, returnPresetId, paymentPresetId (a policy preset of that kind you own, or null for your default). Omitted = unchanged. Listings in the store with no preset of their own follow these.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetStorefrontPolicyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_section_by_section_id

> crate::models::StorefrontSectionResponse update_storefront_section_by_section_id(id, section_id)
Rename, show/hide, refill (listings shelves) or reconfigure one section of a store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**section_id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontSectionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_section_reorder_by_id

> crate::models::StorefrontResponse4 update_storefront_section_reorder_by_id(id)
Set a store's section order in one atomic call: every section id, top to bottom.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontResponse4**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_setting

> crate::models::UpdateStorefrontSettingResponse update_storefront_setting(id)
Update a store's settings. Omitted fields are unchanged, null clears. Policies replace wholesale. shippingPresetId / returnPresetId / paymentPresetId pick the store's presets. The checkout-term fields (offersFreeShipping … arrivalConditionWindowHours) are DEPRECATED aliases: they are written into the store's preset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateStorefrontSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_slug

> crate::models::StorefrontResponse update_storefront_slug(id)
Change a store's slug. The old slug keeps redirecting to the store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_status

> crate::models::StorefrontResponse update_storefront_status(id)
Set a store's status: draft | live | paused | archived. A live store is public once it has a live listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_team_access

> crate::models::OkResponse update_storefront_team_access()
Grant (or replace) a team member's access to one store, or to every store (storefrontId null), with scopes.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

