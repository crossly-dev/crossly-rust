// Written by codegen/integrations/extras/rust/finish.mjs: every { data, pagination } response model.
use crate::client::Paged;

impl Paged for crate::models::ListBuyerActivityResponse {
    type Item = crate::models::ListBuyerActivityItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerAgentSessionsResponse {
    type Item = crate::models::ListBuyerAgentSessionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerAuctionBidsResponse {
    type Item = crate::models::ListBuyerAuctionBidsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerAuctionsResponse {
    type Item = crate::models::ListBuyerAuctionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerCartResponse {
    type Item = crate::models::ListBuyerCartItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerCasesResponse {
    type Item = crate::models::ListBuyerCasesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerCatalogBrowseResponse {
    type Item = crate::models::ListBuyerCatalogBrowseItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerCatalogListingSoldHistoryResponse {
    type Item = crate::models::ListBuyerCatalogListingSoldHistoryItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerCatalogListingVideosResponse {
    type Item = crate::models::ListBuyerCatalogListingVideosItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerCatalogSearchResponse {
    type Item = crate::models::ListBuyerCatalogSearchItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerCatalogSellersResponse {
    type Item = crate::models::ListBuyerCatalogSellersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::BuyerCartResponse {
    type Item = crate::models::ListBuyerCartSavedItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::BuyerResponse3 {
    type Item = ::std::collections::HashMap<String, serde_json::Value>;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerBoardsResponse {
    type Item = crate::models::ListBuyerBoardsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerCashbackResponse {
    type Item = crate::models::ListBuyerCashbackItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerClubsResponse {
    type Item = crate::models::ListBuyerClubsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerFollowingSellersResponse {
    type Item = crate::models::ListBuyerFollowingSellersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerHotspotsResponse {
    type Item = crate::models::ListBuyerHotspotsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerHuntsResponse {
    type Item = crate::models::ListBuyerHuntsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerLayawaysResponse {
    type Item = crate::models::ListBuyerLayawaysItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerLoyaltyResponse {
    type Item = crate::models::ListBuyerLoyaltyItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerMonitorMatchesResponse {
    type Item = crate::models::ListBuyerMonitorMatchesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerMonitorsResponse {
    type Item = crate::models::ListBuyerMonitorsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerPartiesResponse {
    type Item = crate::models::ListBuyerPartiesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerPaymentMethodsResponse {
    type Item = crate::models::ListBuyerPaymentMethodsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerProtectionPlansResponse {
    type Item = crate::models::ListBuyerProtectionPlansItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerRegistriesResponse {
    type Item = crate::models::ListBuyerRegistriesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerSizeMemoryPromptsResponse {
    type Item = crate::models::ListBuyerSizeMemoryPromptsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerSizeMemorySizesResponse {
    type Item = crate::models::ListBuyerSizeMemorySizesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerStoreListingsResponse {
    type Item = crate::models::ListBuyerStoreListingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerSubscriptionsResponse {
    type Item = crate::models::ListBuyerSubscriptionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerTryOnsResponse {
    type Item = crate::models::ListBuyerTryOnsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVideoRequestsResponse {
    type Item = crate::models::ListBuyerVideoRequestsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerWalletPerksResponse {
    type Item = crate::models::ListBuyerWalletPerksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerCreationReviewableResponse {
    type Item = crate::models::ListBuyerCreationReviewableItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerEventsResponse {
    type Item = crate::models::ListBuyerEventsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::CreateBuyerFeedbackResponse {
    type Item = crate::models::CreateBuyerFeedbackItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerFeedbackPendingResponse {
    type Item = crate::models::ListBuyerFeedbackPendingItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerFeedbackResponse {
    type Item = crate::models::ListBuyerFeedbackItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerGiftCardCreditResponse {
    type Item = crate::models::ListBuyerGiftCardCreditItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerGiftCardsResponse {
    type Item = crate::models::ListBuyerGiftCardsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerGiftFinderResponse {
    type Item = crate::models::ListBuyerGiftFinderItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerGiftSentResponse {
    type Item = crate::models::ListBuyerGiftSentItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerHomeBuildersResponse {
    type Item = crate::models::ListBuyerHomeBuildersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerHomeBuildingResponse {
    type Item = crate::models::ListBuyerHomeBuildingItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerListingQuickQuestionsResponse {
    type Item = crate::models::ListBuyerListingQuickQuestionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerListingSellerReviewsResponse {
    type Item = crate::models::ListBuyerListingSellerReviewsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerLiveShipmentsResponse {
    type Item = crate::models::ListBuyerLiveShipmentsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerLiveStreamLineupResponse {
    type Item = crate::models::ListBuyerLiveStreamLineupItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerOffersResponse {
    type Item = crate::models::ListBuyerOffersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerOrdersResponse {
    type Item = crate::models::ListBuyerOrdersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerPackOpeningsResponse {
    type Item = crate::models::ListBuyerPackOpeningsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerPacksResponse {
    type Item = crate::models::ListBuyerPacksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerRegistryGiftsResponse {
    type Item = crate::models::ListBuyerRegistryGiftsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerRegistrySearchResponse {
    type Item = crate::models::ListBuyerRegistrySearchItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerRentalsResponse {
    type Item = crate::models::ListBuyerRentalsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerScanSessionsResponse {
    type Item = crate::models::ListBuyerScanSessionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerTogetherResponse {
    type Item = crate::models::ListBuyerTogetherItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerTotalRequestsResponse {
    type Item = crate::models::ListBuyerTotalRequestsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketHoldingsResponse {
    type Item = crate::models::ListBuyerVaultMarketHoldingsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketIndexesResponse {
    type Item = crate::models::ListBuyerVaultMarketIndexesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketInstantExitResponse {
    type Item = crate::models::ListBuyerVaultMarketInstantExitItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketOptionCombosResponse {
    type Item = crate::models::ListBuyerVaultMarketOptionCombosItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketOptionNoticesResponse {
    type Item = crate::models::ListBuyerVaultMarketOptionNoticesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketOptionSeriesResponse {
    type Item = crate::models::ListBuyerVaultMarketOptionSeriesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketOptionStrategiesResponse {
    type Item = crate::models::ListBuyerVaultMarketOptionStrategiesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketPredictionCommentsResponse {
    type Item = crate::models::ListBuyerVaultMarketPredictionCommentsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketPredictionMarketsResponse {
    type Item = crate::models::ListBuyerVaultMarketPredictionMarketsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketPredictionPositionsResponse {
    type Item = crate::models::ListBuyerVaultMarketPredictionPositionsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketShareAutoInvestResponse {
    type Item = crate::models::ListBuyerVaultMarketShareAutoInvestItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketShareBasketsResponse {
    type Item = crate::models::ListBuyerVaultMarketShareBasketsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketShareDropsResponse {
    type Item = crate::models::ListBuyerVaultMarketShareDropsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketShareGiftsResponse {
    type Item = crate::models::ListBuyerVaultMarketShareGiftsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketShareSeriesResponse {
    type Item = crate::models::ListBuyerVaultMarketShareSeriesItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultMarketWatchlistResponse {
    type Item = crate::models::ListBuyerVaultMarketWatchlistItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerVaultResponse {
    type Item = crate::models::ListBuyerVaultItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerWholesaleDropsResponse {
    type Item = crate::models::ListBuyerWholesaleDropsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerWholesalePrebooksResponse {
    type Item = crate::models::ListBuyerWholesalePrebooksItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerWishlistItemsResponse {
    type Item = crate::models::ListBuyerWishlistItemsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerWishlistMembersResponse {
    type Item = crate::models::ListBuyerWishlistMembersItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}

impl Paged for crate::models::ListBuyerWishlistsResponse {
    type Item = crate::models::ListBuyerWishlistsItem;
    fn into_page(self) -> (Vec<Self::Item>, Option<crate::models::Pagination>) {
        (self.data, self.pagination.map(|p| *p))
    }
}
