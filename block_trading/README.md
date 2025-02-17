# Rust API client for openapi

# 使用说明 
 <b>该功能接口用户需先登陆，接口只会请求模拟环境</b><br><br>*Parameters* 面板中点击`Try it out`按钮，编辑请求参数，点击`Execute`按钮发送请求。*Responses* 面板中查看请求结果。<br>


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0
- Package version: 1.0
- Generator version: 7.11.0-SNAPSHOT
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**api_v5_rfq_cancel_all_quotes_post**](docs/DefaultApi.md#api_v5_rfq_cancel_all_quotes_post) | **POST** /api/v5/rfq/cancel-all-quotes | 取消所有报价
*DefaultApi* | [**api_v5_rfq_cancel_all_rfqs_post**](docs/DefaultApi.md#api_v5_rfq_cancel_all_rfqs_post) | **POST** /api/v5/rfq/cancel-all-rfqs | 取消所有询价
*DefaultApi* | [**api_v5_rfq_cancel_batch_quotes_post**](docs/DefaultApi.md#api_v5_rfq_cancel_batch_quotes_post) | **POST** /api/v5/rfq/cancel-batch-quotes | 批量取消报价
*DefaultApi* | [**api_v5_rfq_cancel_batch_rfqs_post**](docs/DefaultApi.md#api_v5_rfq_cancel_batch_rfqs_post) | **POST** /api/v5/rfq/cancel-batch-rfqs | 批量取消询价
*DefaultApi* | [**api_v5_rfq_cancel_quote_post**](docs/DefaultApi.md#api_v5_rfq_cancel_quote_post) | **POST** /api/v5/rfq/cancel-quote | 取消报价
*DefaultApi* | [**api_v5_rfq_cancel_rfq_post**](docs/DefaultApi.md#api_v5_rfq_cancel_rfq_post) | **POST** /api/v5/rfq/cancel-rfq | 取消询价
*DefaultApi* | [**api_v5_rfq_counterparties_get**](docs/DefaultApi.md#api_v5_rfq_counterparties_get) | **GET** /api/v5/rfq/counterparties | 获取报价方信息
*DefaultApi* | [**api_v5_rfq_create_quote_post**](docs/DefaultApi.md#api_v5_rfq_create_quote_post) | **POST** /api/v5/rfq/create-quote | 报价
*DefaultApi* | [**api_v5_rfq_create_rfq_post**](docs/DefaultApi.md#api_v5_rfq_create_rfq_post) | **POST** /api/v5/rfq/create-rfq | 询价
*DefaultApi* | [**api_v5_rfq_execute_quote_post**](docs/DefaultApi.md#api_v5_rfq_execute_quote_post) | **POST** /api/v5/rfq/execute-quote | 执行报价
*DefaultApi* | [**api_v5_rfq_maker_instrument_settings_post**](docs/DefaultApi.md#api_v5_rfq_maker_instrument_settings_post) | **POST** /api/v5/rfq/maker-instrument-settings | 设置可报价产品
*DefaultApi* | [**api_v5_rfq_mmp_reset_post**](docs/DefaultApi.md#api_v5_rfq_mmp_reset_post) | **POST** /api/v5/rfq/mmp-reset | 重设MMP状态
*DefaultApi* | [**api_v5_rfq_public_trades_get**](docs/DefaultApi.md#api_v5_rfq_public_trades_get) | **GET** /api/v5/rfq/public-trades | 获取大宗交易公共成交数据
*DefaultApi* | [**api_v5_rfq_quotes_get**](docs/DefaultApi.md#api_v5_rfq_quotes_get) | **GET** /api/v5/rfq/quotes | 获取报价单信息
*DefaultApi* | [**api_v5_rfq_rfqs_get**](docs/DefaultApi.md#api_v5_rfq_rfqs_get) | **GET** /api/v5/rfq/rfqs | 获取询价单信息
*DefaultApi* | [**api_v5_rfq_trades_get**](docs/DefaultApi.md#api_v5_rfq_trades_get) | **GET** /api/v5/rfq/trades | 获取大宗交易信息


## Documentation For Models

 - [ApiV5RfqCancelBatchQuotesPostRequest](docs/ApiV5RfqCancelBatchQuotesPostRequest.md)
 - [ApiV5RfqCancelBatchRfqsPostRequest](docs/ApiV5RfqCancelBatchRfqsPostRequest.md)
 - [ApiV5RfqCancelQuotePostRequest](docs/ApiV5RfqCancelQuotePostRequest.md)
 - [ApiV5RfqCancelRfqPostRequest](docs/ApiV5RfqCancelRfqPostRequest.md)
 - [ApiV5RfqCreateQuotePostRequest](docs/ApiV5RfqCreateQuotePostRequest.md)
 - [ApiV5RfqCreateRfqPostRequest](docs/ApiV5RfqCreateRfqPostRequest.md)
 - [ApiV5RfqExecuteQuotePostRequest](docs/ApiV5RfqExecuteQuotePostRequest.md)
 - [ApiV5RfqMakerInstrumentSettingsPostRequest](docs/ApiV5RfqMakerInstrumentSettingsPostRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



