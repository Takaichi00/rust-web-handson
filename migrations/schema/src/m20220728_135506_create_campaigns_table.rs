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
            create table campaigns (
                id int(11) not null auto_increment,
                advertiser_id int(11) not null,
                name varchar(255) not null,
                status tinyint(1) not null,
                start_at datetime null,
                end_at datetime null,
                revenue_share_rate decimal(20,3) not null,
                max_total_budget bigint(20) null,
                max_monthly_budget bigint(20) null,
                max_daily_budget bigint(20) null,
                max_hourly_budget JSON null,
                frequency_control_interval_sec int(11) null,
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
            drop table campaigns;
        "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
