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
            create table advertiser_users (
                id int(11) not null auto_increment,
                advertiser_id int(11) not null,
                login_name varchar(255) null,
                email varchar(255) not null,
                encrypted_password varchar(255) null,
                last_sign_in_at datetime null, 
                created_at datetime not null default current_timestamp,
                updated_at datetime not null default current_timestamp on update current_timestamp,
                deleted_at datetime null default null,
                constraint primary key (id),
                constraint foreign key fk_advertiser_id (advertiser_id) references advertisers(id)
            ) character set utf8mb4 collate utf8mb4_bin;
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            drop table advertiser_users;
        "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
