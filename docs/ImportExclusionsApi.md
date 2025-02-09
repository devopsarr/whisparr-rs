# \ImportExclusionsApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_exclusions**](ImportExclusionsApi.md#create_exclusions) | **POST** /api/v3/exclusions | 
[**create_exclusions_bulk**](ImportExclusionsApi.md#create_exclusions_bulk) | **POST** /api/v3/exclusions/bulk | 
[**delete_exclusions**](ImportExclusionsApi.md#delete_exclusions) | **DELETE** /api/v3/exclusions/{id} | 
[**get_exclusions_by_id**](ImportExclusionsApi.md#get_exclusions_by_id) | **GET** /api/v3/exclusions/{id} | 
[**list_exclusions**](ImportExclusionsApi.md#list_exclusions) | **GET** /api/v3/exclusions | 
[**update_exclusions**](ImportExclusionsApi.md#update_exclusions) | **PUT** /api/v3/exclusions/{id} | 



## create_exclusions

> models::ImportExclusionsResource create_exclusions(import_exclusions_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_exclusions_resource** | Option<[**ImportExclusionsResource**](ImportExclusionsResource.md)> |  |  |

### Return type

[**models::ImportExclusionsResource**](ImportExclusionsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_exclusions_bulk

> create_exclusions_bulk(import_exclusions_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_exclusions_resource** | Option<[**Vec<models::ImportExclusionsResource>**](ImportExclusionsResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_exclusions

> delete_exclusions(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exclusions_by_id

> models::ImportExclusionsResource get_exclusions_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ImportExclusionsResource**](ImportExclusionsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_exclusions

> Vec<models::ImportExclusionsResource> list_exclusions()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ImportExclusionsResource>**](ImportExclusionsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_exclusions

> models::ImportExclusionsResource update_exclusions(id, import_exclusions_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**import_exclusions_resource** | Option<[**ImportExclusionsResource**](ImportExclusionsResource.md)> |  |  |

### Return type

[**models::ImportExclusionsResource**](ImportExclusionsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

