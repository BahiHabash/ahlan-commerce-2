import { createFileRoute } from '@tanstack/react-router'
import { useMemo, useState } from 'react'
import { AlertCircle, Archive, Boxes, CheckCircle2, DollarSign, Search } from 'lucide-react'
import { ProductList } from '../components/ProductList'
import { ProductForm } from '../components/ProductForm'
import { useProducts } from '../hooks/useProducts'

export const Route = createFileRoute('/products')({
  component: Products,
})

function Products() {
  const { data: products = [], isLoading, isError, error } = useProducts()
  const [query, setQuery] = useState('')
  const [status, setStatus] = useState<'all' | 'published' | 'draft'>('all')

  const filteredProducts = useMemo(() => {
    const normalized = query.trim().toLowerCase()
    return products.filter((product) => {
      const matchesSearch =
        !normalized ||
        product.title.toLowerCase().includes(normalized) ||
        product.handle.toLowerCase().includes(normalized)
      const matchesStatus =
        status === 'all' ||
        (status === 'published' && product.published) ||
        (status === 'draft' && !product.published)
      return matchesSearch && matchesStatus
    })
  }, [products, query, status])

  const publishedCount = products.filter((product) => product.published).length
  const inventoryCount = products.reduce((sum, product) => sum + product.inventoryQuantity, 0)
  const catalogValue = products.reduce(
    (sum, product) => sum + product.priceCents * product.inventoryQuantity,
    0,
  )

  return (
    <main className="page">
      <section className="page-header">
        <div>
          <p className="eyebrow">Catalog</p>
          <h1>Products</h1>
          <p className="page-subtitle">
            Create, publish, and audit the product data served by the storefront.
          </p>
        </div>
      </section>

      <section className="metric-grid">
        <div className="metric-panel">
          <Boxes size={22} />
          <span>Total products</span>
          <strong>{products.length}</strong>
        </div>
        <div className="metric-panel">
          <CheckCircle2 size={22} />
          <span>Published</span>
          <strong>{publishedCount}</strong>
        </div>
        <div className="metric-panel">
          <Archive size={22} />
          <span>Inventory units</span>
          <strong>{inventoryCount.toLocaleString()}</strong>
        </div>
        <div className="metric-panel">
          <DollarSign size={22} />
          <span>Inventory value</span>
          <strong>{formatMoney(catalogValue)}</strong>
        </div>
      </section>

      <section className="workspace-grid">
        <div className="panel table-panel">
          <div className="panel-header">
            <div>
              <h2>Product registry</h2>
              <p>{filteredProducts.length} matching records</p>
            </div>
            <div className="toolbar">
              <label className="search-control">
                <Search size={17} />
                <input
                  value={query}
                  onChange={(event) => setQuery(event.target.value)}
                  placeholder="Search title or handle"
                />
              </label>
              <div className="segmented-control" aria-label="Publication status">
                <button className={status === 'all' ? 'active' : ''} onClick={() => setStatus('all')}>
                  All
                </button>
                <button
                  className={status === 'published' ? 'active' : ''}
                  onClick={() => setStatus('published')}
                >
                  Published
                </button>
                <button className={status === 'draft' ? 'active' : ''} onClick={() => setStatus('draft')}>
                  Draft
                </button>
              </div>
            </div>
          </div>

          {isError ? (
            <div className="state-message error">
              <AlertCircle size={18} />
              Error loading products: {error.message}
            </div>
          ) : (
            <ProductList products={filteredProducts} isLoading={isLoading} />
          )}
        </div>

        <aside className="panel form-panel">
          <ProductForm />
        </aside>
      </section>
    </main>
  )
}

function formatMoney(cents: number) {
  return (cents / 100).toLocaleString('en-US', {
    style: 'currency',
    currency: 'USD',
    maximumFractionDigits: 0,
  })
}
