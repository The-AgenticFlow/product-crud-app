import api from './api'
import { Product, ProductCreate, ProductUpdate, ListProductsQuery, PaginatedResponse } from '../types'

export const productService = {
  async createProduct(data: ProductCreate): Promise<Product> {
    const response = await api.post<Product>('/products', data)
    return response.data
  },

  async updateProduct(id: string, data: ProductUpdate): Promise<Product> {
    const response = await api.put<Product>(`/products/${id}`, data)
    return response.data
  },

  async getProduct(id: string): Promise<Product> {
    const response = await api.get<Product>(`/products/${id}`)
    return response.data
  },

  async listProducts(query?: ListProductsQuery): Promise<PaginatedResponse<Product>> {
    const params = new URLSearchParams()

    if (query?.page) params.append('page', String(query.page))
    if (query?.limit) params.append('limit', String(query.limit))
    if (query?.search) params.append('search', query.search)
    if (query?.category) params.append('category', query.category)
    if (query?.sort_by) params.append('sort_by', query.sort_by)
    if (query?.sort_order) params.append('sort_order', query.sort_order)

    const response = await api.get<PaginatedResponse<Product>>(`/products?${params.toString()}`)
    return response.data
  },

  async deleteProduct(id: string): Promise<void> {
    await api.delete(`/products/${id}`)
  },
}
