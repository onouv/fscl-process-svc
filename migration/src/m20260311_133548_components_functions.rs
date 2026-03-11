use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Functions::Table)
                    .if_not_exists()
                    .col(string(Functions::Id).primary_key())
                    .col(string(Functions::Name))
                    .col(string(Functions::Description))
                    .col(string_null(Functions::ParentId))
                    .col(
                        ColumnDef::new(Functions::Version)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(Functions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Functions::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_functions_parent")
                            .from(Functions::Table, Functions::ParentId)
                            .to(Functions::Table, Functions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Components::Table)
                    .if_not_exists()
                    .col(string(Components::Id).primary_key())
                    .col(string(Components::Name))
                    .col(string(Components::Description))
                    .col(string_null(Components::ParentId))
                    .col(
                        ColumnDef::new(Components::Version)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(Components::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Components::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_components_parent")
                            .from(Components::Table, Components::ParentId)
                            .to(Components::Table, Components::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ComponentImplementsFunction::Table)
                    .if_not_exists()
                    .col(string(ComponentImplementsFunction::Id).primary_key())
                    .col(string(ComponentImplementsFunction::ComponentId))
                    .col(string(ComponentImplementsFunction::FunctionId))
                    .col(
                        ColumnDef::new(ComponentImplementsFunction::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_impl_component")
                            .from(
                                ComponentImplementsFunction::Table,
                                ComponentImplementsFunction::ComponentId,
                            )
                            .to(Components::Table, Components::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_impl_function")
                            .from(
                                ComponentImplementsFunction::Table,
                                ComponentImplementsFunction::FunctionId,
                            )
                            .to(Functions::Table, Functions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("uq_component_implements_function")
                            .col(ComponentImplementsFunction::ComponentId)
                            .col(ComponentImplementsFunction::FunctionId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_functions_parent_created")
                    .table(Functions::Table)
                    .col(Functions::ParentId)
                    .col(Functions::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_components_parent_created")
                    .table(Components::Table)
                    .col(Components::ParentId)
                    .col(Components::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_impl_component")
                    .table(ComponentImplementsFunction::Table)
                    .col(ComponentImplementsFunction::ComponentId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_impl_component")
                    .table(ComponentImplementsFunction::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_components_parent_created")
                    .table(Components::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_functions_parent_created")
                    .table(Functions::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(ComponentImplementsFunction::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Components::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Functions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Functions {
    Table,
    Id,
    Name,
    Description,
    ParentId,
    Version,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Components {
    Table,
    Id,
    Name,
    Description,
    ParentId,
    Version,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ComponentImplementsFunction {
    Table,
    Id,
    ComponentId,
    FunctionId,
    CreatedAt,
}
