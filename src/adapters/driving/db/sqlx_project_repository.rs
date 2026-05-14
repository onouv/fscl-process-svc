use fscl_core::{IdFormat, Project, ProjectId, ProjectRepositoryPort};
use sqlx::{Error, Postgres, Row, pool::PoolConnection};

#[derive(Clone)]
pub struct SqlxProjectRepository;

impl SqlxProjectRepository {
    pub fn new() -> Self {
        Self
    }
}

impl ProjectRepositoryPort for SqlxProjectRepository {
    type Error = Error;
    type Tx<'tx> = PoolConnection<Postgres>;

    fn find_project(
        &self,
        tx: &mut Self::Tx<'_>,
        id: &ProjectId,
    ) -> impl std::future::Future<Output = Result<Option<Project>, Self::Error>> + Send {
        let id = id.as_str().to_string();

        async move {
            let row = sqlx::query("SELECT id, name, description FROM projects WHERE id = $1")
                .bind(id)
                .fetch_optional(&mut **tx)
                .await?;

            Ok(row.map(|r| {
                let project_id =
                    ProjectId::new(r.get::<String, _>("id")).expect("stored project id is valid");
                let format = IdFormat::new(None, None, None).expect("default format is valid");

                Project::new(
                    project_id,
                    r.get::<String, _>("name"),
                    r.get::<Option<String>, _>("description"),
                    format,
                )
                .expect("stored project row is valid")
            }))
        }
    }

    fn save_project(
        &self,
        tx: &mut Self::Tx<'_>,
        project: &Project,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let id = project.id.as_str().to_string();
        let name = project.name.clone();
        let description = project.description.clone();

        async move {
            sqlx::query(
                r#"INSERT INTO projects (id, name, description, created_at, updated_at)
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
}
