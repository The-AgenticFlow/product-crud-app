import { useParams, useNavigate } from 'react-router-dom'
import { useProduct } from '../hooks/useProducts'
import ProductForm from '../components/ProductForm'

export default function EditProductPage() {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const { data: product, isLoading, error } = useProduct(id || '')

  const handleSuccess = () => {
    navigate('/products')
  }

  if (isLoading) {
    return (
      <div className="space-y-6">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Edit Product</h1>
        </div>
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-12">
          <div className="text-center">
            <p className="text-gray-500">Loading product...</p>
          </div>
        </div>
      </div>
    )
  }

  if (error || !product) {
    return (
      <div className="space-y-6">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Edit Product</h1>
        </div>
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-12">
          <div className="text-center">
            <p className="text-red-600">Error loading product.</p>
            <button
              onClick={() => navigate('/products')}
              className="mt-4 text-blue-600 hover:text-blue-800"
            >
              Back to Products
            </button>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-gray-900">Edit Product</h1>
        <p className="mt-2 text-sm text-gray-600">
          Update the details for {product.name}.
        </p>
      </div>
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <ProductForm
          mode="edit"
          productId={id}
          initialData={product}
          onSuccess={handleSuccess}
        />
      </div>
    </div>
  )
}
