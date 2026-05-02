import { Link } from 'react-router-dom'
import { Edit, Trash2 } from 'lucide-react'
import type { Product } from '../../types'
import ProductInfo from './ProductInfo'
import ConfirmDialog from '../ConfirmDialog'
import { useConfirmDialog } from '../../hooks/useConfirmDialog'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { productService } from '../../services/productService'

export interface ProductDetailProps {
  product: Product
}

export default function ProductDetail({ product }: ProductDetailProps) {
  const queryClient = useQueryClient()
  const { confirm, dialogProps } = useConfirmDialog()

  const deleteMutation = useMutation({
    mutationFn: (id: string) => productService.deleteProduct(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['products'] })
      // Navigate to products list after successful deletion
      window.location.href = '/products'
    },
  })

  const handleDelete = async () => {
    const confirmed = await confirm({
      title: 'Delete Product',
      message: `Are you sure you want to delete "${product.name}"? This action cannot be undone.`,
      confirmText: 'Delete',
      cancelText: 'Cancel',
      variant: 'danger',
    })

    if (confirmed) {
      deleteMutation.mutate(product.id)
    }
  }

  return (
    <div>
      {/* Product Info */}
      <div className="bg-white rounded-lg shadow-md p-6 mb-6">
        <ProductInfo product={product} />
      </div>

      {/* Action Buttons */}
      <div className="flex gap-3">
        <Link
          to={`/products/${product.id}/edit`}
          className="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-md transition-colors"
        >
          <Edit className="h-4 w-4 mr-2" />
          Edit Product
        </Link>

        <button
          onClick={handleDelete}
          disabled={deleteMutation.isPending}
          className="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-red-600 hover:bg-red-700 rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <Trash2 className="h-4 w-4 mr-2" />
          {deleteMutation.isPending ? 'Deleting...' : 'Delete Product'}
        </button>
      </div>

      {/* Confirmation Dialog */}
      <ConfirmDialog {...dialogProps} />
    </div>
  )
}
