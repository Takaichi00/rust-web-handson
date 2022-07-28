use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, Statement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            create table signages (
                id int(11) not null auto_increment,
                location_id int(11) not null,
                hashkey varchar(255) null,
                external_id varchar(255) null,
                is_active tinyint(1) not null,
                created_at datetime not null default current_timestamp,
                updated_at datetime not null default current_timestamp on update current_timestamp,
                deleted_at datetime null default null,
                constraint primary key (id),
                constraint foreign key fk_location_id (location_id) references locations(id)
            ) character set utf8mb4 collate utf8mb4_bin;
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            drop table signages;
        "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
