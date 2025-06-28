# ProtectedApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**adminDashboard**](#admindashboard) | **GET** /admin/dashboard | GET /admin/dashboard Returns system stats and list of users — only accessible by Admins.|
|[**registerAdmin**](#registeradmin) | **POST** /admin/register | POST /admin/register Allows Admin to create a new Admin user.|
|[**userProfile**](#userprofile) | **GET** /user/profile | GET /user/profile Returns the authenticated user\&#39;s profile info — accessible by both Users and Admins.|

# **adminDashboard**
> UserResponse adminDashboard()


### Example

```typescript
import {
    ProtectedApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ProtectedApi(configuration);

const { status, data } = await apiInstance.adminDashboard();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**UserResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Admin dashboard with user stats |  -  |
|**401** | Unauthorized - Invalid or missing token |  -  |
|**403** | Forbidden - Admin access required |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **registerAdmin**
> UserResponse registerAdmin(registerRequest)


### Example

```typescript
import {
    ProtectedApi,
    Configuration,
    RegisterRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new ProtectedApi(configuration);

let registerRequest: RegisterRequest; //

const { status, data } = await apiInstance.registerAdmin(
    registerRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **registerRequest** | **RegisterRequest**|  | |


### Return type

**UserResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Admin user created |  -  |
|**400** | Bad request - Validation error |  -  |
|**401** | Unauthorized - Invalid or missing token |  -  |
|**403** | Forbidden - Admin access required |  -  |
|**409** | Conflict - Email already registered |  -  |
|**500** | Internal Server Error - Hash failure |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **userProfile**
> UserResponse userProfile()


### Example

```typescript
import {
    ProtectedApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ProtectedApi(configuration);

const { status, data } = await apiInstance.userProfile();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**UserResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | User profile info |  -  |
|**400** | Bad request - Invalid user ID |  -  |
|**401** | Unauthorized - Invalid or missing token |  -  |
|**403** | Forbidden - Authentication required |  -  |
|**404** | Not Found - User not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

