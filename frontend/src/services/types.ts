/**
 * API Request and Response Types
 *
 * These types define the shape of API requests and responses
 * for use with React Query hooks and the Axios service layer.
 */

/**
 * Generic paginated response for list endpoints
 * Supports both offset/limit and cursor-based pagination
 */
export interface PaginatedResponse<T> {
  data: T[]
  pagination: {
    total: number
    page: number
    per_page: number
    total_pages: number
  }
}

/**
 * API error response structure
 * Matches common REST API error formats
 */
export interface ApiErrorResponse {
  error: string
  message: string
  statusCode: number
  details?: Record<string, unknown>
}

/**
 * Parameters for getProducts query
 * Supports pagination, filtering, and sorting
 */
export interface GetProductsParams {
  page?: number
  per_page?: number
  category?: string
  search?: string
  sort_by?: 'name' | 'price' | 'created_at'
  sort_order?: 'asc' | 'desc'
}

/**
 * Request body for creating a new product
 * Omits auto-generated fields (id, timestamps)
 */
export interface ProductCreateRequest {
  name: string
  description: string
  price: number
  stock: number
  category: string
}

/**
 * Request body for updating an existing product
 * All fields are optional for partial updates
 */
export interface ProductUpdateRequest {
  name?: string
  description?: string
  price?: number
  stock?: number
  category?: string
}
