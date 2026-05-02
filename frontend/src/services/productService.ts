import api from './api'
import { Product, ProductCreate, ProductUpdate } from '../types'

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

  async deleteProduct(id: string): Promise<void> {
    await api.delete(`/products/${id}`)
  },
}
