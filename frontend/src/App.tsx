import { useState } from 'react'

interface Product {
  id: string
  name: string
  description: string
  price: number
  stock: number
  category: string
}

function App() {
  const [products] = useState<Product[]>([])

  return (
    <div className="app">
      <h1>Product CRUD App</h1>
      <div className="product-list">
        {products.length === 0 ? (
          <p>No products yet</p>
        ) : (
          products.map(p => (
            <div key={p.id} className="product">
              <h2>{p.name}</h2>
              <p>{p.description}</p>
              <span>${p.price}</span>
            </div>
          ))
        )}
      </div>
    </div>
  )
}

export default App
