# GetMarketProductResponseProduct

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**brand** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**created_at** | **String** |  | 
**status** | **String** |  | 
**retail_cents** | Option<**f32**> |  | [optional]
**category_slug** | **String** |  | 
**external_id** | Option<**String**> |  | [optional]
**image_url** | Option<**String**> |  | [optional]
**model** | Option<**String**> |  | [optional]
**colorway** | Option<**String**> |  | [optional]
**style_code** | Option<**String**> |  | [optional]
**release_date** | Option<**String**> |  | [optional]
**submitted_by_user_id** | Option<**String**> |  | [optional]
**approved_by_user_id** | Option<**String**> |  | [optional]
**approved_at** | Option<**String**> |  | [optional]
**rejection_reason** | Option<**String**> |  | [optional]
**proposed_variants** | [**Vec<crate::models::GetMarketProductResponseProductProposedVariants>**](GetMarketProductResponse_product_proposedVariants.md) |  | 
**verification** | [**serde_json::Value**](.md) |  | 
**ai_verdict** | Option<[**serde_json::Value**](.md)> |  | [optional]
**submission_source** | Option<**String**> |  | [optional]
**duplicate_of_sku_id** | Option<**String**> |  | [optional]
**preferred_provider_slug** | Option<**String**> |  | [optional]
**allow_pre_bid** | **bool** |  | 
**pre_bid_opens_at** | Option<**String**> |  | [optional]
**featured** | **bool** |  | 
**external_source** | Option<**String**> |  | [optional]
**external_synced_at** | Option<**String**> |  | [optional]
**graded_grade_keys** | **Vec<String>** |  | 
**lowest_ask_cents** | Option<**f32**> |  | [optional]
**highest_bid_cents** | Option<**f32**> |  | [optional]
**last_sale_cents** | Option<**f32**> |  | [optional]
**trades_count** | **f32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


