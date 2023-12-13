// Import necessary crates and modules
#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define types for memory and ID cell
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define the structure for Vessel
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Vessel {
    id: u64,
    name: String,
    captain: String,
    capacity: u32,
    current_location: String,
    last_update: u64,
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

// Define thread-local variables for Vessel memory management
thread_local! {
    static VESSEL_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static VESSEL_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(VESSEL_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static VESSEL_STORAGE: RefCell<StableBTreeMap<u64, Vessel, Memory>> =
        RefCell::new(StableBTreeMap::init(
            VESSEL_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Define the structure for Voyage
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Voyage {
    id: u64,
    vessel_id: u64,
    departure_port: String,
    destination_port: String,
    departure_time: u64,
    arrival_time: Option<u64>,
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

// Define thread-local variables for Voyage memory management
thread_local! {
    static VOYAGE_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static VOYAGE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(VOYAGE_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static VOYAGE_STORAGE: RefCell<StableBTreeMap<u64, Voyage, Memory>> =
        RefCell::new(StableBTreeMap::init(
            VOYAGE_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Functions related to Vessel management

// Retrieve a Vessel by ID
#[ic_cdk::query]
fn get_vessel(id: u64) -> Result<Vessel, Error> {
    match _get_vessel(&id) {
        Some(vessel) => Ok(vessel),
        None => Err(Error::NotFound {
            msg: format!("a vessel with id={} not found", id),
        }),
    }
}

// Add a new Vessel
#[ic_cdk::update]
fn add_vessel(vessel: Vessel) -> Option<Vessel> {
    // Generate a new ID for the Vessel
    let id = VESSEL_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment vessel id counter");

    // Create a new Vessel instance
    let vessel = Vessel {
        id,
        name: vessel.name,
        captain: vessel.captain,
        capacity: vessel.capacity,
        current_location: vessel.current_location,
        last_update: time(),
    };

    // Insert the Vessel into storage
    do_insert_vessel(&vessel);
    Some(vessel)
}

// Helper method to insert a Vessel into storage
fn do_insert_vessel(vessel: &Vessel) {
    VESSEL_STORAGE.with(|service| service.borrow_mut().insert(vessel.id, vessel.clone()));
}

// Functions related to Voyage management

// Retrieve a Voyage by ID
#[ic_cdk::query]
fn get_voyage(id: u64) -> Result<Voyage, Error> {
    match _get_voyage(&id) {
        Some(voyage) => Ok(voyage),
        None => Err(Error::NotFound {
            msg: format!("a voyage with id={} not found", id),
        }),
    }
}

// Add a new Voyage
#[ic_cdk::update]
fn add_voyage(voyage: Voyage) -> Option<Voyage> {
    // Generate a new ID for the Voyage
    let id = VOYAGE_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment voyage id counter");

    // Create a new Voyage instance
    let voyage = Voyage {
        id,
        vessel_id: voyage.vessel_id,
        departure_port: voyage.departure_port,
        destination_port: voyage.destination_port,
        departure_time: time(),
        arrival_time: None,
    };

    // Insert the Voyage into storage
    do_insert_voyage(&voyage);
    Some(voyage)
}

// Helper method to insert a Voyage into storage
fn do_insert_voyage(voyage: &Voyage) {
    VOYAGE_STORAGE.with(|service| service.borrow_mut().insert(voyage.id, voyage.clone()));
}

// Other helper methods and structures remain unchanged.

// Define an enum for error handling
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

// Helper methods for vessel and voyage retrieval

// Retrieve a Vessel by ID from storage
fn _get_vessel(id: &u64) -> Option<Vessel> {
    VESSEL_STORAGE.with(|service| service.borrow().get(id))
}

// Retrieve a Voyage by ID from storage
fn _get_voyage(id: &u64) -> Option<Voyage> {
    VOYAGE_STORAGE.with(|service| service.borrow().get(id))
}

// Update a Voyage by ID
#[ic_cdk::update]
fn update_voyage(id: u64, updated_voyage: Voyage) -> Result<(), Error> {
    match _get_voyage(&id) {
        Some(mut existing_voyage) => {
            // Update relevant fields
            existing_voyage.departure_port = updated_voyage.departure_port;
            existing_voyage.destination_port = updated_voyage.destination_port;

            // Update the last_update timestamp
            existing_voyage.departure_time = time();

            // Insert the updated Voyage into storage
            do_insert_voyage(&existing_voyage);
            Ok(())
        }
        None => Err(Error::NotFound {
            msg: format!("a voyage with id={} not found", id),
        }),
    }
}

// Delete a Voyage by ID
#[ic_cdk::update]
fn delete_voyage(id: u64) -> Result<(), Error> {
    // Check if the Voyage exists
    if let Some(_) = _get_voyage(&id) {
        // Remove the Voyage from storage
        VOYAGE_STORAGE.with(|service| service.borrow_mut().remove(&id));
        Ok(())
    } else {
        // Return an error if the Voyage is not found
        Err(Error::NotFound {
            msg: format!("a voyage with id={} not found", id),
        })
    }
}

// Update a Vessel by ID
#[ic_cdk::update]
fn update_vessel(id: u64, updated_vessel: Vessel) -> Result<(), Error> {
    match _get_vessel(&id) {
        Some(mut existing_vessel) => {
            // Update relevant fields
            existing_vessel.name = updated_vessel.name;
            existing_vessel.captain = updated_vessel.captain;
            existing_vessel.capacity = updated_vessel.capacity;
            existing_vessel.current_location = updated_vessel.current_location;

            // Update the last_update timestamp
            existing_vessel.last_update = time();

            // Insert the updated Vessel into storage
            do_insert_vessel(&existing_vessel);
            Ok(())
        }
        None => Err(Error::NotFound {
            msg: format!("a vessel with id={} not found", id),
        }),
    }
}

// Delete a Vessel by ID
#[ic_cdk::update]
fn delete_vessel(id: u64) -> Result<(), Error> {
    // Check if the Vessel exists
    if let Some(_) = _get_vessel(&id) {
        // Remove the Vessel from storage
        VESSEL_STORAGE.with(|service| service.borrow_mut().remove(&id));
        Ok(())
    } else {
        // Return an error if the Vessel is not found
        Err(Error::NotFound {
            msg: format!("a vessel with id={} not found", id),
        })
    }
}

// Need this to generate candid
ic_cdk::export_candid!();

