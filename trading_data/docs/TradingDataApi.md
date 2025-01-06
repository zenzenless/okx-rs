# \TradingDataApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v5_rubik_stat_contracts_long_short_account_ratio_get**](TradingDataApi.md#api_v5_rubik_stat_contracts_long_short_account_ratio_get) | **GET** /api/v5/rubik/stat/contracts/long-short-account-ratio | 获取多空持仓人数比
[**api_v5_rubik_stat_contracts_open_interest_volume_get**](TradingDataApi.md#api_v5_rubik_stat_contracts_open_interest_volume_get) | **GET** /api/v5/rubik/stat/contracts/open-interest-volume | 获取持仓总量及交易量
[**api_v5_rubik_stat_margin_loan_ratio_get**](TradingDataApi.md#api_v5_rubik_stat_margin_loan_ratio_get) | **GET** /api/v5/rubik/stat/margin/loan-ratio | 获取杠杆多空比
[**api_v5_rubik_stat_option_open_interest_volume_expiry_get**](TradingDataApi.md#api_v5_rubik_stat_option_open_interest_volume_expiry_get) | **GET** /api/v5/rubik/stat/option/open-interest-volume-expiry | 看涨看跌持仓总量及交易总量（按到期日分）
[**api_v5_rubik_stat_option_open_interest_volume_get**](TradingDataApi.md#api_v5_rubik_stat_option_open_interest_volume_get) | **GET** /api/v5/rubik/stat/option/open-interest-volume | 获取持仓总量及交易量
[**api_v5_rubik_stat_option_open_interest_volume_ratio_get**](TradingDataApi.md#api_v5_rubik_stat_option_open_interest_volume_ratio_get) | **GET** /api/v5/rubik/stat/option/open-interest-volume-ratio | 看涨（跌）期权合约持仓总量比/交易总量比
[**api_v5_rubik_stat_option_open_interest_volume_strike_get**](TradingDataApi.md#api_v5_rubik_stat_option_open_interest_volume_strike_get) | **GET** /api/v5/rubik/stat/option/open-interest-volume-strike | 看涨看跌持仓总量及交易总量（按执行价格分）
[**api_v5_rubik_stat_option_taker_block_volume_get**](TradingDataApi.md#api_v5_rubik_stat_option_taker_block_volume_get) | **GET** /api/v5/rubik/stat/option/taker-block-volume | 看涨（跌）期权合约主动买入/卖出量
[**api_v5_rubik_stat_taker_volume_get**](TradingDataApi.md#api_v5_rubik_stat_taker_volume_get) | **GET** /api/v5/rubik/stat/taker-volume | 获取币币或衍生品主动买入/卖出情况
[**api_v5_rubik_stat_trading_data_support_coin_get**](TradingDataApi.md#api_v5_rubik_stat_trading_data_support_coin_get) | **GET** /api/v5/rubik/stat/trading-data/support-coin | 获取交易大数据数据支持币种



## api_v5_rubik_stat_contracts_long_short_account_ratio_get

> serde_json::Value api_v5_rubik_stat_contracts_long_short_account_ratio_get(ccy, begin, end, period)
获取多空持仓人数比

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | **String** | 币种，如：`BTC` | [required] |
**begin** | Option<**String**> | 开始时间，如：1597026383085 |  |
**end** | Option<**String**> | 结束时间，如：1597026383085 |  |
**period** | Option<**String**> | 时间粒度，默认值`5m`。支持[5m/1H/1D]<br>`5m`粒度最多只能查询两天之内的数据<br>`1H`粒度最多只能查询30天之内的数据<br>`1D`粒度最多只能查询180天之内的数据 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rubik_stat_contracts_open_interest_volume_get

> serde_json::Value api_v5_rubik_stat_contracts_open_interest_volume_get(ccy, begin, end, period)
获取持仓总量及交易量

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | **String** | 币种，如：`BTC` | [required] |
**begin** | Option<**String**> | 开始时间，如：1597026383085 |  |
**end** | Option<**String**> | 结束时间，如：1597026383085 |  |
**period** | Option<**String**> | 时间粒度，默认值`5m`。支持[5m/1H/1D]<br>`5m`粒度最多只能查询两天之内的数据<br>`1H`粒度最多只能查询30天之内的数据<br>`1D`粒度最多只能查询180天之内的数据 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rubik_stat_margin_loan_ratio_get

> serde_json::Value api_v5_rubik_stat_margin_loan_ratio_get(ccy, begin, end, period)
获取杠杆多空比

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | **String** | 币种，如：`BTC` | [required] |
**begin** | Option<**String**> | 开始时间，如：1597026383085 |  |
**end** | Option<**String**> | 结束时间，如：1597026383085 |  |
**period** | Option<**String**> | 时间粒度，默认值`5m`。支持[5m/1H/1D]<br>`5m`粒度最多只能查询两天之内的数据<br>`1H`粒度最多只能查询30天之内的数据<br>`1D`粒度最多只能查询180天之内的数据 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rubik_stat_option_open_interest_volume_expiry_get

> serde_json::Value api_v5_rubik_stat_option_open_interest_volume_expiry_get(ccy, period)
看涨看跌持仓总量及交易总量（按到期日分）

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | **String** | 币种，如：`BTC` | [required] |
**period** | Option<**String**> | 时间粒度，默认值`8H`。支持[8H/1D]<br>每个粒度最多只能查询72条数据 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rubik_stat_option_open_interest_volume_get

> serde_json::Value api_v5_rubik_stat_option_open_interest_volume_get(ccy, period)
获取持仓总量及交易量

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | **String** | 币种，如：`BTC` | [required] |
**period** | Option<**String**> | 时间粒度，默认值`8H`。支持[8H/1D]<br>每个粒度最多只能查询72条数据 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rubik_stat_option_open_interest_volume_ratio_get

> serde_json::Value api_v5_rubik_stat_option_open_interest_volume_ratio_get(ccy, period)
看涨（跌）期权合约持仓总量比/交易总量比

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | **String** | 币种，如：`BTC` | [required] |
**period** | Option<**String**> | 时间粒度，默认值`8H`。支持[8H/1D]<br>每个粒度最多只能查询72条数据 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rubik_stat_option_open_interest_volume_strike_get

> serde_json::Value api_v5_rubik_stat_option_open_interest_volume_strike_get(ccy, exp_time, period)
看涨看跌持仓总量及交易总量（按执行价格分）

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | **String** | 币种，如：`BTC` | [required] |
**exp_time** | **String** | 到期日（格式: YYYYMMdd，例如：`20210623`） | [required] |
**period** | Option<**String**> | 时间粒度，默认值`8H`。支持[8H/1D]<br>每个粒度最多只能查询72条数据 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rubik_stat_option_taker_block_volume_get

> serde_json::Value api_v5_rubik_stat_option_taker_block_volume_get(ccy, period)
看涨（跌）期权合约主动买入/卖出量

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | **String** | 币种，如：`BTC` | [required] |
**period** | Option<**String**> | 时间粒度，默认值`8H`。支持[8H/1D]<br>每个粒度最多只能查询72条数据 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rubik_stat_taker_volume_get

> serde_json::Value api_v5_rubik_stat_taker_volume_get(ccy, inst_type, begin, end, period)
获取币币或衍生品主动买入/卖出情况

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | **String** | 币种，如：`BTC` | [required] |
**inst_type** | **String** | 产品类型<br>币币：`SPOT`, 衍生品：`CONTRACTS` | [required] |
**begin** | Option<**String**> | 开始时间，例如：1597026383085 |  |
**end** | Option<**String**> | 结束时间，例如：1597026383085 |  |
**period** | Option<**String**> | 时间粒度，默认值`5m`。支持[5m/1H/1D]<br>`5m`粒度最多只能查询两天之内的数据<br>`1H`粒度最多只能查询30天之内的数据<br>`1D`粒度最多只能查询180天之内的数据 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_rubik_stat_trading_data_support_coin_get

> serde_json::Value api_v5_rubik_stat_trading_data_support_coin_get()
获取交易大数据数据支持币种



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

