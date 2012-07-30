use poem_openapi::Object;

#[derive(Object)]
pub struct HomeStatistics {
   pub all_users: u64,
    pub  all_packages: u64,
    pub  all_downloads: u64,
}
