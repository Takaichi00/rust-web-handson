pub use sea_orm_migration::prelude::*;

mod m20220728_120310_create_location_owners_table;
mod m20220728_123218_create_locations_table;
mod m20220728_123353_create_signages_table;
mod m20220728_123518_create_advertisers_table;
mod m20220728_135138_create_advertiser_users_table;
mod m20220728_135506_create_campaigns_table;
mod m20220728_135522_create_adgroups_table;
mod m20220728_135531_create_ads_table;
mod m20220728_135554_create_adgroups_locations_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220728_120310_create_location_owners_table::Migration),
            Box::new(m20220728_123218_create_locations_table::Migration),
            Box::new(m20220728_123353_create_signages_table::Migration),
            Box::new(m20220728_123518_create_advertisers_table::Migration),
            Box::new(m20220728_135138_create_advertiser_users_table::Migration),
            Box::new(m20220728_135506_create_campaigns_table::Migration),
            Box::new(m20220728_135522_create_adgroups_table::Migration),
            Box::new(m20220728_135531_create_ads_table::Migration),
            Box::new(m20220728_135554_create_adgroups_locations_table::Migration),
        ]
    }
}
