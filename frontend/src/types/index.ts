/**
 * Product entity matching backend schema
 * Includes timestamps for audit trail
 */
export interface Product {
  id: string
  name: string
  description?: string
  price: number
  stock: number
  category?: string
  imageUrl?: string
  created_at: string
  updated_at: string
}

export type ProductCreate = Omit<Product, 'id' | 'created_at' | 'updated_at'>
export type ProductUpdate = Partial<ProductCreate>

/**
 * Query parameters for listing products
 * Matches backend API query parameters
 */
export interface ListProductsQuery {
  page?: number
  limit?: number
  search?: string
  category?: string
  sort_by?: 'name' | 'price' | 'created_at' | 'updated_at'
  sort_order?: 'asc' | 'desc'
}

/**
 * Pagination metadata returned by backend
 */
export interface PaginationMeta {
  page: number
  limit: number
  totalItems: number
  totalPages: number
  hasNextPage: boolean
  hasPrevPage: boolean
}

/**
 * Paginated response wrapper for list endpoints
 */
export interface PaginatedResponse<T> {
  data: T[]
  meta: PaginationMeta
}
