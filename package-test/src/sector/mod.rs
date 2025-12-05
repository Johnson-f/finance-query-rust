//! Sector data module
//!
//! Provides functions to fetch sector performance and company data.

pub mod sector;

pub use sector::{
    get_all_sectors, get_all_sectors_performance, get_industry, get_sector_performance,
    get_sector_top_companies, SectorCompany, SectorDetails, SectorPerformance, SectorsOverview,
};

// Re-export Sector enum for convenience
pub use finance_query_core::Sector;
