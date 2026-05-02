import { AlertTriangle, Package } from 'lucide-react'

export interface StockIndicatorProps {
  stock: number
  lowStockThreshold?: number
}

export default function StockIndicator({
  stock,
  lowStockThreshold = 10,
}: StockIndicatorProps) {
  const isOutOfStock = stock === 0
  const isLowStock = stock > 0 && stock < lowStockThreshold

  if (isOutOfStock) {
    return (
      <div className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-sm font-medium bg-red-100 text-red-800">
        <AlertTriangle className="h-4 w-4" />
        <span>Out of Stock</span>
      </div>
    )
  }

  if (isLowStock) {
    return (
      <div className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-sm font-medium bg-yellow-100 text-yellow-800">
        <AlertTriangle className="h-4 w-4" />
        <span>Low Stock: {stock}</span>
      </div>
    )
  }

  return (
    <div className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-sm font-medium bg-green-100 text-green-800">
      <Package className="h-4 w-4" />
      <span>In Stock: {stock}</span>
    </div>
  )
}
