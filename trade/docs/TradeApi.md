# \TradeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v5_trade_amend_batch_orders_post**](TradeApi.md#api_v5_trade_amend_batch_orders_post) | **POST** /api/v5/trade/amend-batch-orders | 批量改单
[**api_v5_trade_amend_order_post**](TradeApi.md#api_v5_trade_amend_order_post) | **POST** /api/v5/trade/amend-order | 改单
[**api_v5_trade_batch_orders_post**](TradeApi.md#api_v5_trade_batch_orders_post) | **POST** /api/v5/trade/batch-orders | 批量下单
[**api_v5_trade_cancel_advance_algos_post**](TradeApi.md#api_v5_trade_cancel_advance_algos_post) | **POST** /api/v5/trade/cancel-advance-algos | 撤销高级策略委托订单
[**api_v5_trade_cancel_algos_post**](TradeApi.md#api_v5_trade_cancel_algos_post) | **POST** /api/v5/trade/cancel-algos | 撤销策略委托订单
[**api_v5_trade_cancel_batch_orders_post**](TradeApi.md#api_v5_trade_cancel_batch_orders_post) | **POST** /api/v5/trade/cancel-batch-orders | 批量撤单
[**api_v5_trade_cancel_order_post**](TradeApi.md#api_v5_trade_cancel_order_post) | **POST** /api/v5/trade/cancel-order | 撤单
[**api_v5_trade_close_position_post**](TradeApi.md#api_v5_trade_close_position_post) | **POST** /api/v5/trade/close-position | 仓位市价全平
[**api_v5_trade_fills_get**](TradeApi.md#api_v5_trade_fills_get) | **GET** /api/v5/trade/fills | 获取成交明细（近三天）
[**api_v5_trade_fills_history_get**](TradeApi.md#api_v5_trade_fills_history_get) | **GET** /api/v5/trade/fills-history | 获取成交明细（近三个月）
[**api_v5_trade_order_algo_post**](TradeApi.md#api_v5_trade_order_algo_post) | **POST** /api/v5/trade/order-algo | 策略委托下单
[**api_v5_trade_order_get**](TradeApi.md#api_v5_trade_order_get) | **GET** /api/v5/trade/order | 获取订单信息
[**api_v5_trade_order_post**](TradeApi.md#api_v5_trade_order_post) | **POST** /api/v5/trade/order | 下单
[**api_v5_trade_orders_algo_history_get**](TradeApi.md#api_v5_trade_orders_algo_history_get) | **GET** /api/v5/trade/orders-algo-history | 获取历史策略委托单列表
[**api_v5_trade_orders_algo_pending_get**](TradeApi.md#api_v5_trade_orders_algo_pending_get) | **GET** /api/v5/trade/orders-algo-pending | 获取未完成策略委托单列表
[**api_v5_trade_orders_history_archive_get**](TradeApi.md#api_v5_trade_orders_history_archive_get) | **GET** /api/v5/trade/orders-history-archive | 获取历史订单记录（近三个月）
[**api_v5_trade_orders_history_get**](TradeApi.md#api_v5_trade_orders_history_get) | **GET** /api/v5/trade/orders-history | 获取历史订单记录（近七天）
[**api_v5_trade_orders_pending_get**](TradeApi.md#api_v5_trade_orders_pending_get) | **GET** /api/v5/trade/orders-pending | 获取未成交订单列表



## api_v5_trade_amend_batch_orders_post

> std::collections::HashMap<String, serde_json::Value> api_v5_trade_amend_batch_orders_post(api_v5_trade_amend_order_post_request)
批量改单

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_trade_amend_order_post_request** | [**ApiV5TradeAmendOrderPostRequest**](ApiV5TradeAmendOrderPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_amend_order_post

> std::collections::HashMap<String, serde_json::Value> api_v5_trade_amend_order_post(api_v5_trade_amend_order_post_request)
改单

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_trade_amend_order_post_request** | [**ApiV5TradeAmendOrderPostRequest**](ApiV5TradeAmendOrderPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_batch_orders_post

> std::collections::HashMap<String, serde_json::Value> api_v5_trade_batch_orders_post(api_v5_trade_order_post_request)
批量下单

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_trade_order_post_request** | [**ApiV5TradeOrderPostRequest**](ApiV5TradeOrderPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_cancel_advance_algos_post

> std::collections::HashMap<String, serde_json::Value> api_v5_trade_cancel_advance_algos_post(api_v5_trade_cancel_algos_post_request)
撤销高级策略委托订单

撤销冰山委托、时间加权等高级策略委托订单，每次最多可以撤销10个策略委托单

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_trade_cancel_algos_post_request** | [**ApiV5TradeCancelAlgosPostRequest**](ApiV5TradeCancelAlgosPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_cancel_algos_post

> std::collections::HashMap<String, serde_json::Value> api_v5_trade_cancel_algos_post(api_v5_trade_cancel_algos_post_request)
撤销策略委托订单

撤销策略委托订单（不包含冰山委托、时间加权等高级策略订单），每次最多可以撤销10个策略委托单

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_trade_cancel_algos_post_request** | [**ApiV5TradeCancelAlgosPostRequest**](ApiV5TradeCancelAlgosPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_cancel_batch_orders_post

> std::collections::HashMap<String, serde_json::Value> api_v5_trade_cancel_batch_orders_post(api_v5_trade_cancel_order_post_request)
批量撤单

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_trade_cancel_order_post_request** | [**ApiV5TradeCancelOrderPostRequest**](ApiV5TradeCancelOrderPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_cancel_order_post

> std::collections::HashMap<String, serde_json::Value> api_v5_trade_cancel_order_post(api_v5_trade_cancel_order_post_request)
撤单

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_trade_cancel_order_post_request** | [**ApiV5TradeCancelOrderPostRequest**](ApiV5TradeCancelOrderPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_close_position_post

> std::collections::HashMap<String, serde_json::Value> api_v5_trade_close_position_post(api_v5_trade_close_position_post_request)
仓位市价全平

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_trade_close_position_post_request** | [**ApiV5TradeClosePositionPostRequest**](ApiV5TradeClosePositionPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_fills_get

> serde_json::Value api_v5_trade_fills_get(inst_type, uly, inst_id, ord_id, after, before, limit)
获取成交明细（近三天）

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`SPOT`：币币<br>`MARGIN`：币币杠杆<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 | [required] |
**uly** | Option<**String**> | 合约标的指数，如`BTC-USD` |  |
**inst_id** | Option<**String**> | 产品ID，如`BTC-USDT-SWAP` |  |
**ord_id** | Option<**String**> | 订单ID |  |
**after** | Option<**String**> | 请求此ID之前（更旧的数据）的分页内容，传的值为对应接口的`billId` |  |
**before** | Option<**String**> | 请求此ID之后（更新的数据）的分页内容，传的值为对应接口的`billId` |  |
**limit** | Option<**String**> | 返回结果的数量，默认100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_fills_history_get

> serde_json::Value api_v5_trade_fills_history_get(inst_type, uly, inst_id, ord_id, after, before, limit)
获取成交明细（近三个月）

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`SPOT`：币币<br>`MARGIN`：币币杠杆<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 | [required] |
**uly** | Option<**String**> | 合约标的指数，如`BTC-USD` |  |
**inst_id** | Option<**String**> | 产品ID，如`BTC-USDT-SWAP` |  |
**ord_id** | Option<**String**> | 订单ID |  |
**after** | Option<**String**> | 请求此ID之前（更旧的数据）的分页内容，传的值为对应接口的`billId` |  |
**before** | Option<**String**> | 请求此ID之后（更新的数据）的分页内容，传的值为对应接口的`billId` |  |
**limit** | Option<**String**> | 返回结果的数量，默认100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_order_algo_post

> std::collections::HashMap<String, serde_json::Value> api_v5_trade_order_algo_post(api_v5_trade_order_algo_post_request)
策略委托下单

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_trade_order_algo_post_request** | [**ApiV5TradeOrderAlgoPostRequest**](ApiV5TradeOrderAlgoPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_order_get

> serde_json::Value api_v5_trade_order_get(inst_id, ord_id, cl_ord_id)
获取订单信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 产品ID，如 `BTC-USDT` | [required] |
**ord_id** | Option<**String**> | 订单ID，ordId和clOrdId必须传一个，若传两个，以ordId为主 |  |
**cl_ord_id** | Option<**String**> | 用户自定义ID |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_order_post

> std::collections::HashMap<String, serde_json::Value> api_v5_trade_order_post(api_v5_trade_order_post_request)
下单

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_trade_order_post_request** | [**ApiV5TradeOrderPostRequest**](ApiV5TradeOrderPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_orders_algo_history_get

> serde_json::Value api_v5_trade_orders_algo_history_get(ord_type, algo_id, state, inst_type, inst_id, after, before, limit)
获取历史策略委托单列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ord_type** | **String** | 订单类型<br>`conditional`：单向止盈止损，`oco`：双向止盈止损，`trigger`：计划委托，`iceberg`：冰山委托，`twap`：时间加权委托 | [required] |
**algo_id** | Option<**String**> | 策略委托单ID<br>`state`和`algoId`必填且只能填其一 |  |
**state** | Option<**String**> | 订单状态<br>`effective`：已生效，`canceled`：已经撤销，`order_failed`：委托失败<br>`state`和`algoId`必填且只能填其一 |  |
**inst_type** | Option<**String**> | 产品类型<br>`SPOT`：币币<br>`MARGIN`：币币杠杆<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 |  |
**inst_id** | Option<**String**> | 产品ID，如`BTC-USDT-SWAP` |  |
**after** | Option<**String**> | 请求此ID之前（更旧的数据）的分页内容，传的值为对应接口的ordId |  |
**before** | Option<**String**> | 请求此ID之后（更新的数据）的分页内容，传的值为对应接口的`ordId` |  |
**limit** | Option<**String**> | 返回结果的数量，默认100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_orders_algo_pending_get

> serde_json::Value api_v5_trade_orders_algo_pending_get(ord_type, algo_id, inst_type, inst_id, after, before, limit)
获取未完成策略委托单列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ord_type** | **String** | 订单类型<br>`conditional`：单向止盈止损<br>`oco`：双向止盈止损<br>`trigger`：计划委托<br>`iceberg`：冰山委托<br>`twap`：时间加权委托 | [required] |
**algo_id** | Option<**String**> | 策略委托单ID |  |
**inst_type** | Option<**String**> | 产品类型<br>`SPOT`：币币<br>`MARGIN`：币币杠杆<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 |  |
**inst_id** | Option<**String**> | 产品ID，如`BTC-USDT-SWAP` |  |
**after** | Option<**String**> | 请求此ID之前（更旧的数据）的分页内容，传的值为对应接口的ordId |  |
**before** | Option<**String**> | 请求此ID之后（更新的数据）的分页内容，传的值为对应接口的ordId |  |
**limit** | Option<**String**> | 返回结果的数量，默认100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_orders_history_archive_get

> serde_json::Value api_v5_trade_orders_history_archive_get(inst_type, uly, inst_id, ord_type, state, category, after, before, limit)
获取历史订单记录（近三个月）

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`SPOT`：币币<br>`MARGIN`：币币杠杆<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 | [required] |
**uly** | Option<**String**> | 合约标的指数，如`BTC-USD` |  |
**inst_id** | Option<**String**> | 产品ID，如`BTC-USDT-SWAP` |  |
**ord_type** | Option<**String**> | 订单类型<br>`market`：市价单<br>`limit`：限价单<br>`post_only`：只做maker单<br>`fok`：全部成交或立即取消<br>`ioc`：立即成交并取消剩余<br>`optimal_limit_ioc`：市价委托立即成交并取消剩余（仅适用交割、永续） |  |
**state** | Option<**String**> | 订单状态<br>`canceled`：撤单成功<br>`filled`：完全成交 |  |
**category** | Option<**String**> | 订单种类<br>`twap`：TWAP自动换币<br>`adl`：ADL自动减仓<br>`full_liquidation`：强制平仓<br>`partial_liquidation`：强制减仓<br>`delivery`：交割 |  |
**after** | Option<**String**> | 请求此ID之前（更旧的数据）的分页内容，传的值为对应接口的ordId |  |
**before** | Option<**String**> | 请求此ID之后（更新的数据）的分页内容，传的值为对应接口的ordId |  |
**limit** | Option<**String**> | 返回结果的数量，默认100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_orders_history_get

> serde_json::Value api_v5_trade_orders_history_get(inst_type, uly, inst_id, ord_type, state, category, after, before, limit)
获取历史订单记录（近七天）

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`SPOT`：币币<br>`MARGIN`：币币杠杆<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 | [required] |
**uly** | Option<**String**> | 合约标的指数，如`BTC-USD` |  |
**inst_id** | Option<**String**> | 产品ID，如`BTC-USDT-SWAP` |  |
**ord_type** | Option<**String**> | 订单类型<br>`market`：市价单<br>`limit`：限价单<br>`post_only`：只做maker单<br>`fok`：全部成交或立即取消<br>`ioc`：立即成交并取消剩余<br>`optimal_limit_ioc`：市价委托立即成交并取消剩余（仅适用交割、永续） |  |
**state** | Option<**String**> | 订单状态<br>`canceled`：撤单成功<br>`filled`：完全成交 |  |
**category** | Option<**String**> | 订单种类<br>`twap`：TWAP自动换币<br>`adl`：ADL自动减仓<br>`full_liquidation`：强制平仓<br>`partial_liquidation`：强制减仓<br>`delivery`：交割 |  |
**after** | Option<**String**> | 请求此ID之前（更旧的数据）的分页内容，传的值为对应接口的ordId |  |
**before** | Option<**String**> | 请求此ID之后（更新的数据）的分页内容，传的值为对应接口的ordId |  |
**limit** | Option<**String**> | 返回结果的数量，默认100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_trade_orders_pending_get

> serde_json::Value api_v5_trade_orders_pending_get(inst_id, inst_type, uly, ord_type)
获取未成交订单列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | Option<**String**> | 产品ID，如`BTC-USDT-SWAP` |  |
**inst_type** | Option<**String**> | 产品类型。<br>`SPOT`：币币<br>`MARGIN`：币币杠杆<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 |  |
**uly** | Option<**String**> | 合约标的指数，如`BTC-USD` |  |
**ord_type** | Option<**String**> | 订单类型<br>`market`：市价单<br>`limit`：限价单<br>`post_only`：只做maker单<br>`fok`：全部成交或立即取消<br>`ioc`：立即成交并取消剩余<br>`optimal_limit_ioc`：市价委托立即成交并取消剩余（仅适用交割、永续） |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

