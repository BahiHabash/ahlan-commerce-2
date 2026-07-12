create table if not exists products (
  id uuid primary key,
  title text not null,
  handle text not null unique,
  description text not null default '',
  price_cents integer not null check (price_cents >= 0),
  inventory_quantity integer not null check (inventory_quantity >= 0),
  published boolean not null default false,
  published_at timestamptz,
  created_at timestamptz not null,
  updated_at timestamptz not null
);

create table if not exists import_jobs (
  id uuid primary key,
  input_path text not null,
  status text not null check (status in ('queued', 'running', 'succeeded', 'failed')),
  error_message text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
