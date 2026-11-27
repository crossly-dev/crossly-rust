# \WorkflowsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_workflow**](WorkflowsApi.md#create_workflow) | **POST** /v1/workflows | Create a workflow: from blank ({ name, graph? }), from a template ({ from: \"template\", templateId, setup?, activate? }) or from an export document ({ from: \"import\", document }). New workflows start paused; activate one with POST /v1/workflows/{id}/activate. Returns the workflow and its validation issues.
[**create_workflow_activate**](WorkflowsApi.md#create_workflow_activate) | **POST** /v1/workflows/{id}/activate | Activate a workflow: its triggers start firing. A graph with blocking issues is refused (422, details.issues).
[**create_workflow_approval_decide**](WorkflowsApi.md#create_workflow_approval_decide) | **POST** /v1/workflow-approvals/{id}/decide | Approve or reject a pending flow.approval request: the waiting run resumes down the approved or rejected port. 409 when it was already decided (another device, Telegram, or the timeout).
[**create_workflow_chain**](WorkflowsApi.md#create_workflow_chain) | **POST** /v1/workflow-chains | Create a multi-step workflow chain.
[**create_workflow_chain_run_now**](WorkflowsApi.md#create_workflow_chain_run_now) | **POST** /v1/workflow-chains/{id}/run-now | Enqueue an ad-hoc run of a workflow chain.
[**create_workflow_chain_toggle**](WorkflowsApi.md#create_workflow_chain_toggle) | **POST** /v1/workflow-chains/{id}/toggle | Flip a workflow chain between active and inactive.
[**create_workflow_deactivate**](WorkflowsApi.md#create_workflow_deactivate) | **POST** /v1/workflows/{id}/deactivate | Pause a workflow: its triggers stop firing. Runs already started finish.
[**create_workflow_duplicate**](WorkflowsApi.md#create_workflow_duplicate) | **POST** /v1/workflows/{id}/duplicate | Copy a workflow (graph, description, tags) into a new paused workflow.
[**create_workflow_expression_preview**](WorkflowsApi.md#create_workflow_expression_preview) | **POST** /v1/workflows/expressions/preview | Resolve one param value (\"{{ $json.priceCents * 0.9 }}\") with the engine's evaluator against an item, the trigger item, earlier nodes' items or a stored run. A bad expression is an answer (ok=false with the message), not an HTTP error. Changes nothing.
[**create_workflow_import**](WorkflowsApi.md#create_workflow_import) | **POST** /v1/workflows/import | Import a crossly.workflow export document (or { document, name }) as a new, paused workflow. Integration and workflow references are cleared (they belong to the exporting account) and listed in stripped.
[**create_workflow_run**](WorkflowsApi.md#create_workflow_run) | **POST** /v1/workflows/{id}/run | Run a workflow (active or paused) live now, on items you pass or on selected listings / orders / inventory items (one kind per call, up to 1,000). The graph must validate (422 with issues otherwise).
[**create_workflow_run_cancel**](WorkflowsApi.md#create_workflow_run_cancel) | **POST** /v1/workflow-runs/{runId}/cancel | Cancel a queued, running or waiting run. 409 when it already finished.
[**create_workflow_run_on**](WorkflowsApi.md#create_workflow_run_on) | **POST** /v1/workflows/run-on | \"Run workflow on…\": the same as POST /v1/workflows/{id}/run with the workflow named in the body.
[**create_workflow_run_retry**](WorkflowsApi.md#create_workflow_run_retry) | **POST** /v1/workflow-runs/{runId}/retry | Retry a failed run from the node that failed: a new run (retryOfRunId) seeded from the failed run's checkpoint, so nodes that succeeded are not repeated. 409 unless the run failed.
[**create_workflow_sale_end**](WorkflowsApi.md#create_workflow_sale_end) | **POST** /v1/workflows/sales/{saleEventId}/end | End a sale now. A sale that has not started is cancelled; a live one closes and every discounted price is put back now (reverted = how many listings were restored). 404 for a sale that is not yours.
[**create_workflow_template_use**](WorkflowsApi.md#create_workflow_template_use) | **POST** /v1/workflows/templates/{id}/use | Create a workflow from a template, filling its setup fields; activate=true activates it when it validates.
[**create_workflow_test**](WorkflowsApi.md#create_workflow_test) | **POST** /v1/workflows/{id}/test | Test a workflow (the saved graph, or a draft graph you pass). mode=test runs it for real, using pinned data where a node has it; mode=dry is a safe test: write, destructive and money nodes report what they would do and change nothing. Trigger items: the ones you pass, else pinned, else the last real ones, else the trigger sample.
[**create_workflow_validate**](WorkflowsApi.md#create_workflow_validate) | **POST** /v1/workflows/validate | Validate a graph without saving it: unknown node types, missing required params, bad ports, cycles, expression errors. blocking=true means activation would be refused.
[**create_workflow_version_restore**](WorkflowsApi.md#create_workflow_version_restore) | **POST** /v1/workflows/{id}/versions/{v}/restore | Restore an earlier version's graph as a new version. Pass version (the current one you read) to refuse the restore when someone saved meanwhile (409). Body optional: { version?: integer }.
[**create_workflow_webhook_rotate**](WorkflowsApi.md#create_workflow_webhook_rotate) | **POST** /v1/workflows/{id}/webhooks/{nodeId}/rotate | Issue a new inbound URL for an on.webhook trigger node (the old one stops working). The URL, and the signing secret when withSecret=true, are returned ONCE: store them now.
[**delete_workflow**](WorkflowsApi.md#delete_workflow) | **DELETE** /v1/workflows/{id} | Delete (archive) a workflow: its triggers stop, open runs are cancelled, run history stays readable.
[**delete_workflow_chain**](WorkflowsApi.md#delete_workflow_chain) | **DELETE** /v1/workflow-chains/{id} | Delete a workflow chain (cascades steps + runs).
[**get_workflow**](WorkflowsApi.md#get_workflow) | **GET** /v1/workflows/{id} | Get one workflow with its full graph and current version.
[**get_workflow_catalog**](WorkflowsApi.md#get_workflow_catalog) | **GET** /v1/workflows/catalog | Everything needed to build a workflow: every node type (params, ports, output fields, risk, platforms capability), which platforms each capability covers and which of those you have connected, the expression helpers and bindings ({{ $json.listingId }}…) and the condition operators. Read this before composing a graph rather than guessing node types or param keys.
[**get_workflow_chain**](WorkflowsApi.md#get_workflow_chain) | **GET** /v1/workflow-chains/{id} | Get one workflow chain with its steps.
[**get_workflow_export**](WorkflowsApi.md#get_workflow_export) | **GET** /v1/workflows/{id}/export | Export a workflow as a portable document (format crossly.workflow) that POST /v1/workflows/import accepts. Credentials and account handles are stripped; pinned test data only with pinnedData=true.
[**get_workflow_run**](WorkflowsApi.md#get_workflow_run) | **GET** /v1/workflow-runs/{runId} | One run: its summary, the graph version it ran, and every node execution (status, item counts per port, timing, errors). Item data per node: GET …/nodes/{nodeId}.
[**get_workflow_run_node**](WorkflowsApi.md#get_workflow_run_node) | **GET** /v1/workflow-runs/{runId}/nodes/{nodeId} | Every execution of one node in a run with its input and output items (each port keeps the first 50 items, 16 KB per item; truncated=true when cut). Data older than the retention window, or not kept by settings.saveSuccessData=false, is pruned: that execution has input/output null and pruned=true, and when every execution is pruned the response is { data: null, meta: { pruned: true } } (never an empty list).
[**list_workflow_approvals**](WorkflowsApi.md#list_workflow_approvals) | **GET** /v1/workflow-approvals | Approval requests from flow.approval nodes, newest first (pending only unless status says otherwise). Decide one with POST /v1/workflow-approvals/{id}/decide.
[**list_workflow_chains**](WorkflowsApi.md#list_workflow_chains) | **GET** /v1/workflow-chains | List workflow chains with their step graph.
[**list_workflow_runs**](WorkflowsApi.md#list_workflow_runs) | **GET** /v1/workflows/{id}/runs | Run history of one workflow, newest first, with per-node status and item counts.
[**list_workflow_runs2**](WorkflowsApi.md#list_workflow_runs2) | **GET** /v1/workflow-runs | Runs of every workflow, newest first. Filter by workflow, status, mode and time.
[**list_workflow_sale_active**](WorkflowsApi.md#list_workflow_sale_active) | **GET** /v1/workflows/sales/active | Every live or upcoming sale (a sales event started by a sale.start node) with the workflow that runs it: percent off, window, and how many listings it applies to.
[**list_workflow_templates**](WorkflowsApi.md#list_workflow_templates) | **GET** /v1/workflows/templates | Ready-made workflows (weekend sale, stale-listing markdowns, offer responder, ship-by alerts…), each a complete graph plus the setup fields its \"use\" call asks for.
[**list_workflow_versions**](WorkflowsApi.md#list_workflow_versions) | **GET** /v1/workflows/{id}/versions | The saved versions of a workflow, newest first (restore one with POST …/versions/{v}/restore).
[**list_workflows**](WorkflowsApi.md#list_workflows) | **GET** /v1/workflows | List workflows, newest change first. A workflow is a graph of nodes: one or more triggers, then actions, lookups and flow nodes (if, switch, wait, approval…). Each row carries its trigger and node types and a 7-day run sparkline, not the graph (GET /v1/workflows/{id} has it).
[**update_workflow**](WorkflowsApi.md#update_workflow) | **PUT** /v1/workflows/{id} | Save a workflow. Pass the version you edited: a save over a newer version is 409 version_conflict (re-read and merge). Saving an active workflow validates it like activation does (422 with issues).
[**update_workflow_chain**](WorkflowsApi.md#update_workflow_chain) | **PUT** /v1/workflow-chains/{id} | Replace a workflow chain wholesale.



## create_workflow

> crate::models::CreateWorkflowResponse create_workflow(create_workflow_request)
Create a workflow: from blank ({ name, graph? }), from a template ({ from: \"template\", templateId, setup?, activate? }) or from an export document ({ from: \"import\", document }). New workflows start paused; activate one with POST /v1/workflows/{id}/activate. Returns the workflow and its validation issues.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_workflow_request** | **CreateWorkflowRequest** |  | [required] |

### Return type

**crate::models::CreateWorkflowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_activate

> crate::models::WorkflowResponse create_workflow_activate(id)
Activate a workflow: its triggers start firing. A graph with blocking issues is refused (422, details.issues).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |

### Return type

**crate::models::WorkflowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_approval_decide

> crate::models::CreateWorkflowApprovalDecideResponse create_workflow_approval_decide(id, create_workflow_approval_decide_request)
Approve or reject a pending flow.approval request: the waiting run resumes down the approved or rejected port. 409 when it was already decided (another device, Telegram, or the timeout).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Approval id from GET /v1/workflow-approvals. | [required] |
**create_workflow_approval_decide_request** | **CreateWorkflowApprovalDecideRequest** |  | [required] |

### Return type

**crate::models::CreateWorkflowApprovalDecideResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_chain

> crate::models::IdResponse create_workflow_chain()
Create a multi-step workflow chain.

Deprecated: workflow chains are workflows now. Use /v1/workflows.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::IdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_chain_run_now

> crate::models::CreateWorkflowChainRunNowResponse create_workflow_chain_run_now(id)
Enqueue an ad-hoc run of a workflow chain.

Deprecated: workflow chains are workflows now. Use /v1/workflows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorkflowChainRunNowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_chain_toggle

> crate::models::CreateWorkflowChainToggleResponse create_workflow_chain_toggle(id)
Flip a workflow chain between active and inactive.

Deprecated: workflow chains are workflows now. Use /v1/workflows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorkflowChainToggleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_deactivate

> crate::models::WorkflowResponse create_workflow_deactivate(id)
Pause a workflow: its triggers stop firing. Runs already started finish.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |

### Return type

**crate::models::WorkflowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_duplicate

> crate::models::CreateWorkflowDuplicateResponse create_workflow_duplicate(id, create_workflow_duplicate_request)
Copy a workflow (graph, description, tags) into a new paused workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |
**create_workflow_duplicate_request** | **CreateWorkflowDuplicateRequest** |  | [required] |

### Return type

**crate::models::CreateWorkflowDuplicateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_expression_preview

> crate::models::CreateWorkflowExpressionPreviewResponse create_workflow_expression_preview(create_workflow_expression_preview_request)
Resolve one param value (\"{{ $json.priceCents * 0.9 }}\") with the engine's evaluator against an item, the trigger item, earlier nodes' items or a stored run. A bad expression is an answer (ok=false with the message), not an HTTP error. Changes nothing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_workflow_expression_preview_request** | **CreateWorkflowExpressionPreviewRequest** |  | [required] |

### Return type

**crate::models::CreateWorkflowExpressionPreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_import

> crate::models::CreateWorkflowImportResponse create_workflow_import(request_body)
Import a crossly.workflow export document (or { document, name }) as a new, paused workflow. Integration and workflow references are cleared (they belong to the exporting account) and listed in stripped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | **::std::collections::HashMap<String, serde_json::Value>** |  | [required] |

### Return type

**crate::models::CreateWorkflowImportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_run

> crate::models::CreateWorkflowRunResponse create_workflow_run(id, create_workflow_run_request)
Run a workflow (active or paused) live now, on items you pass or on selected listings / orders / inventory items (one kind per call, up to 1,000). The graph must validate (422 with issues otherwise).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |
**create_workflow_run_request** | **CreateWorkflowRunRequest** |  | [required] |

### Return type

**crate::models::CreateWorkflowRunResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_run_cancel

> crate::models::CreateWorkflowRunCancelResponse create_workflow_run_cancel(run_id)
Cancel a queued, running or waiting run. 409 when it already finished.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** | Workflow run id. | [required] |

### Return type

**crate::models::CreateWorkflowRunCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_run_on

> crate::models::CreateWorkflowRunOnResponse create_workflow_run_on(create_workflow_run_on_request)
\"Run workflow on…\": the same as POST /v1/workflows/{id}/run with the workflow named in the body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_workflow_run_on_request** | **CreateWorkflowRunOnRequest** |  | [required] |

### Return type

**crate::models::CreateWorkflowRunOnResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_run_retry

> crate::models::CreateWorkflowRunRetryResponse create_workflow_run_retry(run_id)
Retry a failed run from the node that failed: a new run (retryOfRunId) seeded from the failed run's checkpoint, so nodes that succeeded are not repeated. 409 unless the run failed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** | Workflow run id. | [required] |

### Return type

**crate::models::CreateWorkflowRunRetryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_sale_end

> crate::models::CreateWorkflowSaleEndResponse create_workflow_sale_end(sale_event_id)
End a sale now. A sale that has not started is cancelled; a live one closes and every discounted price is put back now (reverted = how many listings were restored). 404 for a sale that is not yours.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sale_event_id** | **String** | Sale id from GET /v1/workflows/sales/active. | [required] |

### Return type

**crate::models::CreateWorkflowSaleEndResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_template_use

> crate::models::CreateWorkflowResponse create_workflow_template_use(id, create_workflow_template_use_request)
Create a workflow from a template, filling its setup fields; activate=true activates it when it validates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Template id from GET /v1/workflows/templates. | [required] |
**create_workflow_template_use_request** | **CreateWorkflowTemplateUseRequest** |  | [required] |

### Return type

**crate::models::CreateWorkflowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_test

> crate::models::CreateWorkflowTestResponse create_workflow_test(id, create_workflow_test_request)
Test a workflow (the saved graph, or a draft graph you pass). mode=test runs it for real, using pinned data where a node has it; mode=dry is a safe test: write, destructive and money nodes report what they would do and change nothing. Trigger items: the ones you pass, else pinned, else the last real ones, else the trigger sample.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |
**create_workflow_test_request** | **CreateWorkflowTestRequest** |  | [required] |

### Return type

**crate::models::CreateWorkflowTestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_validate

> crate::models::CreateWorkflowValidateResponse create_workflow_validate(create_workflow_validate_request)
Validate a graph without saving it: unknown node types, missing required params, bad ports, cycles, expression errors. blocking=true means activation would be refused.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_workflow_validate_request** | **CreateWorkflowValidateRequest** |  | [required] |

### Return type

**crate::models::CreateWorkflowValidateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_version_restore

> crate::models::WorkflowResponse create_workflow_version_restore(id, v)
Restore an earlier version's graph as a new version. Pass version (the current one you read) to refuse the restore when someone saved meanwhile (409). Body optional: { version?: integer }.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |
**v** | **i32** | The version number to restore. | [required] |

### Return type

**crate::models::WorkflowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_workflow_webhook_rotate

> crate::models::CreateWorkflowWebhookRotateResponse create_workflow_webhook_rotate(id, node_id, create_workflow_webhook_rotate_request)
Issue a new inbound URL for an on.webhook trigger node (the old one stops working). The URL, and the signing secret when withSecret=true, are returned ONCE: store them now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |
**node_id** | **String** | The on.webhook trigger node id. | [required] |
**create_workflow_webhook_rotate_request** | **CreateWorkflowWebhookRotateRequest** |  | [required] |

### Return type

**crate::models::CreateWorkflowWebhookRotateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_workflow

> crate::models::DeleteWorkflowResponse delete_workflow(id)
Delete (archive) a workflow: its triggers stop, open runs are cancelled, run history stays readable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |

### Return type

**crate::models::DeleteWorkflowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_workflow_chain

> crate::models::OkResponse delete_workflow_chain(id)
Delete a workflow chain (cascades steps + runs).

Deprecated: workflow chains are workflows now. Use /v1/workflows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_workflow

> crate::models::GetWorkflowResponse get_workflow(id)
Get one workflow with its full graph and current version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |

### Return type

**crate::models::GetWorkflowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_workflow_catalog

> crate::models::GetWorkflowCatalogResponse get_workflow_catalog()
Everything needed to build a workflow: every node type (params, ports, output fields, risk, platforms capability), which platforms each capability covers and which of those you have connected, the expression helpers and bindings ({{ $json.listingId }}…) and the condition operators. Read this before composing a graph rather than guessing node types or param keys.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorkflowCatalogResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_workflow_chain

> crate::models::GetWorkflowChainResponse get_workflow_chain(id)
Get one workflow chain with its steps.

Deprecated: workflow chains are workflows now. Use /v1/workflows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetWorkflowChainResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_workflow_export

> crate::models::GetWorkflowExportResponse get_workflow_export(id, pinned_data)
Export a workflow as a portable document (format crossly.workflow) that POST /v1/workflows/import accepts. Credentials and account handles are stripped; pinned test data only with pinnedData=true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |
**pinned_data** | Option<**String**> | Keep the nodes' pinned test data. |  |

### Return type

**crate::models::GetWorkflowExportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_workflow_run

> crate::models::GetWorkflowRunResponse get_workflow_run(run_id)
One run: its summary, the graph version it ran, and every node execution (status, item counts per port, timing, errors). Item data per node: GET …/nodes/{nodeId}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** | Workflow run id. | [required] |

### Return type

**crate::models::GetWorkflowRunResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_workflow_run_node

> crate::models::GetWorkflowRunNodeResponse get_workflow_run_node(run_id, node_id)
Every execution of one node in a run with its input and output items (each port keeps the first 50 items, 16 KB per item; truncated=true when cut). Data older than the retention window, or not kept by settings.saveSuccessData=false, is pruned: that execution has input/output null and pruned=true, and when every execution is pruned the response is { data: null, meta: { pruned: true } } (never an empty list).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** | Workflow run id. | [required] |
**node_id** | **String** | Node id within the run's graph. | [required] |

### Return type

**crate::models::GetWorkflowRunNodeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_workflow_approvals

> crate::models::ListWorkflowApprovalsResponse list_workflow_approvals(status, workflow_id, page, limit)
Approval requests from flow.approval nodes, newest first (pending only unless status says otherwise). Decide one with POST /v1/workflow-approvals/{id}/decide.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Comma-separated: pending, approved, rejected, expired. Default pending. |  |
**workflow_id** | Option<**String**> | Only approvals of this workflow. |  |
**page** | Option<**i32**> | 1-indexed page. |  |[default to 1]
**limit** | Option<**i32**> | Rows per page (1-100). |  |[default to 25]

### Return type

**crate::models::ListWorkflowApprovalsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_workflow_chains

> crate::models::ListWorkflowChainsResponse list_workflow_chains()
List workflow chains with their step graph.

Deprecated: workflow chains are workflows now. Use /v1/workflows.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorkflowChainsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_workflow_runs

> crate::models::ListWorkflowRunsResponse list_workflow_runs(id, status, mode, since, until, page, limit)
Run history of one workflow, newest first, with per-node status and item counts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |
**status** | Option<**String**> | Comma-separated run statuses: queued, running, waiting, succeeded, failed, cancelled. |  |
**mode** | Option<**String**> | live (triggers), test (editor test run) or dry (safe test: no writes). |  |
**since** | Option<**String**> | Runs queued at or after this time (ISO 8601). |  |
**until** | Option<**String**> | Runs queued at or before this time (ISO 8601). |  |
**page** | Option<**i32**> | 1-indexed page. |  |[default to 1]
**limit** | Option<**i32**> | Rows per page (1-100). |  |[default to 25]

### Return type

**crate::models::ListWorkflowRunsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_workflow_runs2

> crate::models::ListWorkflowRunsResponse list_workflow_runs2(workflow_id, status, mode, since, until, page, limit)
Runs of every workflow, newest first. Filter by workflow, status, mode and time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_id** | Option<**String**> | Only runs of this workflow. |  |
**status** | Option<**String**> | Comma-separated run statuses: queued, running, waiting, succeeded, failed, cancelled. |  |
**mode** | Option<**String**> | live (triggers), test (editor test run) or dry (safe test: no writes). |  |
**since** | Option<**String**> | Runs queued at or after this time (ISO 8601). |  |
**until** | Option<**String**> | Runs queued at or before this time (ISO 8601). |  |
**page** | Option<**i32**> | 1-indexed page. |  |[default to 1]
**limit** | Option<**i32**> | Rows per page (1-100). |  |[default to 25]

### Return type

**crate::models::ListWorkflowRunsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_workflow_sale_active

> crate::models::ListWorkflowSaleActiveResponse list_workflow_sale_active()
Every live or upcoming sale (a sales event started by a sale.start node) with the workflow that runs it: percent off, window, and how many listings it applies to.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorkflowSaleActiveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_workflow_templates

> crate::models::ListWorkflowTemplatesResponse list_workflow_templates(category, q)
Ready-made workflows (weekend sale, stale-listing markdowns, offer responder, ship-by alerts…), each a complete graph plus the setup fields its \"use\" call asks for.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | Option<**String**> | Exact template category, e.g. \"Sales & markdowns\". |  |
**q** | Option<**String**> | Matches name, description, category or tags. |  |

### Return type

**crate::models::ListWorkflowTemplatesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_workflow_versions

> crate::models::ListWorkflowVersionsResponse list_workflow_versions(id)
The saved versions of a workflow, newest first (restore one with POST …/versions/{v}/restore).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |

### Return type

**crate::models::ListWorkflowVersionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_workflows

> crate::models::ListWorkflowsResponse list_workflows(active, tag, trigger, q, legacy_ref)
List workflows, newest change first. A workflow is a graph of nodes: one or more triggers, then actions, lookups and flow nodes (if, switch, wait, approval…). Each row carries its trigger and node types and a 7-day run sparkline, not the graph (GET /v1/workflows/{id} has it).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**active** | Option<**String**> | Only active (true) or paused (false) workflows. |  |
**tag** | Option<**String**> | Only workflows carrying this tag. |  |
**trigger** | Option<**String**> | Only workflows started by this trigger node type, e.g. on.listing.sold. |  |
**q** | Option<**String**> | Name contains (case-insensitive). |  |
**legacy_ref** | Option<**String**> | The workflow an old automation became: rule:<id>, chain:<id> or sale_event:<id>. |  |

### Return type

**crate::models::ListWorkflowsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_workflow

> crate::models::WorkflowResponse update_workflow(id, update_workflow_request)
Save a workflow. Pass the version you edited: a save over a newer version is 409 version_conflict (re-read and merge). Saving an active workflow validates it like activation does (422 with issues).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Workflow id. | [required] |
**update_workflow_request** | **UpdateWorkflowRequest** |  | [required] |

### Return type

**crate::models::WorkflowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_workflow_chain

> crate::models::OkResponse update_workflow_chain(id)
Replace a workflow chain wholesale.

Deprecated: workflow chains are workflows now. Use /v1/workflows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

