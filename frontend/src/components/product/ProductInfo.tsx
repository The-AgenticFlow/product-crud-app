import { Tag, Calendar, Clock } from 'lucide-react'
import type { Product } from '../../types'
import StockIndicator from './StockIndicator'
import ProductImage from './ProductImage'

export interface ProductInfoProps {
  product: Product
  showImage?: boolean
  imageClassName?: string
}

function formatCurrency(price: number): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
  }).format(price)
}

function formatDate(dateString: string): string {
  const date = new Date(dateString)
  return new Intl.DateTimeFormat('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  }).format(date)
}

export default function ProductInfo({
  product,
  showImage = true,
  imageClassName = '',
}: ProductInfoProps) {
  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
      {/* Image Section */}
      {showImage && (
        <div>
          <ProductImage
            imageUrl={product.imageUrl}
            name={product.name}
            className={imageClassName}
          />
        </div>
      )}

      {/* Info Section */}
      <div className="space-y-6">
        {/* Name */}
        <h1 className="text-3xl font-bold text-gray-900">{product.name}</h1>

        {/* Price */}
        <div className="text-3xl font-bold text-blue-600">
          {formatCurrency(product.price)}
        </div>

        {/* Stock Indicator */}
        <div>
          <StockIndicator stock={product.stock} />
        </div>

        {/* Category Badge */}
        {product.category && (
          <div className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-sm font-medium bg-blue-100 text-blue-800">
            <Tag className="h-4 w-4" />
            <span>{product.category}</span>
          </div>
        )}

        {/* Description */}
        {product.description && (
          <div>
            <h2 className="text-lg font-semibold text-gray-900 mb-2">Description</h2>
            <p className="text-gray-600 leading-relaxed">{product.description}</p>
          </div>
        )}

        {/* Timestamps */}
        <div className="border-t pt-6 space-y-3 text-sm text-gray-500">
          <div className="flex items-center gap-2">
            <Calendar className="h-4 w-4" />
            <span>Created: {formatDate(product.created_at)}</span>
          </div>
          <div className="flex items-center gap-2">
            <Clock className="h-4 w-4" />
            <span>Updated: {formatDate(product.updated_at)}</span>
          </div>
        </div>
      </div>
    </div>
  )
}
