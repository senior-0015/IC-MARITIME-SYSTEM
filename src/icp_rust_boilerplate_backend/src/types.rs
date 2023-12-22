use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, Cell, Storable};
use std::borrow::Cow;

// Define types for memory and ID cell
pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type IdCell = Cell<u64, Memory>;


// Define the structure for Vessel
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Vessel {
    pub id: u64,
    pub name: String,
    pub captain: String,
    pub admin_principal: String,
    pub capacity: u32,
    pub current_location: String,
    pub last_update: u64,
    pub voyages_ids: Vec<u64>
}

// Define the structure for VesselPayload
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct VesselPayload {
    pub name: String,
    pub captain: String,
    pub capacity: u32,
    pub current_location: String,
}

// Implement Storable trait for Vessel
impl Storable for Vessel {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement BoundedStorable trait for Vessel
impl BoundedStorable for Vessel {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define the structure for Voyage
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
pub struct Voyage {
    pub id: u64,
    pub vessel_id: u64,
    pub admin_principal: String,
    pub departure_port: String,
    pub destination_port: String,
    pub departure_time: u64,
    pub arrival_time: Option<u64>,
    pub vessel_data: Option<Vessel>
}
// Define the structure for VoyagePayload
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
pub struct VoyagePayload {
    pub vessel_id: u64,
    pub departure_port: String,
    pub destination_port: String,
    pub departure_time: u64,
    pub arrival_time: Option<u64>
}

// Implement Storable trait for Voyage
impl Storable for Voyage {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement BoundedStorable trait for Voyage
impl BoundedStorable for Voyage {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define an enum for error handling
#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum Error {
    NotFound { msg: String },
    InvalidPayload { errors: Vec<String> },
    NotVesselAdmin,
    NotVoyagePrincipal
}
