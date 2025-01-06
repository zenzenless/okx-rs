# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v5_rfq_cancel_all_quotes_post**](DefaultApi.md#api_v5_rfq_cancel_all_quotes_post) | **POST** /api/v5/rfq/cancel-all-quotes | 取消所有报价
[**api_v5_rfq_cancel_all_rfqs_post**](DefaultApi.md#api_v5_rfq_cancel_all_rfqs_post) | **POST** /api/v5/rfq/cancel-all-rfqs | 取消所有询价
[**api_v5_rfq_cancel_batch_quotes_post**](DefaultApi.md#api_v5_rfq_cancel_batch_quotes_post) | **POST** /api/v5/rfq/cancel-batch-quotes | 批量取消报价
[**api_v5_rfq_cancel_batch_rfqs_post**](DefaultApi.md#api_v5_rfq_cancel_batch_rfqs_post) | **POST** /api/v5/rfq/cancel-batch-rfqs | 批量取消询价
[**api_v5_rfq_cancel_quote_post**](DefaultApi.md#api_v5_rfq_cancel_quote_post) | **POST** /api/v5/rfq/cancel-quote | 取消报价
[**api_v5_rfq_cancel_rfq_post**](DefaultApi.md#api_v5_rfq_cancel_rfq_post) | **POST** /api/v5/rfq/cancel-rfq | 取消询价
[**api_v5_rfq_counterparties_get**](DefaultApi.md#api_v5_rfq_counterparties_get) | **GET** /api/v5/rfq/counterparties | 获取报价方信息
[**api_v5_rfq_create_quote_post**](DefaultApi.md#api_v5_rfq_create_quote_post) | **POST** /api/v5/rfq/create-quote | 报价
[**api_v5_rfq_create_rfq_post**](DefaultApi.md#api_v5_rfq_create_rfq_post) | **POST** /api/v5/rfq/create-rfq | 询价
[**api_v5_rfq_execute_quote_post**](DefaultApi.md#api_v5_rfq_execute_quote_post) | **POST** /api/v5/rfq/execute-quote | 执行报价
[**api_v5_rfq_maker_instrument_settings_post**](DefaultApi.md#api_v5_rfq_maker_instrument_settings_post) | **POST** /api/v5/rfq/maker-instrument-settings | 设置可报价产品
[**api_v5_rfq_mmp_reset_post**](DefaultApi.md#api_v5_rfq_mmp_reset_post) | **POST** /api/v5/rfq/mmp-reset | 重设MMP状态
[**api_v5_rfq_public_trades_get**](DefaultApi.md#api_v5_rfq_public_trades_get) | **GET** /api/v5/rfq/public-trades | 获取大宗交易公共成交数据
[**api_v5_rfq_quotes_get**](DefaultApi.md#api_v5_rfq_quotes_get) | **GET** /api/v5/rfq/quotes | 获取报价单信息
[**api_v5_rfq_rfqs_get**](DefaultApi.md#api_v5_rfq_rfqs_get) | **GET** /api/v5/rfq/rfqs | 获取询价单信息
[**api_v5_rfq_trades_get**](DefaultApi.md#api_v5_rfq_trades_get) | **GET** /api/v5/rfq/trades | 获取大宗交易信息



## api_v5_rfq_cancel_all_quotes_post

> std::collections::HashMap<String, serde_json::Value> api_v5_rfq_cancel_all_quotes_post()
取消所有报价

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_cancel_all_rfqs_post

> std::collections::HashMap<String, serde_json::Value> api_v5_rfq_cancel_all_rfqs_post()
取消所有询价

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_cancel_batch_quotes_post

> std::collections::HashMap<String, serde_json::Value> api_v5_rfq_cancel_batch_quotes_post(api_v5_rfq_cancel_batch_quotes_post_request)
批量取消报价

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_rfq_cancel_batch_quotes_post_request** | Option<[**ApiV5RfqCancelBatchQuotesPostRequest**](ApiV5RfqCancelBatchQuotesPostRequest.md)> |  |  |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_cancel_batch_rfqs_post

> std::collections::HashMap<String, serde_json::Value> api_v5_rfq_cancel_batch_rfqs_post(api_v5_rfq_cancel_batch_rfqs_post_request)
批量取消询价

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_rfq_cancel_batch_rfqs_post_request** | Option<[**ApiV5RfqCancelBatchRfqsPostRequest**](ApiV5RfqCancelBatchRfqsPostRequest.md)> |  |  |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_cancel_quote_post

> std::collections::HashMap<String, serde_json::Value> api_v5_rfq_cancel_quote_post(api_v5_rfq_cancel_quote_post_request)
取消报价

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_rfq_cancel_quote_post_request** | Option<[**ApiV5RfqCancelQuotePostRequest**](ApiV5RfqCancelQuotePostRequest.md)> |  |  |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_cancel_rfq_post

> std::collections::HashMap<String, serde_json::Value> api_v5_rfq_cancel_rfq_post(api_v5_rfq_cancel_rfq_post_request)
取消询价

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_rfq_cancel_rfq_post_request** | Option<[**ApiV5RfqCancelRfqPostRequest**](ApiV5RfqCancelRfqPostRequest.md)> |  |  |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_counterparties_get

> std::collections::HashMap<String, serde_json::Value> api_v5_rfq_counterparties_get()
获取报价方信息

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_create_quote_post

> std::collections::HashMap<String, serde_json::Value> api_v5_rfq_create_quote_post(api_v5_rfq_create_quote_post_request)
报价

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_rfq_create_quote_post_request** | [**ApiV5RfqCreateQuotePostRequest**](ApiV5RfqCreateQuotePostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_create_rfq_post

> std::collections::HashMap<String, serde_json::Value> api_v5_rfq_create_rfq_post(api_v5_rfq_create_rfq_post_request)
询价

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_rfq_create_rfq_post_request** | [**ApiV5RfqCreateRfqPostRequest**](ApiV5RfqCreateRfqPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_execute_quote_post

> serde_json::Value api_v5_rfq_execute_quote_post(api_v5_rfq_execute_quote_post_request)
执行报价

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_rfq_execute_quote_post_request** | Option<[**ApiV5RfqExecuteQuotePostRequest**](ApiV5RfqExecuteQuotePostRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_maker_instrument_settings_post

> serde_json::Value api_v5_rfq_maker_instrument_settings_post(api_v5_rfq_maker_instrument_settings_post_request)
设置可报价产品

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_rfq_maker_instrument_settings_post_request** | Option<[**ApiV5RfqMakerInstrumentSettingsPostRequest**](ApiV5RfqMakerInstrumentSettingsPostRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_mmp_reset_post

> std::collections::HashMap<String, serde_json::Value> api_v5_rfq_mmp_reset_post()
重设MMP状态

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_public_trades_get

> serde_json::Value api_v5_rfq_public_trades_get(begin_id, end_id, limit)
获取大宗交易公共成交数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**begin_id** | Option<**String**> | 请求的起始大宗交易ID，请求此ID之后（更新的数据）的分页内容，不包括 beginId |  |
**end_id** | Option<**String**> | 请求的结束大宗交易ID，请求此ID之前（更旧的数据）的分页内容，不包括 endId |  |
**limit** | Option<**String**> | 返回结果的数量，默认100条。 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_quotes_get

> serde_json::Value api_v5_rfq_quotes_get(rfq_id, cl_rfq_id, cl_quote_id, quote_id, state, begin_id, end_id, limit)
获取报价单信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_id** | Option<**String**> | 询价单ID |  |
**cl_rfq_id** | Option<**String**> | 询价单自定义ID， 当 clRfqId 和 rfqId 都传时，以 rfqId 为准。 |  |
**cl_quote_id** | Option<**String**> | 报价单自定义ID，当 clRfqId 和 rfqId 都传时，以 rfqId 为准。 |  |
**quote_id** | Option<**String**> | 报价单ID |  |
**state** | Option<**String**> | 报价单的状态，`active` `canceled` `pending_fill` `filled` `expired` `failed` |  |
**begin_id** | Option<**String**> | 请求的起始报价单ID，请求此ID之后（更新的数据）的分页内容，不包括 beginId |  |
**end_id** | Option<**String**> | 请求的结束报价单ID，请求此ID之前（更旧的数据）的分页内容，不包括 endId |  |
**limit** | Option<**String**> | 返回结果的数量，默认100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_rfqs_get

> serde_json::Value api_v5_rfq_rfqs_get(rfq_id, cl_rfq_id, state, begin_id, end_id, limit)
获取询价单信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_id** | Option<**String**> | 询价单ID |  |
**cl_rfq_id** | Option<**String**> | 客户询价单自定义ID，当 clRfqId 和 rfqId 都传时，以 rfqId 为准 |  |
**state** | Option<**String**> | 询价单的状态，`active``canceled``pending_fill``filled``expired``traded_away``failed``traded_away`。`traded_away` 仅适用于报价方 |  |
**begin_id** | Option<**String**> | 请求的起始询价单ID，请求此ID之后（更新的数据）的分页内容，不包括 beginId |  |
**end_id** | Option<**String**> | 请求的结束询价单ID，请求此ID之前（更旧的数据）的分页内容，不包括 endId |  |
**limit** | Option<**String**> | 返回结果的数量，默认100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rfq_trades_get

> serde_json::Value api_v5_rfq_trades_get(rfq_id, cl_rfq_id, block_td_id, cl_quote_id, quote_id, state, begin_id, end_id, limit)
获取大宗交易信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_id** | Option<**String**> | 询价单ID |  |
**cl_rfq_id** | Option<**String**> | 由用户设置的 RFQ ID. 如果 clRfqId 和 rfqId 都传， 以rfqId 为准 |  |
**block_td_id** | Option<**String**> | 大宗交易ID |  |
**cl_quote_id** | Option<**String**> | 由用户设置的 Quote ID。如果clQuoteId 和 quoteId 都传，则 以 quoteId 为准 |  |
**quote_id** | Option<**String**> | 报价单ID |  |
**state** | Option<**String**> | 询价单的状态，active canceled pending_fill filled expired traded_away failed traded_away。traded_away 仅适用于报价方 |  |
**begin_id** | Option<**String**> | 请求的起始大宗交易ID，请求此ID之后（更新的数据）的分页内容，不包括 beginId |  |
**end_id** | Option<**String**> | 请求的结束大宗交易ID，请求此ID之前（更旧的数据）的分页内容，不包括 endId |  |
**limit** | Option<**String**> | 返回结果的数量，默认100条。 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

