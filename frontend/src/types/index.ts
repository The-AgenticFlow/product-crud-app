export interface Product {
  id: string
  name: string
  description: string
  price: number
  stock: number
  category: string
  imageUrl?: string
}

export type ProductCreate = Omit<Product, 'id'>
export type ProductUpdate = Partial<ProductCreate>
