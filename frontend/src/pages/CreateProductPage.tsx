import { useNavigate } from 'react-router-dom'
import ProductForm from '../components/ProductForm'

export default function CreateProductPage() {
  const navigate = useNavigate()

  const handleSuccess = () => {
    navigate('/products')
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-gray-900">Create Product</h1>
        <p className="mt-2 text-sm text-gray-600">
          Fill in the details below to add a new product to your inventory.
        </p>
      </div>
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <ProductForm mode="create" onSuccess={handleSuccess} />
      </div>
    </div>
  )
}
