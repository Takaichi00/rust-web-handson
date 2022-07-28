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
            insert into locations(id, location_owner_id, name, address, latitude, longitude)
                values
                    (1, 1, 'ベストお茶ノ水II', '東京都千代田区外神田２丁目８－４', 35.70104862, 139.7684097),
                    (2, 1, 'コスモ調布', '東京都調布市布田３－４４－４', 35.64949017, 139.5498519);
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            delete from locations where id in (1, 2);
        "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
