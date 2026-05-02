export default function HomePage() {
  return (
    <div className="space-y-6">
      <div className="text-center">
        <h1 className="text-4xl font-bold text-gray-900 mb-4">
          Welcome to Product CRUD App
        </h1>
        <p className="text-lg text-gray-600">
          A simple application to manage your products
        </p>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mt-8">
        <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
          <h2 className="text-xl font-semibold text-gray-900 mb-2">
            View Products
          </h2>
          <p className="text-gray-600">
            Browse and search through your product catalog
          </p>
        </div>
        <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
          <h2 className="text-xl font-semibold text-gray-900 mb-2">
            Add Products
          </h2>
          <p className="text-gray-600">
            Create new products with detailed information
          </p>
        </div>
        <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
          <h2 className="text-xl font-semibold text-gray-900 mb-2">
            Manage Inventory
          </h2>
          <p className="text-gray-600">
            Update stock levels and product details
          </p>
        </div>
      </div>
    </div>
  )
}
