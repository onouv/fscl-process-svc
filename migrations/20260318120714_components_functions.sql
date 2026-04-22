CREATE TABLE IF NOT EXISTS functions (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  parent_id TEXT REFERENCES functions(id) ON DELETE CASCADE,
  version INT NOT NULL DEFAULT 1,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS components (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  parent_id TEXT REFERENCES components(id) ON DELETE CASCADE,
  version INT NOT NULL DEFAULT 1,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS component_implements_function (
  id TEXT PRIMARY KEY,
  component_id TEXT NOT NULL REFERENCES components(id) ON DELETE CASCADE,
  function_id TEXT NOT NULL REFERENCES functions(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  UNIQUE(component_id, function_id)
);

CREATE INDEX idx_functions_parent_created ON functions(parent_id, created_at);
CREATE INDEX idx_components_parent_created ON components(parent_id, created_at);
CREATE INDEX idx_impl_component ON component_implements_function(component_id);