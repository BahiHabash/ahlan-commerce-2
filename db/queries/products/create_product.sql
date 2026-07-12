--! create_product
insert into products (
  id,
  title,
  handle,
  description,
  price_cents,
  inventory_quantity,
  published,
  published_at,
  created_at,
  updated_at
) values (
  :id,
  :title,
  :handle,
  :description,
  :price_cents,
  :inventory_quantity,
  :published,
  :published_at,
  :created_at,
  :updated_at
)
returning
  id,
  title,
  handle,
  description,
  price_cents,
  inventory_quantity,
  published,
  published_at,
  created_at,
  updated_at;
