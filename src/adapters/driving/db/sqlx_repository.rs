use std::collections::BTreeMap;

use fscl_core::{
	Component,
	ComponentRepositoryPort,
	ResourceId,
};
use sqlx::{Error, Postgres, Row, pool::PoolConnection};

#[derive(Clone)]
pub struct SqlxRepository {
	pool: sqlx::PgPool,
}

impl SqlxRepository {
	pub fn new(pool: sqlx::PgPool) -> Self {
		Self { pool }
	}
}

impl ComponentRepositoryPort for SqlxRepository {
	type Error = Error;
	type Tx<'tx> = PoolConnection<Postgres>;

	fn find(
		&self,
		tx: &mut Self::Tx<'_>,
		id: &ResourceId,
	) -> impl std::future::Future<Output = Result<Option<Component>, Self::Error>> + Send {
		let id = id.as_str().to_string();
		async move {
			let row = sqlx::query("SELECT id, name, description FROM components WHERE id = $1")
			.bind(id)
			.fetch_optional(&mut **tx)
			.await?;

			Ok(row.map(|r| {
				Component::create(
					ResourceId::new(r.get::<String, _>("id")).expect("stored id is valid"),
					r.get::<String, _>("name"),
					r.get::<Option<String>, _>("description"),
					None,
					vec![],
					BTreeMap::new(),
				)
				.expect("stored component is valid")
				.0
			}))
		}
	}

	fn upsert_component(
		&self,
		tx: &mut Self::Tx<'_>,
		component: &Component,
	) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
		let id = component.id().as_str().to_string();
		let name = component.name();
		let description = component.description();
		async move {
			sqlx::query(
				r#"INSERT INTO components (id, name, description, created_at, updated_at)
				   VALUES ($1, $2, $3, now(), now())
				   ON CONFLICT (id) DO UPDATE
				   SET name        = EXCLUDED.name,
					   description = EXCLUDED.description,
					   updated_at  = now()"#,
			)
			.bind(id)
			.bind(name)
			.bind(description)
			.execute(&mut **tx)
			.await?;
			Ok(())
		}
	}

	fn delete_component(
		&self,
		tx: &mut Self::Tx<'_>,
		id: &ResourceId,
	) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
		let id = id.as_str().to_string();
		async move {
			sqlx::query("DELETE FROM components WHERE id = $1")
				.bind(id)
				.execute(&mut **tx)
				.await?;
			Ok(())
		}
	}
}

