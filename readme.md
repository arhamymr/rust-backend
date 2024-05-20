# [experimental project] Create API for an ecommerce platform

API Spesification

### 1. Authentication

- Generate Client Id And Client Secret

  - Endpoint: POST /generateCredentials
  - Request Body:
    - `name`: name of the client
  - Response
    - `client_id` : [Go to Client ID](#12-client-id)
    - `client_secret`: [Go to Client Secret](#13-client-secret)
    - `registered_at`: The timestamp when client was registered

-

### 2. Authorization [RFC 6749]

API using RFC 6749 -> https://datatracker.ietf.org/doc/html/rfc6749

- Access Token Request

  Endpoint: POST /token
  Request Body:

  - `client_id`: [Go to Client ID](#12-client-id)
  - `client_secret`: [Go to Client Secret](#13-client-secret)
  - `grant_type`: [Go to Grant-Type](#14-grant-type)

  response:

  - `access_token`: The access token that the client can use to authenticate subsequent API requests.
  - `expires_in`: The lifetime in seconds of the access token.
    expired time depend on how high security is it can be 15 minutes for high-security application or 7 hours for low-security application (implement 1 hour)
  - `token_type`: The type of the token issued (typically "Bearer").

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

is a unique identifier issued by the server when a new client (application) is registered. It's used to identify the client in subsequent requests to the server

### 13. CLIENT SECRET

TODO !!

### 14. GRANT-TYPE

- Authorization code
- Password (Resource Owner Password Crendentials)
- Refresh Token
- Client Credentials

### 15. DATABASE DESIGN

- clients
  - client-id
  - client-secret
  - name
  - created-date
