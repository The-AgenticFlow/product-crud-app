import { useParams, Link } from 'react-router-dom'
import { ArrowLeft } from 'lucide-react'
import { useProduct } from '../hooks/useProducts'
import ProductDetail from '../components/product/ProductDetail'

export default function ProductDetailPage() {
  const { id } = useParams<{ id: string }>()
  const { data: product, isLoading, error, isError } = useProduct(id ?? '')

  if (!id) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-600">No product ID provided</p>
        <Link
          to="/products"
          className="mt-4 inline-flex items-center text-blue-600 hover:text-blue-800"
        >
          <ArrowLeft className="h-4 w-4 mr-1" />
          Back to Products
        </Link>
      </div>
    )
  }

  if (isLoading) {
    return (
      <div className="flex justify-center items-center py-12">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>
    )
  }

  if (isError) {
    const errorMessage = error instanceof Error ? error.message : 'An error occurred'

    // Check if it's a 404 error
    if (errorMessage.includes('404') || errorMessage.includes('Not Found')) {
      return (
        <div className="text-center py-12">
          <h2 className="text-2xl font-bold text-gray-900 mb-4">Product Not Found</h2>
          <p className="text-gray-600 mb-4">The product you're looking for doesn't exist or has been deleted.</p>
          <Link
            to="/products"
            className="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-md transition-colors"
          >
            <ArrowLeft className="h-4 w-4 mr-1" />
            Back to Products
          </Link>
        </div>
      )
    }

    return (
      <div className="text-center py-12">
        <h2 className="text-2xl font-bold text-red-600 mb-4">Error Loading Product</h2>
        <p className="text-gray-600 mb-4">{errorMessage}</p>
        <Link
          to="/products"
          className="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-md transition-colors"
        >
          <ArrowLeft className="h-4 w-4 mr-1" />
          Back to Products
        </Link>
      </div>
    )
  }

  if (!product) {
    return (
      <div className="text-center py-12">
        <h2 className="text-2xl font-bold text-gray-900 mb-4">Product Not Found</h2>
        <p className="text-gray-600 mb-4">The product you're looking for doesn't exist or has been deleted.</p>
        <Link
          to="/products"
          className="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-md transition-colors"
        >
          <ArrowLeft className="h-4 w-4 mr-1" />
          Back to Products
        </Link>
      </div>
    )
  }

  return (
    <div>
      {/* Back button */}
      <Link
        to="/products"
        className="inline-flex items-center text-blue-600 hover:text-blue-800 mb-6"
      >
        <ArrowLeft className="h-4 w-4 mr-1" />
        Back to Products
      </Link>

      {/* Product details */}
      <ProductDetail product={product} />
    </div>
  )
}
