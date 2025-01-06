# \MarketDataApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v5_market_books_get**](MarketDataApi.md#api_v5_market_books_get) | **GET** /api/v5/market/books | 获取产品深度
[**api_v5_market_candles_get**](MarketDataApi.md#api_v5_market_candles_get) | **GET** /api/v5/market/candles | 获取所有交易产品K线数据
[**api_v5_market_history_candles_get**](MarketDataApi.md#api_v5_market_history_candles_get) | **GET** /api/v5/market/history-candles | 获取交易产品历史K线数据（仅主流币）
[**api_v5_market_index_candles_get**](MarketDataApi.md#api_v5_market_index_candles_get) | **GET** /api/v5/market/index-candles | 获取指数K线数据
[**api_v5_market_index_components_get**](MarketDataApi.md#api_v5_market_index_components_get) | **GET** /api/v5/market/index-components | 获取指数成分数据
[**api_v5_market_index_tickers_get**](MarketDataApi.md#api_v5_market_index_tickers_get) | **GET** /api/v5/market/index-tickers | 获取指数行情
[**api_v5_market_mark_price_candles_get**](MarketDataApi.md#api_v5_market_mark_price_candles_get) | **GET** /api/v5/market/mark-price-candles | 获取标记价格K线数据
[**api_v5_market_open_oracle_get**](MarketDataApi.md#api_v5_market_open_oracle_get) | **GET** /api/v5/market/open-oracle | Oracle上链交易数据
[**api_v5_market_platform24_volume_get**](MarketDataApi.md#api_v5_market_platform24_volume_get) | **GET** /api/v5/market/platform-24-volume | 获取平台24小时总成交量
[**api_v5_market_ticker_get**](MarketDataApi.md#api_v5_market_ticker_get) | **GET** /api/v5/market/ticker | 获取单个产品行情信息
[**api_v5_market_tickers_get**](MarketDataApi.md#api_v5_market_tickers_get) | **GET** /api/v5/market/tickers | 获取所有产品行情信息
[**api_v5_market_trades_get**](MarketDataApi.md#api_v5_market_trades_get) | **GET** /api/v5/market/trades | 获取交易产品公共成交数据



## api_v5_market_books_get

> serde_json::Value api_v5_market_books_get(inst_id, sz)
获取产品深度

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 产品ID，如：`BTC-USDT` | [required] |
**sz** | Option<**String**> | 深度档位数量<br>最大值可传400，即买卖深度共800条<br>不填写此参数，默认返回1档深度数据 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_market_candles_get

> serde_json::Value api_v5_market_candles_get(inst_id, bar, after, before, limit)
获取所有交易产品K线数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 产品ID，如：`BTC-USDT` | [required] |
**bar** | Option<**String**> | 时间粒度，默认值`1m`<br>如 [1m/3m/5m/15m/30m/1H/2H/4H/6H/12H/1D/1W/1M/3M/6M/1Y] |  |
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


## api_v5_market_history_candles_get

> serde_json::Value api_v5_market_history_candles_get(inst_id, bar, after, before, limit)
获取交易产品历史K线数据（仅主流币）

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 产品ID，如：`BTC-USDT` | [required] |
**bar** | Option<**String**> | 时间粒度，默认值`1m`<br>如 [1m/3m/5m/15m/30m/1H/2H/4H/6H/12H/1D/1W/1M/3M/6M/1Y] |  |
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


## api_v5_market_index_candles_get

> serde_json::Value api_v5_market_index_candles_get(inst_id, bar, after, before, limit)
获取指数K线数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 现货指数，如：`BTC-USD` | [required] |
**bar** | Option<**String**> | 时间粒度，默认值`1m`<br>如 [1m/3m/5m/15m/30m/1H/2H/4H/6H/12H/1D/1W/1M/3M/6M/1Y] |  |
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


## api_v5_market_index_components_get

> serde_json::Value api_v5_market_index_components_get(index)
获取指数成分数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | 指数，如：`BTC-USDT` | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_market_index_tickers_get

> serde_json::Value api_v5_market_index_tickers_get(inst_id, quote_ccy)
获取指数行情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | Option<**String**> | 指数，如：`BTC-USD`<br>`instId`和`quoteCcy`必须填写一个 |  |
**quote_ccy** | Option<**String**> | 指数计价单位<br>目前只有`USD`/`USDT`/`BTC`为计价单位的指数 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_market_mark_price_candles_get

> serde_json::Value api_v5_market_mark_price_candles_get(inst_id, bar, after, before, limit)
获取标记价格K线数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 现货指数，如：`BTC-USD-SWAP` | [required] |
**bar** | Option<**String**> | 时间粒度，默认值`1m`<br>如 [1m/3m/5m/15m/30m/1H/2H/4H/6H/12H/1D/1W/1M/3M/6M/1Y] |  |
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


## api_v5_market_open_oracle_get

> serde_json::Value api_v5_market_open_oracle_get()
Oracle上链交易数据

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


## api_v5_market_platform24_volume_get

> serde_json::Value api_v5_market_platform24_volume_get()
获取平台24小时总成交量

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


## api_v5_market_ticker_get

> serde_json::Value api_v5_market_ticker_get(inst_id)
获取单个产品行情信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 产品ID，如：`BTC-USD-SWAP` | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_market_tickers_get

> serde_json::Value api_v5_market_tickers_get(inst_type, uly)
获取所有产品行情信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类型<br>`SPOT`：币币<br>`SWAP`：永续合约<br>`FUTURES`：交割合约<br>`OPTION`：期权 | [required] |
**uly** | Option<**String**> | 合约标的指数<br>仅适用于`交割/永续/期权`,如 `BTC-USD` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_market_trades_get

> serde_json::Value api_v5_market_trades_get(inst_id, limit)
获取交易产品公共成交数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inst_id** | **String** | 产品ID，如：`BTC-USDT` | [required] |
**limit** | Option<**String**> | 分页返回的结果集数量，最大为500，不填默认返回100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

