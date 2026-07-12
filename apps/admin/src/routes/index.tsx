import { createFileRoute } from '@tanstack/react-router'
import { ArrowRight, Boxes, Gauge, RadioTower } from 'lucide-react'

export const Route = createFileRoute('/')({
  component: Index,
})

function Index() {
  return (
    <main className="page">
      <section className="page-header">
        <div>
          <p className="eyebrow">Store operations</p>
          <h1>Catalog control center</h1>
          <p className="page-subtitle">
            Manage product records, publication status, inventory readiness, and storefront data from one workflow.
          </p>
        </div>
        <a className="primary-action" href="/products">
          <Boxes size={18} />
          Open Products
          <ArrowRight size={16} />
        </a>
      </section>

      <section className="overview-grid">
        <div className="metric-panel">
          <Gauge size={22} />
          <span>Runtime</span>
          <strong>Connected</strong>
          <small>API and admin routes are configured.</small>
        </div>
        <div className="metric-panel">
          <RadioTower size={22} />
          <span>GraphQL</span>
          <strong>Active</strong>
          <small>Product list and create flows use the API boundary.</small>
        </div>
        <div className="metric-panel">
          <Boxes size={22} />
          <span>Catalog</span>
          <strong>Ready</strong>
          <small>Create products and review storefront publishing state.</small>
        </div>
      </section>
    </main>
  )
}
