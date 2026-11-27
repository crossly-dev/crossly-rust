// Written by codegen/integrations/extras/rust/finish.mjs: every { data, pagination } response model.
use crate::client::Paged;

impl Paged for crate::models::ListAccountsResponse {
    type Item = crate::models::ListAccountsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListActionLogCallsResponse {
    type Item = crate::models::ListActionLogCallsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListActionLogHistogramResponse {
    type Item = crate::models::ListActionLogHistogramItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListActionLogResponse {
    type Item = crate::models::ListActionLogItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListAppsResponse {
    type Item = crate::models::ListAppsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListAutomaticDiscountsResponse {
    type Item = crate::models::ListAutomaticDiscountsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListAutomationRulesResponse {
    type Item = crate::models::ListAutomationRulesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListAutomationRunsResponse {
    type Item = crate::models::ListAutomationRunsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBillingAppAllowancesResponse {
    type Item = crate::models::ListBillingAppAllowancesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBillingPlansResponse {
    type Item = crate::models::ListBillingPlansItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBrandDropListingsResponse {
    type Item = crate::models::ListBrandDropListingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBrandDropsResponse {
    type Item = crate::models::ListBrandDropsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBrandingDropsResponse {
    type Item = crate::models::ListBrandingDropsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBreaksResponse {
    type Item = crate::models::ListBreaksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBulkBatchesResponse {
    type Item = crate::models::ListBulkBatchesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCatalogMatchesResponse {
    type Item = crate::models::ListCatalogMatchesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCategoryAspectsResponse {
    type Item = crate::models::ListCategoryAspectsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCategorySearchResponse {
    type Item = crate::models::ListCategorySearchItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCbxAdCreditLedgerResponse {
    type Item = crate::models::ListCbxAdCreditLedgerItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCbxBoostsResponse {
    type Item = crate::models::ListCbxBoostsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCbxCampaignPayoutsResponse {
    type Item = crate::models::ListCbxCampaignPayoutsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCbxCampaignsResponse {
    type Item = crate::models::ListCbxCampaignsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCbxDisbursementProgressResponse {
    type Item = crate::models::ListCbxDisbursementProgressItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCbxDisbursementRulesResponse {
    type Item = crate::models::ListCbxDisbursementRulesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCbxEarnTiersResponse {
    type Item = crate::models::ListCbxEarnTiersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCbxRedemptionServicesResponse {
    type Item = crate::models::ListCbxRedemptionServicesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCbxStakeTiersResponse {
    type Item = crate::models::ListCbxStakeTiersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCbxSubjectGrantsResponse {
    type Item = crate::models::ListCbxSubjectGrantsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCbxSubjectLedgerResponse {
    type Item = crate::models::ListCbxSubjectLedgerItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCbxWalletPaymentReviewResponse {
    type Item = crate::models::ListCbxWalletPaymentReviewItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListChannelInventoryResponse {
    type Item = crate::models::ListChannelInventoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::CreatePriceRulePreviewResponse {
    type Item = crate::models::CreatePriceRulePreviewItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::CreateStoryResponse {
    type Item = crate::models::CreateStoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListAuthSessionsResponse {
    type Item = crate::models::ListAuthSessionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCompWatchlistRecentResponse {
    type Item = crate::models::ListCompWatchlistRecentItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCompWatchlistsResponse {
    type Item = crate::models::ListCompWatchlistsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListConnectedAppsResponse {
    type Item = crate::models::ListConnectedAppsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListDevicesResponse {
    type Item = crate::models::ListDevicesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListEmbedKeysResponse {
    type Item = crate::models::ListEmbedKeysItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListFestivalsResponse {
    type Item = crate::models::ListFestivalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListInsightByPlatformResponse {
    type Item = crate::models::ListInsightByPlatformItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListJobAppraisalsResponse {
    type Item = crate::models::ListJobAppraisalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListJobShiftsResponse {
    type Item = crate::models::ListJobShiftsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListLayawaysResponse {
    type Item = crate::models::ListLayawaysItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListMileageResponse {
    type Item = crate::models::ListMileageItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListMobilePushTokensResponse {
    type Item = crate::models::ListMobilePushTokensItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListNeighbourhoodsResponse {
    type Item = crate::models::ListNeighbourhoodsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListNotificationIntegrationsResponse {
    type Item = crate::models::ListNotificationIntegrationsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListPatResponse {
    type Item = crate::models::ListPatItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListPriceRulesResponse {
    type Item = crate::models::ListPriceRulesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListReferenceCountriesResponse {
    type Item = crate::models::ListReferenceCountriesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListReferenceMetrosResponse {
    type Item = crate::models::ListReferenceMetrosItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListRepriceEventsResponse {
    type Item = crate::models::ListRepriceEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListRestockPromptsResponse {
    type Item = crate::models::ListRestockPromptsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSalesResponse {
    type Item = crate::models::ListSalesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSavedViewsResponse {
    type Item = crate::models::ListSavedViewsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListScanCapturesResponse {
    type Item = crate::models::ListScanCapturesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListScanModelsResponse {
    type Item = crate::models::ListScanModelsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListStoriesResponse {
    type Item = crate::models::ListStoriesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::StorefrontResponse4 {
    type Item = crate::models::ListStorefrontSectionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::UpdateHotspotResponse {
    type Item = crate::models::UpdateHotspotItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::WalletResponse {
    type Item = crate::models::ListWalletDefaultsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::WorldLinkResponse {
    type Item = crate::models::GetWorldLinkSellerItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::GetConnectionPolicyManagementOptionResponse {
    type Item = crate::models::GetConnectionPolicyManagementOptionItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListConnectionPolicyManagementResponse {
    type Item = crate::models::ListConnectionPolicyManagementItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListConnectionsResponse {
    type Item = crate::models::ListConnectionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListConversationMessagesResponse {
    type Item = crate::models::ListConversationMessagesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListConversationsResponse {
    type Item = crate::models::ListConversationsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCosmAttestationsResponse {
    type Item = crate::models::ListCosmAttestationsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCosmAuctionResponse {
    type Item = crate::models::ListCosmAuctionItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCosmCosmeticsResponse {
    type Item = crate::models::ListCosmCosmeticsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCosmGoalsResponse {
    type Item = crate::models::ListCosmGoalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCosmInventoryResponse {
    type Item = crate::models::ListCosmInventoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCosmPegsResponse {
    type Item = crate::models::ListCosmPegsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCosmSupplyEpochsResponse {
    type Item = crate::models::ListCosmSupplyEpochsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCosmSupplyScheduleResponse {
    type Item = crate::models::ListCosmSupplyScheduleItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCosmUnlocksResponse {
    type Item = crate::models::ListCosmUnlocksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCosmWithdrawalsResponse {
    type Item = crate::models::ListCosmWithdrawalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCreatorCardSetsResponse {
    type Item = crate::models::ListCreatorCardSetsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCreatorCreationReviewsResponse {
    type Item = crate::models::ListCreatorCreationReviewsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCreatorCreationsResponse {
    type Item = crate::models::ListCreatorCreationsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCreatorEarningTrendResponse {
    type Item = crate::models::ListCreatorEarningTrendItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCreatorEarningsResponse {
    type Item = crate::models::ListCreatorEarningsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCreatorExperienceKeysResponse {
    type Item = crate::models::ListCreatorExperienceKeysItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCreatorItemModelsResponse {
    type Item = crate::models::ListCreatorItemModelsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCreatorItemWearablesResponse {
    type Item = crate::models::ListCreatorItemWearablesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCreatorItemsResponse {
    type Item = crate::models::ListCreatorItemsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCreatorReviewsResponse {
    type Item = crate::models::ListCreatorReviewsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCreatorRewardMeResponse {
    type Item = crate::models::ListCreatorRewardMeItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCreatorRewardMonthsResponse {
    type Item = crate::models::ListCreatorRewardMonthsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListCustomersResponse {
    type Item = crate::models::ListCustomersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListDaoProposalsResponse {
    type Item = crate::models::ListDaoProposalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListDetectedEventEventsResponse {
    type Item = crate::models::ListDetectedEventEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListDetectedEventRulesResponse {
    type Item = crate::models::ListDetectedEventRulesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListDeveloperAppClientVersionsResponse {
    type Item = crate::models::ListDeveloperAppClientVersionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListDeveloperAppConformanceRunsResponse {
    type Item = crate::models::ListDeveloperAppConformanceRunsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListDiscountCodeRedemptionsResponse {
    type Item = crate::models::ListDiscountCodeRedemptionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListDiscountCodesResponse {
    type Item = crate::models::ListDiscountCodesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListDiscountHoldersResponse {
    type Item = crate::models::ListDiscountHoldersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListDropMineResponse {
    type Item = crate::models::ListDropMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListEmailEventsResponse {
    type Item = crate::models::ListEmailEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListEmailProvidersResponse {
    type Item = crate::models::ListEmailProvidersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListEventAttendeesResponse {
    type Item = crate::models::ListEventAttendeesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListEventPolicyDecisionsResponse {
    type Item = crate::models::ListEventPolicyDecisionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListEventsResponse {
    type Item = crate::models::ListEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListFieldRulesResponse {
    type Item = crate::models::ListFieldRulesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListFundingApprovalsResponse {
    type Item = crate::models::ListFundingApprovalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListFundingDrawsResponse {
    type Item = crate::models::ListFundingDrawsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListFundingReceiptsResponse {
    type Item = crate::models::ListFundingReceiptsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListFundingRepaymentsResponse {
    type Item = crate::models::ListFundingRepaymentsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListGameBreakUpcomingResponse {
    type Item = crate::models::ListGameBreakUpcomingItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListGameCouponsResponse {
    type Item = crate::models::ListGameCouponsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListGameHuntMineResponse {
    type Item = crate::models::ListGameHuntMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListGameHuntsResponse {
    type Item = crate::models::ListGameHuntsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListGameResultMineResponse {
    type Item = crate::models::ListGameResultMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListGradingRateCardResponse {
    type Item = crate::models::ListGradingRateCardItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListGradingRunsResponse {
    type Item = crate::models::ListGradingRunsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListGradingSubmissionsResponse {
    type Item = crate::models::ListGradingSubmissionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListGroupBuysResponse {
    type Item = crate::models::ListGroupBuysItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListHomeMineBuildersResponse {
    type Item = crate::models::ListHomeMineBuildersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListHomeOfTheWeekResponse {
    type Item = crate::models::ListHomeOfTheWeekItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListImportsResponse {
    type Item = crate::models::ListImportsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListInboxResponse {
    type Item = crate::models::ListInboxItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListInventoryActivityResponse {
    type Item = crate::models::ListInventoryActivityItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListInventoryResponse {
    type Item = crate::models::ListInventoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListInventoryUnitsResponse {
    type Item = crate::models::ListInventoryUnitsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::CreateListingAdBulkAddResponse {
    type Item = crate::models::CreateListingAdBulkAddItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::CreateListingAdCampaignListingResponse {
    type Item = crate::models::CreateListingAdCampaignListingItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::GetListingAdListingResponse {
    type Item = crate::models::GetListingAdListingItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListListingAdCampaignsResponse {
    type Item = crate::models::ListListingAdCampaignsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListListingAdPlatformsResponse {
    type Item = crate::models::ListListingAdPlatformsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListListingAiDetectionsResponse {
    type Item = crate::models::ListListingAiDetectionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListListingCommandsResponse {
    type Item = crate::models::ListListingCommandsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListListingDiscrepanciesResponse {
    type Item = crate::models::ListListingDiscrepanciesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListListingIdsResponse {
    type Item = String;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListListingTagSuggestResponse {
    type Item = crate::models::ListListingTagSuggestItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListListingTagsResponse {
    type Item = crate::models::ListListingTagsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListListingsResponse {
    type Item = crate::models::ListListingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListLiveShipmentsResponse {
    type Item = crate::models::ListLiveShipmentsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListLiveStreamsResponse {
    type Item = crate::models::ListLiveStreamsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::UpdateLiveStreamLineupResponse {
    type Item = crate::models::UpdateLiveStreamLineupItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListLookbookCampaignRecipientsResponse {
    type Item = crate::models::ListLookbookCampaignRecipientsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListLookbookCampaignsResponse {
    type Item = crate::models::ListLookbookCampaignsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListLookbooksResponse {
    type Item = crate::models::ListLookbooksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListLoyaltyAirdropsResponse {
    type Item = crate::models::ListLoyaltyAirdropsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListLoyaltyMembersResponse {
    type Item = crate::models::ListLoyaltyMembersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListLoyaltyProgramInviteCandidatesResponse {
    type Item = crate::models::ListLoyaltyProgramInviteCandidatesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListLoyaltyProgramMembersResponse {
    type Item = crate::models::ListLoyaltyProgramMembersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListMagicAutoRunDecisionsResponse {
    type Item = crate::models::ListMagicAutoRunDecisionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListMagicRecentResponse {
    type Item = crate::models::ListMagicRecentItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListMainStageEventsResponse {
    type Item = crate::models::ListMainStageEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListMarketBullionResponse {
    type Item = crate::models::ListMarketBullionItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListMarketGradersResponse {
    type Item = crate::models::ListMarketGradersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListMarketMetalSpotResponse {
    type Item = crate::models::ListMarketMetalSpotItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListMarketProductsResponse {
    type Item = crate::models::ListMarketProductsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListMarketVariantTiersResponse {
    type Item = crate::models::ListMarketVariantTiersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListMeTemplatesResponse {
    type Item = crate::models::ListMeTemplatesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListNetworkPoolLogResponse {
    type Item = crate::models::ListNetworkPoolLogItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListOrderPickupReadyResponse {
    type Item = crate::models::ListOrderPickupReadyItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListOrderPickupsResponse {
    type Item = crate::models::ListOrderPickupsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListOrderRefundsResponse {
    type Item = crate::models::ListOrderRefundsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListOrderScanFormsResponse {
    type Item = crate::models::ListOrderScanFormsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListOrderUnitsResponse {
    type Item = crate::models::ListOrderUnitsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListOrdersResponse {
    type Item = crate::models::ListOrdersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListPassportEventsResponse {
    type Item = crate::models::ListPassportEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListPassportPrintTargetsResponse {
    type Item = crate::models::ListPassportPrintTargetsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListPassportsResponse {
    type Item = crate::models::ListPassportsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListPayoutCompareResponse {
    type Item = crate::models::ListPayoutCompareItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListPolicyPresetsResponse {
    type Item = crate::models::ListPolicyPresetsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListPolicyReturnsResponse {
    type Item = crate::models::ListPolicyReturnsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListPrintMailJobsResponse {
    type Item = crate::models::ListPrintMailJobsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListProductFeedsResponse {
    type Item = crate::models::ListProductFeedsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListProductSetMembersResponse {
    type Item = crate::models::ListProductSetMembersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListProductSetsResponse {
    type Item = crate::models::ListProductSetsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::CreateRentalBulkRentableResponse {
    type Item = crate::models::CreateRentalBulkRentableItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListRentalHistoryResponse {
    type Item = crate::models::ListRentalHistoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListRentalOffersResponse {
    type Item = crate::models::ListRentalOffersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListRentalsResponse {
    type Item = crate::models::ListRentalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListReturnsResponse {
    type Item = crate::models::ListReturnsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSandboxEventsResponse {
    type Item = crate::models::ListSandboxEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListShowroomItemSearchResponse {
    type Item = crate::models::ListShowroomItemSearchItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListShowroomsResponse {
    type Item = crate::models::ListShowroomsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSourceSyncEventsResponse {
    type Item = crate::models::ListSourceSyncEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSourceSyncResponse {
    type Item = crate::models::ListSourceSyncItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSourcingDemandMineResponse {
    type Item = crate::models::ListSourcingDemandMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSourcingDemandResponse {
    type Item = crate::models::ListSourcingDemandItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialAuctionRoomLotsResponse {
    type Item = crate::models::ListSpatialAuctionRoomLotsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialAuctionRoomsResponse {
    type Item = crate::models::ListSpatialAuctionRoomsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialDeedListingsResponse {
    type Item = crate::models::ListSpatialDeedListingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialEmployeeSessionsResponse {
    type Item = crate::models::ListSpatialEmployeeSessionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialEmployeeUsageResponse {
    type Item = crate::models::ListSpatialEmployeeUsageItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialLeasesResponse {
    type Item = crate::models::ListSpatialLeasesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialMomentsResponse {
    type Item = crate::models::ListSpatialMomentsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialPublicOffersResponse {
    type Item = crate::models::ListSpatialPublicOffersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialPublicResponse {
    type Item = crate::models::ListSpatialPublicItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialSceneGrantsResponse {
    type Item = crate::models::ListSpatialSceneGrantsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialSceneMovementsResponse {
    type Item = crate::models::ListSpatialSceneMovementsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialSceneWarehouseUnplacedResponse {
    type Item = crate::models::ListSpatialSceneWarehouseUnplacedItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialScenesResponse {
    type Item = crate::models::ListSpatialScenesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialSharedWithMeResponse {
    type Item = crate::models::ListSpatialSharedWithMeItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialShopAddressesResponse {
    type Item = crate::models::ListSpatialShopAddressesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialStaffingResponse {
    type Item = crate::models::ListSpatialStaffingItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialStreetMineResponse {
    type Item = crate::models::ListSpatialStreetMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSpatialStreetsResponse {
    type Item = crate::models::ListSpatialStreetsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSponsoredAdBillingHistoryResponse {
    type Item = crate::models::ListSponsoredAdBillingHistoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSponsoredAdBillingStatementsResponse {
    type Item = crate::models::ListSponsoredAdBillingStatementsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSponsoredAdBoostsResponse {
    type Item = crate::models::ListSponsoredAdBoostsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSponsoredAdCampaignsResponse {
    type Item = crate::models::ListSponsoredAdCampaignsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListSponsoredAdListingsResponse {
    type Item = crate::models::ListSponsoredAdListingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListStallListingSearchResponse {
    type Item = crate::models::ListStallListingSearchItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListStallMineResponse {
    type Item = crate::models::ListStallMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListStorefrontHighlightsResponse {
    type Item = crate::models::ListStorefrontHighlightsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListStorefrontSiteVersionsResponse {
    type Item = crate::models::ListStorefrontSiteVersionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListStorefrontStoryArchiveResponse {
    type Item = crate::models::ListStorefrontStoryArchiveItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListStorefrontTeamAccessResponse {
    type Item = crate::models::ListStorefrontTeamAccessItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListStorefrontsResponse {
    type Item = crate::models::ListStorefrontsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListStreetDropsResponse {
    type Item = crate::models::ListStreetDropsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListStreetEventsResponse {
    type Item = crate::models::ListStreetEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListStreetStallsResponse {
    type Item = crate::models::ListStreetStallsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListTaxCrosslyCollectsResponse {
    type Item = crate::models::ListTaxCrosslyCollectsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListTaxSalesTaxNotCollectedResponse {
    type Item = crate::models::ListTaxSalesTaxNotCollectedItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListTaxonomyEbayCharitiesResponse {
    type Item = crate::models::ListTaxonomyEbayCharitiesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListTaxonomySuggestResponse {
    type Item = crate::models::ListTaxonomySuggestItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListTcgCardsResponse {
    type Item = crate::models::ListTcgCardsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListTcgCreatorGamePacksResponse {
    type Item = crate::models::ListTcgCreatorGamePacksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListTcgCreatorGamesResponse {
    type Item = crate::models::ListTcgCreatorGamesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListTcgCreatorMineResponse {
    type Item = crate::models::ListTcgCreatorMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListTcgGamesResponse {
    type Item = crate::models::ListTcgGamesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListTcgNftsResponse {
    type Item = crate::models::ListTcgNftsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListTcgTournamentsResponse {
    type Item = crate::models::ListTcgTournamentsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListToolboxAssetsResponse {
    type Item = crate::models::ListToolboxAssetsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListToolboxLibraryResponse {
    type Item = crate::models::ListToolboxLibraryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListToolboxMineResponse {
    type Item = crate::models::ListToolboxMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListToolboxSalesResponse {
    type Item = crate::models::ListToolboxSalesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVariationGroupsResponse {
    type Item = crate::models::ListVariationGroupsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::GetVaultMarketCallResponse {
    type Item = crate::models::GetVaultMarketCallItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketAccountsResponse {
    type Item = crate::models::ListVaultMarketAccountsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketAdlEventsResponse {
    type Item = crate::models::ListVaultMarketAdlEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketAdlPositionsResponse {
    type Item = crate::models::ListVaultMarketAdlPositionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketAgentDraftsResponse {
    type Item = crate::models::ListVaultMarketAgentDraftsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketAgentWatchesResponse {
    type Item = crate::models::ListVaultMarketAgentWatchesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketAlertLogResponse {
    type Item = crate::models::ListVaultMarketAlertLogItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketAlertsResponse {
    type Item = crate::models::ListVaultMarketAlertsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketAuctionsResponse {
    type Item = crate::models::ListVaultMarketAuctionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketBlocksResponse {
    type Item = crate::models::ListVaultMarketBlocksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketBullionFamilyPegsResponse {
    type Item = crate::models::ListVaultMarketBullionFamilyPegsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketBullionKeysResponse {
    type Item = crate::models::ListVaultMarketBullionKeysItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketBullionPegsResponse {
    type Item = crate::models::ListVaultMarketBullionPegsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketCollectionWorthResponse {
    type Item = crate::models::ListVaultMarketCollectionWorthItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketConfirmationsResponse {
    type Item = crate::models::ListVaultMarketConfirmationsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketCreatorCohostInvitesResponse {
    type Item = crate::models::ListVaultMarketCreatorCohostInvitesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketCreditResponse {
    type Item = crate::models::ListVaultMarketCreditItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketDealerBandsResponse {
    type Item = crate::models::ListVaultMarketDealerBandsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketDealerBidsResponse {
    type Item = crate::models::ListVaultMarketDealerBidsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketDealerPoolFillsResponse {
    type Item = crate::models::ListVaultMarketDealerPoolFillsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketDealerPoolsResponse {
    type Item = crate::models::ListVaultMarketDealerPoolsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketDigestHistoryResponse {
    type Item = crate::models::ListVaultMarketDigestHistoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketEntityAuditResponse {
    type Item = crate::models::ListVaultMarketEntityAuditItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketEntityConversionsResponse {
    type Item = crate::models::ListVaultMarketEntityConversionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketEstimatesAccuracyResponse {
    type Item = crate::models::ListVaultMarketEstimatesAccuracyItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketFeedResponse {
    type Item = crate::models::ListVaultMarketFeedItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketFollowCopyWatchAlertsResponse {
    type Item = crate::models::ListVaultMarketFollowCopyWatchAlertsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketFollowCopyWatchResponse {
    type Item = crate::models::ListVaultMarketFollowCopyWatchItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketFollowFeedResponse {
    type Item = crate::models::ListVaultMarketFollowFeedItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketFollowFollowingResponse {
    type Item = crate::models::ListVaultMarketFollowFollowingItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketFollowLeaderboardsResponse {
    type Item = crate::models::ListVaultMarketFollowLeaderboardsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketGiftsResponse {
    type Item = crate::models::ListVaultMarketGiftsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketGradeToVaultResponse {
    type Item = crate::models::ListVaultMarketGradeToVaultItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketGradingCandidatesResponse {
    type Item = crate::models::ListVaultMarketGradingCandidatesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketGradingOrdersResponse {
    type Item = crate::models::ListVaultMarketGradingOrdersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketGradingResponse {
    type Item = crate::models::ListVaultMarketGradingItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketHallLendersResponse {
    type Item = crate::models::ListVaultMarketHallLendersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketHoldingsResponse {
    type Item = crate::models::ListVaultMarketHoldingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketIndexCommitteeMinutesResponse {
    type Item = crate::models::ListVaultMarketIndexCommitteeMinutesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketIndexComplaintsResponse {
    type Item = crate::models::ListVaultMarketIndexComplaintsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketIndexConsultationsResponse {
    type Item = crate::models::ListVaultMarketIndexConsultationsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketIndexEventsResponse {
    type Item = crate::models::ListVaultMarketIndexEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketIndexJudgmentsResponse {
    type Item = crate::models::ListVaultMarketIndexJudgmentsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketIndexLevelSeriesResponse {
    type Item = crate::models::ListVaultMarketIndexLevelSeriesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketIndexLicenceMineResponse {
    type Item = crate::models::ListVaultMarketIndexLicenceMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketIndexMembersResponse {
    type Item = crate::models::ListVaultMarketIndexMembersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketIndexesResponse {
    type Item = crate::models::ListVaultMarketIndexesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketInstantExitHistoryResponse {
    type Item = crate::models::ListVaultMarketInstantExitHistoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketInstantExitHoldingsResponse {
    type Item = crate::models::ListVaultMarketInstantExitHoldingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketJointTradesResponse {
    type Item = crate::models::ListVaultMarketJointTradesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketLargeTraderReportsResponse {
    type Item = crate::models::ListVaultMarketLargeTraderReportsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketLendingStatementsResponse {
    type Item = crate::models::ListVaultMarketLendingStatementsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketLoansResponse {
    type Item = crate::models::ListVaultMarketLoansItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketLocateMineResponse {
    type Item = crate::models::ListVaultMarketLocateMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketMidpointResponse {
    type Item = crate::models::ListVaultMarketMidpointItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketMoversResponse {
    type Item = crate::models::ListVaultMarketMoversItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketNavHistoryResponse {
    type Item = crate::models::ListVaultMarketNavHistoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOffersResponse {
    type Item = crate::models::ListVaultMarketOffersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOpeningResponse {
    type Item = crate::models::ListVaultMarketOpeningItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOptionComboRfqResponse {
    type Item = crate::models::ListVaultMarketOptionComboRfqItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOptionCombosResponse {
    type Item = crate::models::ListVaultMarketOptionCombosItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOptionContractDailyResponse {
    type Item = crate::models::ListVaultMarketOptionContractDailyItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOptionMmQuotesResponse {
    type Item = crate::models::ListVaultMarketOptionMmQuotesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOptionNoticesResponse {
    type Item = crate::models::ListVaultMarketOptionNoticesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOptionOrdersResponse {
    type Item = crate::models::ListVaultMarketOptionOrdersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOptionPracticeFillsResponse {
    type Item = crate::models::ListVaultMarketOptionPracticeFillsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOptionPracticeOrdersResponse {
    type Item = crate::models::ListVaultMarketOptionPracticeOrdersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOptionRfqResponse {
    type Item = crate::models::ListVaultMarketOptionRfqItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOptionSeriesResponse {
    type Item = crate::models::ListVaultMarketOptionSeriesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOptionStrategiesResponse {
    type Item = crate::models::ListVaultMarketOptionStrategiesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketOracleHistoryResponse {
    type Item = crate::models::ListVaultMarketOracleHistoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketP2pLoansResponse {
    type Item = crate::models::ListVaultMarketP2pLoansItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketP2pOffersResponse {
    type Item = crate::models::ListVaultMarketP2pOffersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketP2pUnitsResponse {
    type Item = crate::models::ListVaultMarketP2pUnitsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPaperFillsResponse {
    type Item = crate::models::ListVaultMarketPaperFillsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPaperLeaderboardResponse {
    type Item = crate::models::ListVaultMarketPaperLeaderboardItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPaperMissionsResponse {
    type Item = crate::models::ListVaultMarketPaperMissionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPaperOrdersResponse {
    type Item = crate::models::ListVaultMarketPaperOrdersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPartnerAccrualsResponse {
    type Item = crate::models::ListVaultMarketPartnerAccrualsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPartnerFinderResponse {
    type Item = crate::models::ListVaultMarketPartnerFinderItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPoolStakesResponse {
    type Item = crate::models::ListVaultMarketPoolStakesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPortfolioMarginHistoryResponse {
    type Item = crate::models::ListVaultMarketPortfolioMarginHistoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPositionUnitsResponse {
    type Item = crate::models::ListVaultMarketPositionUnitsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPositionsResponse {
    type Item = crate::models::ListVaultMarketPositionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPositionsRiskResponse {
    type Item = crate::models::ListVaultMarketPositionsRiskItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionAlertsResponse {
    type Item = crate::models::ListVaultMarketPredictionAlertsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionComboRfqsResponse {
    type Item = crate::models::ListVaultMarketPredictionComboRfqsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionCombosResponse {
    type Item = crate::models::ListVaultMarketPredictionCombosItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionCommentsResponse {
    type Item = crate::models::ListVaultMarketPredictionCommentsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionConversionsResponse {
    type Item = crate::models::ListVaultMarketPredictionConversionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionFollowingResponse {
    type Item = crate::models::ListVaultMarketPredictionFollowingItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionForecastsResponse {
    type Item = crate::models::ListVaultMarketPredictionForecastsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionMakersResponse {
    type Item = crate::models::ListVaultMarketPredictionMakersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionMarketsResponse {
    type Item = crate::models::ListVaultMarketPredictionMarketsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionOrdersResponse {
    type Item = crate::models::ListVaultMarketPredictionOrdersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionPositionsResponse {
    type Item = crate::models::ListVaultMarketPredictionPositionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionProposalListedResponse {
    type Item = crate::models::ListVaultMarketPredictionProposalListedItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPredictionProposalsResponse {
    type Item = crate::models::ListVaultMarketPredictionProposalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketPriceWidgetsResponse {
    type Item = crate::models::ListVaultMarketPriceWidgetsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketProCandlesResponse {
    type Item = crate::models::ListVaultMarketProCandlesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketProEventsResponse {
    type Item = crate::models::ListVaultMarketProEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketProLinksResponse {
    type Item = crate::models::ListVaultMarketProLinksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketProOrdersResponse {
    type Item = crate::models::ListVaultMarketProOrdersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketProQuotesResponse {
    type Item = crate::models::ListVaultMarketProQuotesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketProRewardsResponse {
    type Item = crate::models::ListVaultMarketProRewardsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketProTapeResponse {
    type Item = crate::models::ListVaultMarketProTapeItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketProofOfReserveLeavesResponse {
    type Item = crate::models::ListVaultMarketProofOfReserveLeavesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketRecurringResponse {
    type Item = crate::models::ListVaultMarketRecurringItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketRecurringRunsResponse {
    type Item = crate::models::ListVaultMarketRecurringRunsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketRfqInboxResponse {
    type Item = crate::models::ListVaultMarketRfqInboxItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketRfqResponse {
    type Item = crate::models::ListVaultMarketRfqItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketSetGoalBasketsResponse {
    type Item = crate::models::ListVaultMarketSetGoalBasketsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketSetGoalSetsResponse {
    type Item = crate::models::ListVaultMarketSetGoalSetsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketSetGoalsResponse {
    type Item = crate::models::ListVaultMarketSetGoalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareAutoInvestResponse {
    type Item = crate::models::ListVaultMarketShareAutoInvestItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareAutoInvestRunsResponse {
    type Item = crate::models::ListVaultMarketShareAutoInvestRunsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareBasketsResponse {
    type Item = crate::models::ListVaultMarketShareBasketsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareCardsResponse {
    type Item = crate::models::ListVaultMarketShareCardsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareDropsResponse {
    type Item = crate::models::ListVaultMarketShareDropsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareGiftsResponse {
    type Item = crate::models::ListVaultMarketShareGiftsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareOrdersResponse {
    type Item = crate::models::ListVaultMarketShareOrdersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareSeriesResponse {
    type Item = crate::models::ListVaultMarketShareSeriesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareSeryAppraisalsResponse {
    type Item = crate::models::ListVaultMarketShareSeryAppraisalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareSeryTapeResponse {
    type Item = crate::models::ListVaultMarketShareSeryTapeItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareSeryUpdatesResponse {
    type Item = crate::models::ListVaultMarketShareSeryUpdatesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareSubscriptionsResponse {
    type Item = crate::models::ListVaultMarketShareSubscriptionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketShareWorldResponse {
    type Item = crate::models::ListVaultMarketShareWorldItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketSupplyDeclaredValuesResponse {
    type Item = crate::models::ListVaultMarketSupplyDeclaredValuesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketSupplyDepositsResponse {
    type Item = crate::models::ListVaultMarketSupplyDepositsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketSupplyEligibilityResponse {
    type Item = crate::models::ListVaultMarketSupplyEligibilityItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketSupplyInvitesResponse {
    type Item = crate::models::ListVaultMarketSupplyInvitesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketSupplyShipmentsResponse {
    type Item = crate::models::ListVaultMarketSupplyShipmentsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketValuationStatementsResponse {
    type Item = crate::models::ListVaultMarketValuationStatementsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketVaultSuggestionsResponse {
    type Item = crate::models::ListVaultMarketVaultSuggestionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketWatchlistResponse {
    type Item = crate::models::ListVaultMarketWatchlistItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketWidgetTokensResponse {
    type Item = crate::models::ListVaultMarketWidgetTokensItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketWorldCohostsResponse {
    type Item = crate::models::ListVaultMarketWorldCohostsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketWorldCreatorRoomsResponse {
    type Item = crate::models::ListVaultMarketWorldCreatorRoomsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultMarketWorldMarketOpenResponse {
    type Item = crate::models::ListVaultMarketWorldMarketOpenItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultReceiptTokenWalletsResponse {
    type Item = crate::models::ListVaultReceiptTokenWalletsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultReceiptTokensResponse {
    type Item = crate::models::ListVaultReceiptTokensItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultReceiptsResponse {
    type Item = crate::models::ListVaultReceiptsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVaultUnitListResponse {
    type Item = crate::models::ListVaultUnitListItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListVideoRequestsResponse {
    type Item = crate::models::ListVideoRequestsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::CreateWeb3CrosslistResponse {
    type Item = crate::models::CreateWeb3CrosslistItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3ActivityResponse {
    type Item = crate::models::ListWeb3ActivityItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3AlertsResponse {
    type Item = crate::models::ListWeb3AlertsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3ApprovalsResponse {
    type Item = crate::models::ListWeb3ApprovalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3AuctionsResponse {
    type Item = crate::models::ListWeb3AuctionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3BrowseCollectionsResponse {
    type Item = crate::models::ListWeb3BrowseCollectionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3BrowseTokensResponse {
    type Item = crate::models::ListWeb3BrowseTokensItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3BundlesResponse {
    type Item = crate::models::ListWeb3BundlesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3CollectionHistoryResponse {
    type Item = crate::models::ListWeb3CollectionHistoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3CollectionsResponse {
    type Item = crate::models::ListWeb3CollectionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3CreatorCollectionsResponse {
    type Item = crate::models::ListWeb3CreatorCollectionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3DropsResponse {
    type Item = crate::models::ListWeb3DropsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3HolderPerksResponse {
    type Item = crate::models::ListWeb3HolderPerksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3LaunchPerksResponse {
    type Item = crate::models::ListWeb3LaunchPerksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3LaunchesResponse {
    type Item = crate::models::ListWeb3LaunchesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3ListingsResponse {
    type Item = crate::models::ListWeb3ListingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3MarketplaceKeysResponse {
    type Item = crate::models::ListWeb3MarketplaceKeysItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3NftsResponse {
    type Item = crate::models::ListWeb3NftsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3OrdersResponse {
    type Item = crate::models::ListWeb3OrdersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3PurchasesResponse {
    type Item = crate::models::ListWeb3PurchasesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3RankingsResponse {
    type Item = crate::models::ListWeb3RankingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3SalesResponse {
    type Item = crate::models::ListWeb3SalesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3SearchResponse {
    type Item = crate::models::ListWeb3SearchItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3SearchTokensResponse {
    type Item = crate::models::ListWeb3SearchTokensItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3StoreWalletBalancesResponse {
    type Item = crate::models::ListWeb3StoreWalletBalancesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3StoreWalletsResponse {
    type Item = crate::models::ListWeb3StoreWalletsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3WalletsResponse {
    type Item = crate::models::ListWeb3WalletsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWeb3WatchlistResponse {
    type Item = crate::models::ListWeb3WatchlistItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWebhookDeliveriesResponse {
    type Item = crate::models::ListWebhookDeliveriesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWebhooksResponse {
    type Item = crate::models::ListWebhooksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWholesaleBuyersResponse {
    type Item = crate::models::ListWholesaleBuyersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWholesaleCatalogItemsResponse {
    type Item = crate::models::ListWholesaleCatalogItemsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWholesaleCatalogsResponse {
    type Item = crate::models::ListWholesaleCatalogsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWholesaleDropsResponse {
    type Item = crate::models::ListWholesaleDropsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWholesaleFreightRequestsResponse {
    type Item = crate::models::ListWholesaleFreightRequestsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWholesaleGroupsResponse {
    type Item = crate::models::ListWholesaleGroupsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWholesaleInvoicesResponse {
    type Item = crate::models::ListWholesaleInvoicesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWholesaleOrdersResponse {
    type Item = crate::models::ListWholesaleOrdersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWholesaleQuotesResponse {
    type Item = crate::models::ListWholesaleQuotesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWholesaleRequestsResponse {
    type Item = crate::models::ListWholesaleRequestsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWholesaleSetupStepsResponse {
    type Item = crate::models::ListWholesaleSetupStepsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::UpdateWholesaleListingTierResponse {
    type Item = crate::models::UpdateWholesaleListingTierItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::GetWorkflowRunNodeResponse {
    type Item = crate::models::GetWorkflowRunNodeItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorkflowApprovalsResponse {
    type Item = crate::models::ListWorkflowApprovalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorkflowChainsResponse {
    type Item = crate::models::ListWorkflowChainsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorkflowRunsResponse {
    type Item = crate::models::ListWorkflowRunsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorkflowSaleActiveResponse {
    type Item = crate::models::ListWorkflowSaleActiveItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorkflowTemplatesResponse {
    type Item = crate::models::ListWorkflowTemplatesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorkflowVersionsResponse {
    type Item = crate::models::ListWorkflowVersionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorkflowsResponse {
    type Item = crate::models::ListWorkflowsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::CreateWorldPartyWishlistResponse {
    type Item = crate::models::CreateWorldPartyWishlistItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::GetWorldDmResponse {
    type Item = crate::models::GetWorldDmItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::GetWorldExperienceMemoryMapResponse {
    type Item = crate::models::GetWorldExperienceMemoryMapItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::GetWorldExperienceOrderedResponse {
    type Item = crate::models::GetWorldExperienceOrderedItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldAppealsResponse {
    type Item = crate::models::ListWorldAppealsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldAttendantUsageResponse {
    type Item = crate::models::ListWorldAttendantUsageItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldAvatarClosetResponse {
    type Item = crate::models::ListWorldAvatarClosetItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldAvatarStockResponse {
    type Item = crate::models::ListWorldAvatarStockItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldAvatarWearablesResponse {
    type Item = crate::models::ListWorldAvatarWearablesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldBestDressedResponse {
    type Item = crate::models::ListWorldBestDressedItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldBlocksResponse {
    type Item = crate::models::ListWorldBlocksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldCapabilityDeclarationsResponse {
    type Item = crate::models::ListWorldCapabilityDeclarationsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldCollaborationsResponse {
    type Item = crate::models::ListWorldCollaborationsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldContestStandingsResponse {
    type Item = crate::models::ListWorldContestStandingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldContestsResponse {
    type Item = crate::models::ListWorldContestsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldConventionExhibitorsResponse {
    type Item = crate::models::ListWorldConventionExhibitorsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldConventionMessagesResponse {
    type Item = crate::models::ListWorldConventionMessagesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldConventionMineResponse {
    type Item = crate::models::ListWorldConventionMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldConventionsResponse {
    type Item = crate::models::ListWorldConventionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldCosmDesignsResponse {
    type Item = crate::models::ListWorldCosmDesignsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldCosmInventoryResponse {
    type Item = crate::models::ListWorldCosmInventoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldDecorCatalogueResponse {
    type Item = crate::models::ListWorldDecorCatalogueItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldDecorDesignsResponse {
    type Item = ::std::collections::HashMap<String, serde_json::Value>;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldDecorInventoryResponse {
    type Item = crate::models::ListWorldDecorInventoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldDecorMarketResponse {
    type Item = crate::models::ListWorldDecorMarketItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldDecorMineResponse {
    type Item = crate::models::ListWorldDecorMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldDecorStreetsResponse {
    type Item = crate::models::ListWorldDecorStreetsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldDmsResponse {
    type Item = crate::models::ListWorldDmsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldEarningAffiliateResponse {
    type Item = crate::models::ListWorldEarningAffiliateItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldEarningLedgerResponse {
    type Item = crate::models::ListWorldEarningLedgerItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldEarningPayoutsResponse {
    type Item = crate::models::ListWorldEarningPayoutsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceActivityResponse {
    type Item = crate::models::ListWorldExperienceActivityItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceAlertsResponse {
    type Item = crate::models::ListWorldExperienceAlertsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceAssetsResponse {
    type Item = crate::models::ListWorldExperienceAssetsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceAuditResponse {
    type Item = crate::models::ListWorldExperienceAuditItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceBansResponse {
    type Item = crate::models::ListWorldExperienceBansItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceConfigVersionsResponse {
    type Item = crate::models::ListWorldExperienceConfigVersionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceDatastoreEntriesResponse {
    type Item = crate::models::ListWorldExperienceDatastoreEntriesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceDatastoreEntryVersionsResponse {
    type Item = crate::models::ListWorldExperienceDatastoreEntryVersionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceDatastoresResponse {
    type Item = crate::models::ListWorldExperienceDatastoresItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceDropTablesResponse {
    type Item = crate::models::ListWorldExperienceDropTablesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceEventUpcomingResponse {
    type Item = crate::models::ListWorldExperienceEventUpcomingItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceEventsResponse {
    type Item = crate::models::ListWorldExperienceEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceExperienceEventsResponse {
    type Item = crate::models::ListWorldExperienceExperienceEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceExperimentsResponse {
    type Item = crate::models::ListWorldExperienceExperimentsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceModerationReportsResponse {
    type Item = crate::models::ListWorldExperienceModerationReportsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceNotificationTemplatesResponse {
    type Item = crate::models::ListWorldExperienceNotificationTemplatesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperiencePlaceCollabScriptsResponse {
    type Item = crate::models::ListWorldExperiencePlaceCollabScriptsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperiencePlacePackagesResponse {
    type Item = crate::models::ListWorldExperiencePlacePackagesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperiencePlacePublishesResponse {
    type Item = crate::models::ListWorldExperiencePlacePublishesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperiencePlaceVersionsResponse {
    type Item = crate::models::ListWorldExperiencePlaceVersionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperiencePlacesResponse {
    type Item = crate::models::ListWorldExperiencePlacesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperiencePlaytestersResponse {
    type Item = crate::models::ListWorldExperiencePlaytestersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceProductsResponse {
    type Item = crate::models::ListWorldExperienceProductsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceReceiptsResponse {
    type Item = crate::models::ListWorldExperienceReceiptsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceRewardGrantsResponse {
    type Item = crate::models::ListWorldExperienceRewardGrantsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceRewardLedgerResponse {
    type Item = crate::models::ListWorldExperienceRewardLedgerItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceRewardTopupsResponse {
    type Item = crate::models::ListWorldExperienceRewardTopupsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperienceServersResponse {
    type Item = crate::models::ListWorldExperienceServersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldExperiencesResponse {
    type Item = crate::models::ListWorldExperiencesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldFollowingRoomsResponse {
    type Item = crate::models::ListWorldFollowingRoomsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldFollowsResponse {
    type Item = crate::models::ListWorldFollowsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldFriendsResponse {
    type Item = crate::models::ListWorldFriendsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldGameMineResponse {
    type Item = crate::models::ListWorldGameMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldGamePackageReceiptsResponse {
    type Item = crate::models::ListWorldGamePackageReceiptsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldGamePackagesResponse {
    type Item = crate::models::ListWorldGamePackagesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldGameRewardsResponse {
    type Item = crate::models::ListWorldGameRewardsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldGamesResponse {
    type Item = crate::models::ListWorldGamesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldHomeBuildingResponse {
    type Item = crate::models::ListWorldHomeBuildingItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldInteractiveCatalogueResponse {
    type Item = crate::models::ListWorldInteractiveCatalogueItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldInteractiveDesignsResponse {
    type Item = crate::models::ListWorldInteractiveDesignsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldItemSearchResponse {
    type Item = crate::models::ListWorldItemSearchItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldJobBoardResponse {
    type Item = crate::models::ListWorldJobBoardItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldMarketResponse {
    type Item = crate::models::ListWorldMarketItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldMarketStreetSignsResponse {
    type Item = crate::models::ListWorldMarketStreetSignsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldMeExperienceEventsResponse {
    type Item = crate::models::ListWorldMeExperienceEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldMeNotificationOptInsResponse {
    type Item = crate::models::ListWorldMeNotificationOptInsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldMeSubscriptionsResponse {
    type Item = crate::models::ListWorldMeSubscriptionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldOutfitsResponse {
    type Item = crate::models::ListWorldOutfitsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldPackagesResponse {
    type Item = crate::models::ListWorldPackagesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldParentalChildrenResponse {
    type Item = crate::models::ListWorldParentalChildrenItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldPlaceBansResponse {
    type Item = crate::models::ListWorldPlaceBansItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldPlaceBundlesResponse {
    type Item = crate::models::ListWorldPlaceBundlesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldPlaceCollectionsResponse {
    type Item = crate::models::ListWorldPlaceCollectionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldPlaceOffersResponse {
    type Item = crate::models::ListWorldPlaceOffersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldPlaceRadarResponse {
    type Item = crate::models::ListWorldPlaceRadarItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldPlaceTradeTablesResponse {
    type Item = crate::models::ListWorldPlaceTradeTablesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldPlaceTradesResponse {
    type Item = crate::models::ListWorldPlaceTradesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldPromotedResponse {
    type Item = crate::models::ListWorldPromotedItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldPropertyPlotsResponse {
    type Item = crate::models::ListWorldPropertyPlotsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldPropertyRentalsResponse {
    type Item = crate::models::ListWorldPropertyRentalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldPropertyTrespassedResponse {
    type Item = crate::models::ListWorldPropertyTrespassedItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldProvenanceResponse {
    type Item = crate::models::ListWorldProvenanceItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldQuestsResponse {
    type Item = crate::models::ListWorldQuestsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldRecentResponse {
    type Item = crate::models::ListWorldRecentItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldRegionsResponse {
    type Item = crate::models::ListWorldRegionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldRoomWantsResponse {
    type Item = crate::models::ListWorldRoomWantsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldShopListingsResponse {
    type Item = crate::models::ListWorldShopListingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldSpaceRequestsResponse {
    type Item = crate::models::ListWorldSpaceRequestsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldSpaceWhatsOnResponse {
    type Item = crate::models::ListWorldSpaceWhatsOnItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldSpacesResponse {
    type Item = crate::models::ListWorldSpacesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldStaffApplicantsResponse {
    type Item = crate::models::ListWorldStaffApplicantsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldStaffPeopleResponse {
    type Item = crate::models::ListWorldStaffPeopleItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldStaffPlacesResponse {
    type Item = crate::models::ListWorldStaffPlacesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldStaffPostingsResponse {
    type Item = crate::models::ListWorldStaffPostingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldStaffShiftsResponse {
    type Item = crate::models::ListWorldStaffShiftsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldStreaksResponse {
    type Item = crate::models::ListWorldStreaksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldTradeMineResponse {
    type Item = crate::models::ListWorldTradeMineItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldTradeTableBinderResponse {
    type Item = crate::models::ListWorldTradeTableBinderItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldTradeTableListingsResponse {
    type Item = crate::models::ListWorldTradeTableListingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldTransferRequestsResponse {
    type Item = crate::models::ListWorldTransferRequestsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldTransparencyResponse {
    type Item = crate::models::ListWorldTransparencyItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListWorldVenuesResponse {
    type Item = crate::models::ListWorldVenuesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::UpdateWorldExperienceGalleryRoleResponse {
    type Item = crate::models::UpdateWorldExperienceGalleryRoleItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}
