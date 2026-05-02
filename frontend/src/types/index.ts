/**
 * Product entity matching backend schema
 * Includes timestamps for audit trail
 */
export interface Product {
  id: string
  name: string
  description: string
  price: number
  stock: number
  category: string
  created_at: string
  updated_at: string
}
