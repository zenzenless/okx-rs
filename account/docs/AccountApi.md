# \AccountApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v5_account_account_position_risk_get**](AccountApi.md#api_v5_account_account_position_risk_get) | **GET** /api/v5/account/account-position-risk | 查看账户持仓风险
[**api_v5_account_balance_get**](AccountApi.md#api_v5_account_balance_get) | **GET** /api/v5/account/balance | 获取账户中资金余额信息
[**api_v5_account_bills_archive_get**](AccountApi.md#api_v5_account_bills_archive_get) | **GET** /api/v5/account/bills-archive | 账单流水查询（近三月）
[**api_v5_account_bills_get**](AccountApi.md#api_v5_account_bills_get) | **GET** /api/v5/account/bills | 账单流水查询（近七天）
[**api_v5_account_config_get**](AccountApi.md#api_v5_account_config_get) | **GET** /api/v5/account/config | 查看账户配置
[**api_v5_account_greeks_get**](AccountApi.md#api_v5_account_greeks_get) | **GET** /api/v5/account/greeks | 查看账户greeks
[**api_v5_account_interest_accrued_get**](AccountApi.md#api_v5_account_interest_accrued_get) | **GET** /api/v5/account/interest-accrued | 获取计息记录
[**api_v5_account_interest_rate_get**](AccountApi.md#api_v5_account_interest_rate_get) | **GET** /api/v5/account/interest-rate | 获取用户当前杠杆借币利率
[**api_v5_account_leverage_info_get**](AccountApi.md#api_v5_account_leverage_info_get) | **GET** /api/v5/account/leverage-info | 获取杠杆倍数
[**api_v5_account_max_avail_size_get**](AccountApi.md#api_v5_account_max_avail_size_get) | **GET** /api/v5/account/max-avail-size | 获取最大可用数量
[**api_v5_account_max_loan_get**](AccountApi.md#api_v5_account_max_loan_get) | **GET** /api/v5/account/max-loan | 获取交易产品最大可借
[**api_v5_account_max_size_get**](AccountApi.md#api_v5_account_max_size_get) | **GET** /api/v5/account/max-size | 获取最大可买卖/开仓数量
[**api_v5_account_max_withdrawal_get**](AccountApi.md#api_v5_account_max_withdrawal_get) | **GET** /api/v5/account/max-withdrawal | 查看账户最大可转余额
[**api_v5_account_position_margin_balance_post**](AccountApi.md#api_v5_account_position_margin_balance_post) | **POST** /api/v5/account/position/margin-balance | 调整保证金
[**api_v5_account_positions_get**](AccountApi.md#api_v5_account_positions_get) | **GET** /api/v5/account/positions | 查看持仓信息
[**api_v5_account_set_greeks_post**](AccountApi.md#api_v5_account_set_greeks_post) | **POST** /api/v5/account/set-greeks | 期权希腊字母PA/BS切换
[**api_v5_account_set_isolated_mode_post**](AccountApi.md#api_v5_account_set_isolated_mode_post) | **POST** /api/v5/account/set-isolated-mode | 逐仓交易设置
[**api_v5_account_set_leverage_post**](AccountApi.md#api_v5_account_set_leverage_post) | **POST** /api/v5/account/set-leverage | 设置杠杆倍数
[**api_v5_account_set_position_mode_post**](AccountApi.md#api_v5_account_set_position_mode_post) | **POST** /api/v5/account/set-position-mode | 设置持仓模式
[**api_v5_account_simulated_margin_post**](AccountApi.md#api_v5_account_simulated_margin_post) | **POST** /api/v5/account/simulated_margin | 组合保证金的虚拟持仓保证金计算
[**api_v5_account_trade_fee_get**](AccountApi.md#api_v5_account_trade_fee_get) | **GET** /api/v5/account/trade-fee | 获取当前账户交易手续费费率



## api_v5_account_account_position_risk_get

> serde_json::Value api_v5_account_account_position_risk_get(inst_type)
查看账户持仓风险

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | Option<**String**> | 产品类型<br>`SPOT`：币币<br>`MARGIN`：币币杠杆<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_balance_get

> serde_json::Value api_v5_account_balance_get(ccy)
获取账户中资金余额信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | Option<**String**> | 币种，如：`BTC` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_bills_archive_get

> serde_json::Value api_v5_account_bills_archive_get(inst_type, ccy, mgn_mode, ct_type, r#type, sub_type, after, before, limit)
账单流水查询（近三月）

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | Option<**String**> | 产品类型<br>`SPOT`：币币<br>`MARGIN`：币币杠杆<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 |  |
**ccy** | Option<**String**> | 币种，如：`BTC` |  |
**mgn_mode** | Option<**String**> | 仓位类型<br>`isolated`：逐仓 `cross`：全仓 |  |
**ct_type** | Option<**String**> | 合约类型<br>`linear`：正向合约 `inverse`：反向合约<br>仅交割/永续有效 |  |
**r#type** | Option<**String**> | 账单类型<br>1：划转 2：交易 3：交割 4：自动换币 5：强平 6：保证金划转 7：扣息 8：资金费 9：自动减仓 10：穿仓补偿 11：系统换币 12：策略划拨 |  |
**sub_type** | Option<**String**> | 子账单类型<br>1：买入 2：卖出 3：开多 4：开空 5：平多 6：平空 9：扣息 11：转入 12：转出 160：手动追加保证金 161：手动减少保证金 162：自动追加保证金 114：自动换币买入 115：自动换币卖出 118：系统换币转入 119：系统换币转出 100：强减平多 101：强减平空 102：强减买入 103：强减卖出 104：强平平多 105：强平平空 106：强平买入 107：强平卖出 110：强平换币转入 111：强平换币转出 125：自动减仓平多 126：自动减仓平空 127：自动减仓买入 128：自动减仓卖出 170：到期行权 171：到期被行权 172：到期作废 112：交割平多 113：交割平空 117：交割/期权穿仓补偿 173：资金费支出 174：资金费收入 200:系统转入 201:手动转入 202:系统转出 203:手动转出 |  |
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


## api_v5_account_bills_get

> serde_json::Value api_v5_account_bills_get(inst_type, ccy, mgn_mode, ct_type, r#type, sub_type, after, before, limit)
账单流水查询（近七天）

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | Option<**String**> | 产品类型<br>`SPOT`：币币<br>`MARGIN`：币币杠杆<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 |  |
**ccy** | Option<**String**> | 币种，如：`BTC` |  |
**mgn_mode** | Option<**String**> | 仓位类型<br>`isolated`：逐仓 `cross`：全仓 |  |
**ct_type** | Option<**String**> | 合约类型<br>`linear`：正向合约 `inverse`：反向合约<br>仅交割/永续有效 |  |
**r#type** | Option<**String**> | 账单类型<br>1：划转 2：交易 3：交割 4：自动换币 5：强平 6：保证金划转 7：扣息 8：资金费 9：自动减仓 10：穿仓补偿 11：系统换币 12：策略划拨 |  |
**sub_type** | Option<**String**> | 子账单类型<br>1：买入 2：卖出 3：开多 4：开空 5：平多 6：平空 9：扣息 11：转入 12：转出 160：手动追加保证金 161：手动减少保证金 162：自动追加保证金 114：自动换币买入 115：自动换币卖出 118：系统换币转入 119：系统换币转出 100：强减平多 101：强减平空 102：强减买入 103：强减卖出 104：强平平多 105：强平平空 106：强平买入 107：强平卖出 110：强平换币转入 111：强平换币转出 125：自动减仓平多 126：自动减仓平空 127：自动减仓买入 128：自动减仓卖出 170：到期行权 171：到期被行权 172：到期作废 112：交割平多 113：交割平空 117：交割/期权穿仓补偿 173：资金费支出 174：资金费收入 200:系统转入 201:手动转入 202:系统转出 203:手动转出 |  |
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


## api_v5_account_config_get

> serde_json::Value api_v5_account_config_get()
查看账户配置

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_greeks_get

> serde_json::Value api_v5_account_greeks_get(ccy)
查看账户greeks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | Option<**String**> | 币种，如 `BTC` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_interest_accrued_get

> serde_json::Value api_v5_account_interest_accrued_get(inst_id, mgn_mode, ccy, after, before, limit)
获取计息记录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | Option<**String**> | 产品ID，如：`BTC-USDT` |  |
**mgn_mode** | Option<**String**> | 保证金模式<br>`isolated`：逐仓 `cross`：全仓 |  |
**ccy** | Option<**String**> | 币种，如：`BTC` |  |
**after** | Option<**String**> | 查询在此之前的内容，值为时间戳，Unix时间戳为毫秒数格式 |  |
**before** | Option<**String**> | 查询在此之后的内容，值为时间戳，Unix时间戳为毫秒数格式 |  |
**limit** | Option<**String**> | 分页返回的结果集数量，最大为100，不填默认返回100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_interest_rate_get

> serde_json::Value api_v5_account_interest_rate_get(ccy)
获取用户当前杠杆借币利率

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | Option<**String**> | 币种，如：`BTC` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_leverage_info_get

> serde_json::Value api_v5_account_leverage_info_get(inst_id, mgn_mode)
获取杠杆倍数

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 产品ID，如：`BTC-USDT-SWAP`<br>支持多产品ID查询（不超过20个），半角逗号分隔 | [required] |
**mgn_mode** | **String** | 保证金模式<br>`cross`：全仓 `isolated`：逐仓 | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_max_avail_size_get

> serde_json::Value api_v5_account_max_avail_size_get(inst_id, td_mode, ccy, reduce_only, px)
获取最大可用数量

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 产品ID，如：`BTC-USDT`<br>支持多产品ID查询（不超过5个），半角逗号分隔 | [required] |
**td_mode** | **String** | 交易模式<br>`cross`：全仓 `isolated`：逐仓 `cash`：非保证金 | [required] |
**ccy** | Option<**String**> | 保证金币种，仅适用于单币种保证金模式下的全仓杠杆订单 |  |
**reduce_only** | Option<**bool**> | 是否为只减仓模式，仅适用于`币币杠杆` |  |
**px** | Option<**String**> | 委托价格，当不填委托价时会按当前最新成交价计算，当指定多个产品ID查询时，忽略该参数，按当前最新成交价计算 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_max_loan_get

> serde_json::Value api_v5_account_max_loan_get(inst_id, mgn_mode, mgn_ccy)
获取交易产品最大可借

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 产品ID，如：`BTC-USDT` | [required] |
**mgn_mode** | **String** | 仓位类型<br>`cross`：全仓 `isolated`：逐仓 | [required] |
**mgn_ccy** | Option<**String**> | 保证金币种，如：`BTC`<br>币币杠杆单币种全仓情况下必须指定保证金币种 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_max_size_get

> serde_json::Value api_v5_account_max_size_get(inst_id, td_mode, ccy, px)
获取最大可买卖/开仓数量

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 产品ID，如：`BTC-USDT`<br>支持多产品ID查询（不超过5个），半角逗号分隔 | [required] |
**td_mode** | **String** | 交易模式<br>`cross`：全仓 `isolated`：逐仓 `cash`：非保证金 | [required] |
**ccy** | Option<**String**> | 保证金币种，仅适用于单币种保证金模式下的全仓杠杆订单 |  |
**px** | Option<**String**> | 委托价格<br>当不填委托价时会按当前最新成交价计算<br>当指定多个`instId`查询时，忽略该参数，按当前最新成交价计算 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_max_withdrawal_get

> serde_json::Value api_v5_account_max_withdrawal_get(ccy)
查看账户最大可转余额

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | Option<**String**> | 币种，如：`BTC` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_position_margin_balance_post

> std::collections::HashMap<String, serde_json::Value> api_v5_account_position_margin_balance_post(api_v5_account_position_margin_balance_post_request)
调整保证金

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_account_position_margin_balance_post_request** | [**ApiV5AccountPositionMarginBalancePostRequest**](ApiV5AccountPositionMarginBalancePostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_positions_get

> serde_json::Value api_v5_account_positions_get(inst_type, inst_id, pos_id)
查看持仓信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | Option<**String**> | 产品类型<br>`SPOT`：币币<br>`MARGIN`：币币杠杆<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 |  |
**inst_id** | Option<**String**> | 产品ID，如：`BTC-USDT`<br>支持多个instId查询（不超过10个），半角逗号分隔 |  |
**pos_id** | Option<**String**> | 持仓ID，支持多个posId查询（不超过20个），半角逗号分割 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_set_greeks_post

> std::collections::HashMap<String, serde_json::Value> api_v5_account_set_greeks_post(api_v5_account_set_greeks_post_request)
期权希腊字母PA/BS切换

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_account_set_greeks_post_request** | [**ApiV5AccountSetGreeksPostRequest**](ApiV5AccountSetGreeksPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_set_isolated_mode_post

> std::collections::HashMap<String, serde_json::Value> api_v5_account_set_isolated_mode_post(api_v5_account_set_isolated_mode_post_request)
逐仓交易设置

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_account_set_isolated_mode_post_request** | [**ApiV5AccountSetIsolatedModePostRequest**](ApiV5AccountSetIsolatedModePostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_set_leverage_post

> std::collections::HashMap<String, serde_json::Value> api_v5_account_set_leverage_post(api_v5_account_set_leverage_post_request)
设置杠杆倍数

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_account_set_leverage_post_request** | [**ApiV5AccountSetLeveragePostRequest**](ApiV5AccountSetLeveragePostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_set_position_mode_post

> std::collections::HashMap<String, serde_json::Value> api_v5_account_set_position_mode_post(api_v5_account_set_position_mode_post_request)
设置持仓模式

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_account_set_position_mode_post_request** | [**ApiV5AccountSetPositionModePostRequest**](ApiV5AccountSetPositionModePostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_simulated_margin_post

> std::collections::HashMap<String, serde_json::Value> api_v5_account_simulated_margin_post(api_v5_account_simulated_margin_post_request)
组合保证金的虚拟持仓保证金计算

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_account_simulated_margin_post_request** | [**ApiV5AccountSimulatedMarginPostRequest**](ApiV5AccountSimulatedMarginPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_account_trade_fee_get

> serde_json::Value api_v5_account_trade_fee_get(inst_type, inst_id, uly, category)
获取当前账户交易手续费费率

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`SPOT`：币币<br>`MARGIN`：币币杠杆<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 | [required] |
**inst_id** | Option<**String**> | 产品ID，如：`BTC-USDT`<br>仅适用于instType为币币/币币杠杆 |  |
**uly** | Option<**String**> | 合约标的指数，如：`BTC-USD`<br>仅适用于`instType`为`交割`/`永续`/`期权` |  |
**category** | Option<**String**> | 币种手续费类别<br>1：第一类币种费率 2：第二类币种费率 3：第三类币种费率 4：第四类币种费率 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

