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
