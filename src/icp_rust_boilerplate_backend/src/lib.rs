// Import necessary crates and modules
#[macro_use]
extern crate serde;
use ic_cdk::api::time;
use ic_cdk::caller;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

mod types;
use types::*;
mod helpers;
use helpers::*;


// Define thread-local variables for Voyage memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static VOYAGE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static VOYAGE_STORAGE: RefCell<StableBTreeMap<u64, Voyage, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static VESSEL_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 0)
            .expect("Cannot create a counter")
    );

    static VESSEL_STORAGE: RefCell<StableBTreeMap<u64, Vessel, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
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
fn add_vessel(vessel: VesselPayload) -> Result<Vessel, Error> {
    validate_vessel_payload(&vessel)?;

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
        admin_principal: caller().to_string(),
        captain: vessel.captain,
        capacity: vessel.capacity,
        current_location: vessel.current_location,
        last_update: time(),
        voyages_ids: Vec::new()
    };

    // Insert the Vessel into storage
    do_insert_vessel(&vessel);
    Ok(vessel)
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
fn add_voyage(voyage: VoyagePayload) -> Result<Voyage, Error> {
    let mut vessel = get_vessel(voyage.vessel_id)?;

    is_caller_vessel_admin(&vessel)?;

    validate_voyage_payload(&voyage)?;

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
        admin_principal: vessel.admin_principal.clone(),
        departure_port: voyage.departure_port,
        destination_port: voyage.destination_port,
        departure_time: time(),
        arrival_time: voyage.arrival_time,
        vessel_data: None
    };
    // add voyage's id to the voyages_ids of the vessel
    vessel.voyages_ids.push(id);

    // Insert the updatedVessel into storage
    do_insert_vessel(&vessel);
    // Insert the Voyage into storage
    do_insert_voyage(&voyage);
    Ok(voyage)
}

// Helper method to insert a Voyage into storage
fn do_insert_voyage(voyage: &Voyage) {
    VOYAGE_STORAGE.with(|service| service.borrow_mut().insert(voyage.id, voyage.clone()));
}

// Other helper methods and structures remain unchanged.

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
fn update_voyage(id: u64, updated_voyage: VoyagePayload) -> Result<(), Error> {
    match _get_voyage(&id) {
        Some(mut existing_voyage) => {
            // ensures that only voyages of existing vessels can be updated
            let vessel = get_vessel(existing_voyage.vessel_id)?;
            is_caller_vessel_admin(&vessel)?;

            validate_voyage_payload(&updated_voyage)?;

            // Update relevant fields
            existing_voyage.departure_port = updated_voyage.departure_port;
            existing_voyage.destination_port = updated_voyage.destination_port;
            existing_voyage.departure_time = updated_voyage.departure_time;
            existing_voyage.arrival_time = updated_voyage.arrival_time;

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
    if let Some(voyage) = _get_voyage(&id) {
        is_caller_voyage_admin(&voyage)?;

        // Checks if vessel exists
        let vessel_opt = get_vessel(voyage.vessel_id);
        // if vessel exists, remove the voyage_id from the voyages_ids field and save the updated vessel
        if vessel_opt.is_ok(){
            let mut vessel = vessel_opt.ok().unwrap();
            vessel.voyages_ids.retain(|&voyage_id| voyage_id != id);
            do_insert_vessel(&vessel);
        }

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
fn update_vessel(id: u64, updated_vessel: VesselPayload) -> Result<(), Error> {
    match _get_vessel(&id) {
        Some(mut existing_vessel) => {
            is_caller_vessel_admin(&existing_vessel)?;
            validate_vessel_payload(&updated_vessel)?;

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
    if let Some(vessel) = _get_vessel(&id) {
        is_caller_vessel_admin(&vessel)?;
        vessel.voyages_ids.iter().for_each(|voyage_id| {
            let voyage_opt = get_voyage(voyage_id.clone());
            if voyage_opt.is_ok(){
                let mut voyage = voyage_opt.ok().unwrap();
                voyage.vessel_data = Some(vessel.clone());
                do_insert_voyage(&voyage);
            }
        });

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

