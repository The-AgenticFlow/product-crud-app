import { useState } from 'react'
import { Link } from 'react-router-dom'
import { Trash } from 'lucide-react'
import { Product } from '../types'
import ConfirmDialog from '../components/ConfirmDialog'
import { useConfirmDialog } from '../hooks/useConfirmDialog'

interface ProductsPageProps {
  initialProducts?: Product[]
}

export default function ProductsPage({ initialProducts = [] }: ProductsPageProps) {
  const [products] = useState<Product[]>(initialProducts)
  const [deletingId, setDeletingId] = useState<string | null>(null)
  const { confirm, dialogProps } = useConfirmDialog()

  const handleDelete = async (product: Product) => {
    const confirmed = await confirm({
      title: 'Delete Product',
      message: `Are you sure you want to delete "${product.name}"? This action cannot be undone.`,
      confirmText: 'Delete',
      cancelText: 'Cancel',
      variant: 'danger'
    })

    if (confirmed) {
      setDeletingId(product.id)
      // Delete operation would happen here
      // For now, we'll just simulate it
      console.log('Deleting product:', product.id)
      setDeletingId(null)
    }
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold text-gray-900">Products</h1>
        <Link
          to="/products/new"
          className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md text-sm font-medium transition-colors"
        >
          Add Product
        </Link>
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
                  <div className="flex items-start gap-4">
                    <div className="text-right">
                      <p className="text-2xl font-bold text-gray-900">
                        ${product.price}
                      </p>
                    </div>
                    <button
                      onClick={() => handleDelete(product)}
                      disabled={deletingId === product.id}
                      className="p-2 text-gray-400 hover:text-red-600 hover:bg-red-50 rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                      aria-label={`Delete ${product.name}`}
                    >
                      <Trash className="w-5 h-5" />
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
      <ConfirmDialog {...dialogProps} />
    </div>
  )
}
