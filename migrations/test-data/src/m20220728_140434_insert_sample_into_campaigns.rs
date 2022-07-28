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
            insert into campaigns(id, advertiser_id, name, status, start_at, end_at, revenue_share_rate, max_total_budget, max_monthly_budget, max_daily_budget, max_hourly_budget, frequency_control_interval_sec)
                values
                    (1, 1, '「どうする家族」番宣キャンペーン', 1, '2022-10-01 00:00:00', '2022-11-30 23:59:59', 10.0, 2000000, 1000000, 100000, '[0,0,0,0,0,3000,5000,5000,10000,5000,3000,4000,4000,3000,3000,3000,5000,5000,5000,5000,4000,3000,2000,1000]', null),
                    (2, 1, '「舞いあがれ」番宣キャンペーン', 1, '2022-10-01 00:00:00', '2022-10-31 23:59:59', 10.0, null, null, null, null, null),
                    (3, 2, '5周年記念特別セールキャンペーン', 1, '2022-11-01 00:00:00', '2022-12-31 23:59:59', 10.0, 5000000, null, null, null, null);
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            delete from campaigns where id in (1, 2, 3);
        "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
