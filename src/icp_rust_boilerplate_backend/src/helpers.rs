use crate::types::*;
use ic_cdk::{api::time, caller};

fn is_invalid_string_input(str_input: &str) -> bool {
    return str_input.trim().len() == 0;
}

pub fn validate_vessel_payload(payload: &VesselPayload) -> Result<(), Error>{
    let mut errors: Vec<String> = Vec::new();
    if is_invalid_string_input(&payload.name){
        errors.push(format!("Vessel name='{}' cannot be empty.", payload.name))
    }
    if is_invalid_string_input(&payload.captain){
        errors.push(format!("Captain='{}' cannot be empty.", payload.captain))
    }
    if is_invalid_string_input(&payload.current_location){
        errors.push(format!("Current location='{}' cannot be empty.", payload.current_location))
    }
    if payload.capacity == 0{
        errors.push(format!("Capacity cannot be set to zero"))
    }

    if errors.is_empty(){
        Ok(())
    }else{
        return Err(Error::InvalidPayload { errors })
    }

}
pub fn validate_voyage_payload(payload: &VoyagePayload) -> Result<(), Error>{
    let mut errors: Vec<String> = Vec::new();
    if is_invalid_string_input(&payload.departure_port){
        errors.push(format!("Voyage departure port='{}' cannot be empty.", payload.departure_port))
    }
    if is_invalid_string_input(&payload.destination_port){
        errors.push(format!("Voyage destination port='{}' cannot be empty.", payload.destination_port))
    }
    let timestamp = time();
    if payload.departure_time < timestamp{
        errors.push(
            format!("Departure time={} needs to be equal or greater than the current timestamp={}", 
            payload.departure_time, timestamp
        ))
    }
    let arrival_time = payload.arrival_time.unwrap_or_default();
    if payload.arrival_time.is_some() && arrival_time <= payload.departure_time{
        errors.push(
            format!("Arrival time={} needs to be greater than the departure time={}", 
            arrival_time, payload.departure_time
        ))
    }

    if errors.is_empty(){
        Ok(())
    }else{
        return Err(Error::InvalidPayload { errors })
    }

}


pub fn is_caller_vessel_admin(vessel: &Vessel) -> Result<(), Error>{
    if vessel.admin_principal != caller().to_string(){
        return Err(Error::NotVesselAdmin)
    }else{
        Ok(())
    }
}
pub fn is_caller_voyage_admin(voyage: &Voyage) -> Result<(), Error>{
    if voyage.admin_principal != caller().to_string(){
        return Err(Error::NotVoyagePrincipal)
    }else{
        Ok(())
    }
}