import { useState } from 'react'
import { Product } from '../types'

export default function ProductsPage() {
  const [products] = useState<Product[]>([])

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold text-gray-900">Products</h1>
        <button className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md text-sm font-medium transition-colors">
          Add Product
        </button>
      </div>
      <div className="bg-white rounded-lg shadow-sm border border-gray-200">
        {products.length === 0 ? (
          <div className="p-12 text-center">
            <p className="text-gray-500 text-lg">No products yet</p>
            <p className="text-gray-400 text-sm mt-2">
              Click "Add Product" to create your first product
            </p>
          </div>
        ) : (
          <div className="divide-y divide-gray-200">
            {products.map((product) => (
              <div key={product.id} className="p-6 hover:bg-gray-50">
                <div className="flex justify-between items-start">
                  <div>
                    <h3 className="text-lg font-semibold text-gray-900">
                      {product.name}
                    </h3>
                    <p className="text-gray-600 mt-1">{product.description}</p>
                    <div className="flex gap-4 mt-2 text-sm text-gray-500">
                      <span>Category: {product.category}</span>
                      <span>Stock: {product.stock}</span>
                    </div>
                  </div>
                  <div className="text-right">
                    <p className="text-2xl font-bold text-gray-900">
                      ${product.price}
                    </p>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
