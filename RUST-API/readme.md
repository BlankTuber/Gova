# Rust User API

## Installation

- Clone repository
- Go to "userspace" folder
- Cargo build --release

## API Endpoint documentation

### Authentication Endpoints

#### Login

- **URL**: `/api/auth/login`
- **Method**: `POST`
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "email": "string",     // Valid email address
    "password": "string"   // Minimum 8 characters
}
```

- **Success Response**:
  - **Code**: 200
  - **Content**: Authentication token (jwt)

#### Logout

- **URL**: `/api/auth/logout`
- **Method**: `POST`
- **Authentication**: Required
- **Success Response**:
  - **Code**: 200

#### Register

- **URL**: `/api/auth/register`
- **Method**: `POST`
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "email": "string",     // Valid email address
    "username": "string",  // 3-30 characters
    "password": "string"   // Minimum 8 characters
}
```

- **Success Response**:
  - **Code**: 201
  - **Content**: User object (without password hash)

### User Management Endpoints

#### Get Current User

- **URL**: `/api/users/`
- **Method**: `GET`
- **Authentication**: Required
- **Success Response**:
  - **Code**: 200
  - **Content**: User object

#### Update Email

- **URL**: `/api/users/email`
- **Method**: `PUT`
- **Authentication**: Required
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "email": "string"  // Valid email address
}
```

#### Update Username

- **URL**: `/api/users/username`
- **Method**: `PUT`
- **Authentication**: Required
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "username": "string"  // 3-30 characters
}
```

#### Update Profile

- **URL**: `/api/users/profile`
- **Method**: `PUT`
- **Authentication**: Required
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "display_name": "string",  // Optional, max 100 characters
    "bio": "string",          // Optional
    "birth_date": "string",   // Optional, ISO 8601 date format
    "language": "string",     // Optional, max 10 characters
    "timezone": "string",     // Optional, max 50 characters
    "social_links": {}        // Optional, JSON object
}
```

### Administrative Endpoints

#### Roles

##### Create Role

- **URL**: `/api/admin/role`
- **Method**: `POST`
- **Authentication**: Required (Admin)
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "name": "string"
}
```

###### Delete Role

- **URL**: `/api/admin/role`
- **Method**: `DELETE`
- **Authentication**: Required (Admin)
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "id": "uuid"
}
```

###### Get All Roles

- **URL**: `/api/admin/roles`
- **Method**: `GET`
- **Authentication**: Required (Admin)
- **Success Response**:
  - **Code**: 200
  - **Content**: Array of Role objects

#### Users

##### Get All Users

- **URL**: `/api/admin/users`
- **Method**: `GET`
- **Authentication**: Required (Admin)

###### Delete User

- **URL**: `/api/admin/user`
- **Method**: `DELETE`
- **Authentication**: Required (Admin)
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "id": "uuid"
}
```

#### Permissions

##### Create Permission

- **URL**: `/api/admin/permission`
- **Method**: `POST`
- **Authentication**: Required (Admin)
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "name": "string"  // 3-50 characters
}
```

###### Delete Permission

- **URL**: `/api/admin/permission`
- **Method**: `DELETE`
- **Authentication**: Required (Admin)
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "id": "uuid"
}
```

###### Get All Permissions

- **URL**: `/api/admin/permissions`
- **Method**: `GET`
- **Authentication**: Required (Admin)

#### Role Management

##### Assign Role to User

- **URL**: `/api/admin/role/user`
- **Method**: `POST`
- **Authentication**: Required (Admin)
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "user_id": "uuid",
    "role_id": "uuid"
}
```

###### Remove User from Role

- **URL**: `/api/admin/role/user`
- **Method**: `DELETE`
- **Authentication**: Required (Admin)
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "user_id": "uuid",
    "role_id": "uuid"
}
```

###### Assign Permission to Role

- **URL**: `/api/admin/permission/role`
- **Method**: `POST`
- **Authentication**: Required (Admin)
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
    "role_id": "uuid",
    "permission_id": "uuid"
}
```

### Response Objects

#### User Object

```json
{
    "id": "uuid",
    "email": "string",
    "username": "string",
    "created_at": "string",    // ISO 8601 datetime
    "updated_at": "string",    // ISO 8601 datetime
    "last_login": "string"     // ISO 8601 datetime, optional
}
```

#### Role Object

```json
{
    "id": "uuid",
    "name": "string",
    "created_at": "string"     // ISO 8601 datetime
}
```

#### Permission Object

```json
{
    "id": "uuid",
    "name": "string",
    "created_at": "string"     // ISO 8601 datetime
}
```

#### UserProfile Object

```json
{
    "id": "uuid",
    "user_id": "uuid",
    "display_name": "string",  // optional
    "bio": "string",          // optional
    "birth_date": "string",   // ISO 8601 date, optional
    "language": "string",     // optional
    "timezone": "string",     // optional
    "social_links": {},       // JSON object
    "created_at": "string",   // ISO 8601 datetime
    "updated_at": "string"    // ISO 8601 datetime
}
```

### Error Responses

All endpoints can return the following error responses:

- **401 Unauthorized**: Authentication is required or invalid
- **403 Forbidden**: User doesn't have required permissions
- **404 Not Found**: Requested resource doesn't exist
- **422 Unprocessable Entity**: Invalid request body or parameters
- **500 Internal Server Error**: Server-side error

### Notes

1. All endpoints that require authentication expect a valid authentication token (jwt)
2. Admin endpoints require both authentication and appropriate admin permissions
3. All datetime fields follow ISO 8601 format
4. All IDs are UUID v4 format
5. Request bodies must be valid JSON with Content-Type header set to `application/json`
