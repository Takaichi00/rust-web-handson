use sea_orm_migration::prelude::*;

struct ConcatMigrator;

impl MigratorTrait for ConcatMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations = migration::Migrator::migrations();
        migrations.append(&mut test_data::Migrator::migrations());
        migrations
    }
}

#[async_std::main]
async fn main() {
    cli::run_cli(ConcatMigrator).await;
}
