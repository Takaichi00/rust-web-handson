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
            insert into ads(id, adgroup_id, name, status, start_at, end_at, movie_sec, content_path, width, height)
                values
                    (1, 1, '「どうする家族」番宣キャンペーン_アドA', 1, '2022-10-01 00:00:00', '2022-11-30 23:59:59', 30, 'path/to/1', 1920, 1080),
                    (2, 1, '「どうする家族」番宣キャンペーン_アドB', 1, '2022-10-01 00:00:00', '2022-11-30 23:59:59', 30, 'path/to/2', 1920, 1080),
                    (3, 2, '「舞いあがれ」番宣キャンペーン_アド', 1, '2022-10-01 00:00:00', '2022-10-31 23:59:59', 30, 'path/to/3', 1920, 1080),
                    (4, 3, '5周年記念特別セールキャンペーン_調布店', 1, '2022-11-01 00:00:00', '2022-12-31 23:59:59', 15, 'path/to/4', 1920, 1080),
                    (5, 4, '5周年記念特別セールキャンペーン_丸の内店', 1, '2022-11-01 00:00:00', '2022-12-31 23:59:59', 15, 'path/to/5', 1920, 1080);
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            delete from ads where id in (1, 2, 3, 4, 5);
        "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
