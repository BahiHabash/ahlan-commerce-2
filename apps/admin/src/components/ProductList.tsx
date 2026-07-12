import { CalendarDays, PackageCheck, PackageX } from 'lucide-react'
import { type Product } from '../hooks/useProducts'

type ProductListProps = {
  products: Product[]
  isLoading: boolean
}

export function ProductList({ products, isLoading }: ProductListProps) {
  if (isLoading) return <div className="state-message">Loading products...</div>
  if (products.length === 0) return <div className="state-message">No products match the current view.</div>

  return (
    <div className="table-container">
      <table className="modern-table">
        <thead>
          <tr>
            <th>Product</th>
            <th>Price</th>
            <th>Inventory</th>
            <th>Status</th>
            <th>Updated</th>
          </tr>
        </thead>
        <tbody>
          {products.map((product) => (
            <tr key={product.id}>
              <td>
                <div className="product-cell">
                  <strong>{product.title}</strong>
                  <span>{product.handle}</span>
                </div>
              </td>
              <td>{formatMoney(product.priceCents)}</td>
              <td>
                <span className={`inventory ${product.inventoryQuantity > 0 ? 'available' : 'empty'}`}>
                  {product.inventoryQuantity > 0 ? <PackageCheck size={16} /> : <PackageX size={16} />}
                  {product.inventoryQuantity.toLocaleString()}
                </span>
              </td>
              <td>
                <span className={`status-pill ${product.published ? 'published' : 'draft'}`}>
                  {product.published ? 'Published' : 'Draft'}
                </span>
              </td>
              <td>
                <span className="date-cell">
                  <CalendarDays size={15} />
                  {new Date(product.updatedAt).toLocaleDateString()}
                </span>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

function formatMoney(cents: number) {
  return (cents / 100).toLocaleString('en-US', {
    style: 'currency',
    currency: 'USD',
  })
}
