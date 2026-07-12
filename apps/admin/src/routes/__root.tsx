import { createRootRoute, Link, Outlet } from '@tanstack/react-router'
import { BarChart3, Boxes, Store } from 'lucide-react'

export const Route = createRootRoute({
  component: () => (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand-lockup">
          <div className="brand-mark">
            <Store size={20} />
          </div>
          <div>
            <strong>Ahlan Commerce</strong>
            <span>Catalog admin</span>
          </div>
        </div>

        <nav className="side-nav" aria-label="Primary">
          <Link to="/" className="nav-link">
            <BarChart3 size={18} />
            Overview
          </Link>
          <Link to="/products" className="nav-link">
            <Boxes size={18} />
            Products
          </Link>
        </nav>
      </aside>

      <div className="main-column">
        <Outlet />
      </div>
    </div>
  ),
})
