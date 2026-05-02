/**
 * Product API Service Functions
 *
 * CRUD operations for products using the configured Axios instance.
 * All functions are typed for React Query compatibility.
 */

import apiClient from './api'
import { Product } from '../types'
import {
  PaginatedResponse,
  GetProductsParams,
  ProductCreateRequest,
  ProductUpdateRequest,
} from './types'

/**
 * Fetch paginated list of products with optional filtering and sorting
 * @param params - Query parameters for pagination, filtering, and sorting
 * @returns Promise resolving to paginated product list
 */
export async function getProducts(
  params?: GetProductsParams
): Promise<PaginatedResponse<Product>> {
  const response = await apiClient.get<PaginatedResponse<Product>>('/api/products', {
    params,
  })
  return response.data
}

/**
 * Fetch a single product by ID
 * @param id - Product ID
 * @returns Promise resolving to product data
 */
export async function getProduct(id: string): Promise<Product> {
  const response = await apiClient.get<Product>(`/api/products/${id}`)
  return response.data
}

/**
 * Create a new product
 * @param data - Product creation data
 * @returns Promise resolving to created product
 */
export async function createProduct(data: ProductCreateRequest): Promise<Product> {
  const response = await apiClient.post<Product>('/api/products', data)
  return response.data
}

/**
 * Update an existing product
 * @param id - Product ID
 * @param data - Partial product update data
 * @returns Promise resolving to updated product
 */
export async function updateProduct(
  id: string,
  data: ProductUpdateRequest
): Promise<Product> {
  const response = await apiClient.put<Product>(`/api/products/${id}`, data)
  return response.data
}

/**
 * Delete a product by ID
 * @param id - Product ID
 * @returns Promise resolving when deletion is complete
 */
export async function deleteProduct(id: string): Promise<void> {
  await apiClient.delete(`/api/products/${id}`)
}
