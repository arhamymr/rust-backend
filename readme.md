# [experimental project] Create API for an ecommerce platform

API Spesification

### 1. Authentication

- Client Registration

  - Endpoint: POST /register
  - Request Body:
    - `name`: name of the client
  - Response
    - `client_id` : [Go to Client ID](#12-client-id)
    - `client_secret`: The secret key for the client
    - `registered_at`: The timestamp when client was registered

-

### 2. Authorization [RFC 6749]

API using RFC 6749 -> https://datatracker.ietf.org/doc/html/rfc6749

- Access Token Request

  Endpoint: POST /token
  Request Body:

  - `client_id`:
  - `client_secret`:
  - `grant_type:
  - The API Server validates the client credentials and issues an access token if authentication is successful
  - Client include the access token in Authorization header of subsequent API requests to access protected resources

- Grant-type

  - Authorization code
    TODO !!
  - Password (Resource Owner Password Crendentials)
    Endpont

    POST /login
    Content-Type:

    Response:

  - Refresh Token
    TODO !!
  - Client Credentials
    TODO !!

- Scope

### 3. Product Management

### 4. Orders

### 5. Customers

### 6. Payments

### 7. Reviews and Ratings

### 8. Search and Filterst

### 9. Analytic and Reporting

### 10. Error Handling and Validation

### 11. Documentatio and Testing

### 12. Client ID
