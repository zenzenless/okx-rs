# \PublicDataApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v5_public_delivery_exercise_history_get**](PublicDataApi.md#api_v5_public_delivery_exercise_history_get) | **GET** /api/v5/public/delivery-exercise-history | 获取交割和行权记录
[**api_v5_public_discount_rate_interest_free_quota_get**](PublicDataApi.md#api_v5_public_discount_rate_interest_free_quota_get) | **GET** /api/v5/public/discount-rate-interest-free-quota | 获取免息额度和币种折算率等级
[**api_v5_public_estimated_price_get**](PublicDataApi.md#api_v5_public_estimated_price_get) | **GET** /api/v5/public/estimated-price | 获取预估交割/行权价格
[**api_v5_public_funding_rate_get**](PublicDataApi.md#api_v5_public_funding_rate_get) | **GET** /api/v5/public/funding-rate | 获取永续合约当前资金费率
[**api_v5_public_funding_rate_history_get**](PublicDataApi.md#api_v5_public_funding_rate_history_get) | **GET** /api/v5/public/funding-rate-history | 获取永续合约历史资金费率
[**api_v5_public_instruments_get**](PublicDataApi.md#api_v5_public_instruments_get) | **GET** /api/v5/public/instruments | 获取所有产品行情信息
[**api_v5_public_insurance_fund_get**](PublicDataApi.md#api_v5_public_insurance_fund_get) | **GET** /api/v5/public/insurance-fund | 获取风险准备金余额
[**api_v5_public_interest_rate_loan_quota_get**](PublicDataApi.md#api_v5_public_interest_rate_loan_quota_get) | **GET** /api/v5/public/interest-rate-loan-quota | 获取杠杆利率和借币限额
[**api_v5_public_liquidation_orders_get**](PublicDataApi.md#api_v5_public_liquidation_orders_get) | **GET** /api/v5/public/liquidation-orders | 获取平台公共爆仓单信息
[**api_v5_public_mark_price_get**](PublicDataApi.md#api_v5_public_mark_price_get) | **GET** /api/v5/public/mark-price | 获取标记价格
[**api_v5_public_open_interest_get**](PublicDataApi.md#api_v5_public_open_interest_get) | **GET** /api/v5/public/open-interest | 获取持仓总量
[**api_v5_public_opt_summary_get**](PublicDataApi.md#api_v5_public_opt_summary_get) | **GET** /api/v5/public/opt-summary | 获取期权定价
[**api_v5_public_position_tiers_get**](PublicDataApi.md#api_v5_public_position_tiers_get) | **GET** /api/v5/public/position-tiers | 获取衍生品仓位档位
[**api_v5_public_price_limit_get**](PublicDataApi.md#api_v5_public_price_limit_get) | **GET** /api/v5/public/price-limit | 获取限价
[**api_v5_public_time_get**](PublicDataApi.md#api_v5_public_time_get) | **GET** /api/v5/public/time | 获取系统时间
[**api_v5_public_underlying_get**](PublicDataApi.md#api_v5_public_underlying_get) | **GET** /api/v5/public/underlying | 获取衍生品标的指数



## api_v5_public_delivery_exercise_history_get

> serde_json::Value api_v5_public_delivery_exercise_history_get(inst_type, uly, after, before, limit)
获取交割和行权记录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`FUTURES`：交割合约，`OPTION`：期权 | [required] |
**uly** | **String** | 合约标的指数，如：`BTC-USD`<br>仅适用于`交割/期权` | [required] |
**after** | Option<**String**> | 请求此时间戳之前（更旧的数据）的分页内容，传的值为对应接口的`ts` |  |
**before** | Option<**String**> | 请求此时间戳之后（更新的数据）的分页内容，传的值为对应接口的`ts` |  |
**limit** | Option<**String**> | 分页返回的结果集数量，最大为100，不填默认返回100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_discount_rate_interest_free_quota_get

> serde_json::Value api_v5_public_discount_rate_interest_free_quota_get(ccy, discount_lv)
获取免息额度和币种折算率等级

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | Option<**String**> | 币种，如：`BTC` |  |
**discount_lv** | Option<**String**> | 折算率等级<br>`1`:第一档，`2`:第二档，`3`:第三档，`4`:第四档，`5`:第五档 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_estimated_price_get

> serde_json::Value api_v5_public_estimated_price_get(inst_id)
获取预估交割/行权价格

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 产品ID，如：`BTC-USD-200214`<br>适用于`交割/期权` | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_funding_rate_get

> serde_json::Value api_v5_public_funding_rate_get(inst_id)
获取永续合约当前资金费率

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | Option<**String**> | 产品ID，如：`BTC-USDT-SWAP`<br>仅适用于`永续` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_funding_rate_history_get

> serde_json::Value api_v5_public_funding_rate_history_get(inst_id, after, before, limit)
获取永续合约历史资金费率

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | Option<**String**> | 产品ID，如：`BTC-USDT-SWAP`<br>仅适用于`永续` |  |
**after** | Option<**String**> | 请求此时间戳之前（更旧的数据）的分页内容，传的值为对应接口的`fundingTime` |  |
**before** | Option<**String**> | 请求此时间戳之后（更新的数据）的分页内容，传的值为对应接口的`fundingTime` |  |
**limit** | Option<**String**> | 分页返回的结果集数量，最大为100，不填默认返回100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_instruments_get

> serde_json::Value api_v5_public_instruments_get(inst_type, uly, inst_id)
获取所有产品行情信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`SPOT`：币币；`MARGIN`：币币杠杆；`SWAP`：永续合约 `FUTURES`：交割合约；`OPTION`：期权 | [required] |
**uly** | Option<**String**> | 合约标的指数，如：`BTC-USD`<br>仅适用于`交割/永续/期权`，`期权`必填 |  |
**inst_id** | Option<**String**> | 产品ID，如 `BTC-USDT` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_insurance_fund_get

> serde_json::Value api_v5_public_insurance_fund_get(inst_type, r#type, uly, ccy, before, after, limit)
获取风险准备金余额

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`MARGIN`：币币杠杆，`SWAP`：永续合约，`FUTURES`：交割合约，`OPTION`：期权 | [required] |
**r#type** | Option<**String**> | 产品类型<br>`liquidation_balance_deposit`：强平注入，`bankruptcy_loss`：穿仓亏损，`platform_revenue`：平台收入注入 |  |
**uly** | Option<**String**> | 标的指数<br>仅适用于`交割/永续/期权`，且必填写 |  |
**ccy** | Option<**String**> | 币种，仅适用`币币杠杆`，且必填写 |  |
**before** | Option<**String**> | 请求此时间戳之后（更新的数据）的分页内容，传的值为对应接口的`ts` |  |
**after** | Option<**String**> | 请求此时间戳之前（更旧的数据）的分页内容，传的值为对应接口的`ts` |  |
**limit** | Option<**String**> | 分页返回的结果集数量，最大为100，不填默认返回100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_interest_rate_loan_quota_get

> serde_json::Value api_v5_public_interest_rate_loan_quota_get()
获取杠杆利率和借币限额

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


## api_v5_public_liquidation_orders_get

> serde_json::Value api_v5_public_liquidation_orders_get(inst_type, mgn_mode, inst_id, ccy, uly, alias, state, after, before, limit)
获取平台公共爆仓单信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`MARGIN`：币币杠杆，`SWAP`：永续合约，`FUTURES`：交割合约，`OPTION`：期权 | [required] |
**mgn_mode** | Option<**String**> | 保证金模式<br>`cross`：全仓，`isolated`：逐仓 |  |
**inst_id** | Option<**String**> | 产品ID，仅适用于`币币杠杆` |  |
**ccy** | Option<**String**> | 币种，仅适用于全仓`币币杠杆` |  |
**uly** | Option<**String**> | 合约标的指数<br>`交割/永续/期权`合约情况下，该参数必填 |  |
**alias** | Option<**String**> | `this_week`：本周，`next_week`：次周，`quarter`：季度，`next_quarter`：次季度<br>`交割`合约情况下，该参数必填 |  |
**state** | Option<**String**> | 状态<br>`unfilled`：未成交，`filled`：已成交<br>默认为`unfilled`<br>`交割/永续`合约情况下，该参数必填 |  |
**after** | Option<**String**> | 请求此时间戳之前（更旧的数据）的分页内容，传的值为对应接口的`ts` |  |
**before** | Option<**String**> | 请求此时间戳之后（更新的数据）的分页内容，传的值为对应接口的`ts` |  |
**limit** | Option<**String**> | 分页返回的结果集数量，最大为100，不填默认返回100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_mark_price_get

> serde_json::Value api_v5_public_mark_price_get(inst_type, inst_id, uly)
获取标记价格

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`MARGIN`：币币杠杆，`SWAP`：永续合约，`FUTURES`：交割合约，`OPTION`：期权 | [required] |
**inst_id** | Option<**String**> | 产品ID，如：`BTC-USDT-SWAP` |  |
**uly** | Option<**String**> | 合约标的指数，如：`BTC-USD` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_open_interest_get

> serde_json::Value api_v5_public_open_interest_get(inst_type, uly, inst_id)
获取持仓总量

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`FUTURES`：交割合约，`SWAP`：永续合约，`OPTION`：期权 | [required] |
**uly** | Option<**String**> | 合约标的指数，如：`BTC-USD`<br>仅适用于`交割/永续/期权` |  |
**inst_id** | Option<**String**> | 产品ID，如：`BTC-USD-SWAP`<br>仅适用于`交割/永续/期权` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_opt_summary_get

> serde_json::Value api_v5_public_opt_summary_get(uly, exp_time)
获取期权定价

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uly** | **String** | 合约标的指数，如：`BTC-USD-200103-5500-C`<br>仅适用于`期权` | [required] |
**exp_time** | **String** | 合约到期日，格式为`YYMMDD`，如 `200527` | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_position_tiers_get

> serde_json::Value api_v5_public_position_tiers_get(inst_type, td_mode, inst_id, uly, tier)
获取衍生品仓位档位

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`MARGIN`：币币杠杆，`SWAP`：永续合约，`FUTURES`：交割合约，`OPTION`：期权 | [required] |
**td_mode** | **String** | 交易模式<br>`isolated`：逐仓，`cross`：全仓 | [required] |
**inst_id** | Option<**String**> | 产品ID，如：`BTC-USDT`<br>仅适用`币币杠杆`，且必填 |  |
**uly** | Option<**String**> | 合约标的指数，如：`BTC-USD`<br>仅适用于`交割/永续/期权`，且必填 |  |
**tier** | Option<**String**> | 指定档位 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_price_limit_get

> serde_json::Value api_v5_public_price_limit_get(inst_id)
获取限价

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 产品ID，如：`BTC-USDT-SWAP`<br>适用于`交割/永续/期权` | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_public_time_get

> serde_json::Value api_v5_public_time_get()
获取系统时间

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


## api_v5_public_underlying_get

> serde_json::Value api_v5_public_underlying_get(inst_type)
获取衍生品标的指数

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`SWAP`：永续合约，`FUTURES`：交割合约，`OPTION`：期权 | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

