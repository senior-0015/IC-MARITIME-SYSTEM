import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Error = { 'NotFound' : { 'msg' : string } };
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Vessel } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : Voyage } |
  { 'Err' : Error };
export interface Vessel {
  'id' : bigint,
  'name' : string,
  'current_location' : string,
  'captain' : string,
  'capacity' : number,
  'last_update' : bigint,
}
export interface Voyage {
  'id' : bigint,
  'departure_port' : string,
  'departure_time' : bigint,
  'arrival_time' : [] | [bigint],
  'destination_port' : string,
  'vessel_id' : bigint,
}
export interface _SERVICE {
  'add_vessel' : ActorMethod<[Vessel], [] | [Vessel]>,
  'add_voyage' : ActorMethod<[Voyage], [] | [Voyage]>,
  'delete_vessel' : ActorMethod<[bigint], Result>,
  'delete_voyage' : ActorMethod<[bigint], Result>,
  'get_vessel' : ActorMethod<[bigint], Result_1>,
  'get_voyage' : ActorMethod<[bigint], Result_2>,
  'update_vessel' : ActorMethod<[bigint, Vessel], Result>,
  'update_voyage' : ActorMethod<[bigint, Voyage], Result>,
}
