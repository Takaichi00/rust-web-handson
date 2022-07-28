pub use sea_orm_migration::prelude::*;

mod m20220728_140143_insert_sample_into_advertisers;
mod m20220728_140424_insert_sample_into_advertiser_users;
mod m20220728_140434_insert_sample_into_campaigns;
mod m20220728_140448_insert_sample_into_adgroups;
mod m20220728_140455_insert_sample_into_ads;
mod m20220728_140508_insert_sample_into_location_owners;
mod m20220728_140516_insert_sample_into_locations;
mod m20220728_140525_insert_sample_into_signages;
mod m20220728_140533_insert_sample_into_adgroups_locations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220728_140143_insert_sample_into_advertisers::Migration),
            Box::new(m20220728_140424_insert_sample_into_advertiser_users::Migration),
            Box::new(m20220728_140434_insert_sample_into_campaigns::Migration),
            Box::new(m20220728_140448_insert_sample_into_adgroups::Migration),
            Box::new(m20220728_140455_insert_sample_into_ads::Migration),
            Box::new(m20220728_140508_insert_sample_into_location_owners::Migration),
            Box::new(m20220728_140516_insert_sample_into_locations::Migration),
            Box::new(m20220728_140525_insert_sample_into_signages::Migration),
            Box::new(m20220728_140533_insert_sample_into_adgroups_locations::Migration),
        ]
    }
}
