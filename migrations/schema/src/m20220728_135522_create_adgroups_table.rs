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
            create table adgroups (
                id int(11) not null auto_increment,
                campaign_id int(11) not null,
                name varchar(255) not null,
                status tinyint(1) not null,
                start_at datetime null,
                end_at datetime null,
                max_total_budget bigint(20) null,
                max_monthly_budget bigint(20) null,
                max_daily_budget bigint(20) null,
                max_hourly_budget JSON null,
                cpv decimal(20, 6) not null,
                created_at datetime not null default current_timestamp,
                updated_at datetime not null default current_timestamp on update current_timestamp,
                deleted_at datetime null default null,
                constraint primary key (id),
                constraint foreign key fk_campaign_id (campaign_id) references campaigns(id)
            ) character set utf8mb4 collate utf8mb4_bin;
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            drop table adgroups;
        "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
