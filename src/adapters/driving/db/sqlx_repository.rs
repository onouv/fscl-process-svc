use std::collections::BTreeMap;

use fscl_core::{
	IdFormat,
	Component,
	ComponentRepositoryPort,
	ProjectId,
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
		let id = id.to_string();
		async move {
			let row = sqlx::query("SELECT id, name, description FROM components WHERE id = $1")
			.bind(id)
			.fetch_optional(&mut **tx)
			.await?;

			Ok(row.map(|r| {
				let stored_id = r.get::<String, _>("id");
				let (project, local) = stored_id
					.split_once(':')
					.expect("stored id is expected as '<project>:<local>'");
				let project_id = ProjectId::new(project.to_string()).expect("stored project id is valid");
				let format = IdFormat::new(None, None, None).expect("default format is valid");

				Component::create(
					ResourceId::new(project_id, local.to_string(), format).expect("stored id is valid"),
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
		let id = component.id().to_string();
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
		let id = id.to_string();
		async move {
			sqlx::query("DELETE FROM components WHERE id = $1")
				.bind(id)
				.execute(&mut **tx)
				.await?;
			Ok(())
		}
	}
}

