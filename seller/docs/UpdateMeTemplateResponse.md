# UpdateMeTemplateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**user_id** | **String** |  | 
**scope** | **String** |  | 
**name** | **String** |  | 
**notes** | Option<**String**> | Optional short blurb the seller can attach to remember what it's for. | [optional]
**description** | Option<**String**> | Primary description body. For scope='description' this is the  snippet body; for scope='listing' this is the default description  the seller wants pre-filled. | [optional]
**description_variants** | Option<**Vec<String>**> | A/B variants for description. Populated for scope='listing';  typically null for scope='description' (a snippet is one string). | [optional]
**title** | Option<**String**> |  | [optional]
**title_variants** | Option<**Vec<String>**> |  | [optional]
**brand** | Option<**String**> |  | [optional]
**condition** | Option<**String**> | Master condition enum — new/like_new/good/fair/poor. | [optional]
**color** | Option<**String**> |  | [optional]
**material** | Option<**String**> |  | [optional]
**size** | Option<**String**> |  | [optional]
**size_system** | Option<**String**> |  | [optional]
**weight_oz** | Option<**f32**> |  | [optional]
**department** | Option<**String**> |  | [optional]
**gender** | Option<**String**> |  | [optional]
**style** | Option<**String**> |  | [optional]
**pattern** | Option<**String**> |  | [optional]
**item_type** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**default_for_category** | Option<**String**> | When set, form's category picker prompts \"Use your default for  this category\" on match. | [optional]
**is_default** | **bool** |  | 
**share_token** | Option<**String**> | URL-safe random token. Populated by POST /me/templates/:id/share;  the /public/templates/:token route surfaces a read-only view any  visitor can browse + import. | [optional]
**sort_order** | **f32** | Snippet ordering — kept for scope='description' back-compat with  the description_templates.sort_order behavior. | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


