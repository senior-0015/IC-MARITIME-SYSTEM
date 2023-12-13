export const idlFactory = ({ IDL }) => {
  const Vessel = IDL.Record({
    'id' : IDL.Nat64,
    'name' : IDL.Text,
    'current_location' : IDL.Text,
    'captain' : IDL.Text,
    'capacity' : IDL.Nat32,
    'last_update' : IDL.Nat64,
  });
  const Voyage = IDL.Record({
    'id' : IDL.Nat64,
    'departure_port' : IDL.Text,
    'departure_time' : IDL.Nat64,
    'arrival_time' : IDL.Opt(IDL.Nat64),
    'destination_port' : IDL.Text,
    'vessel_id' : IDL.Nat64,
  });
  const Error = IDL.Variant({ 'NotFound' : IDL.Record({ 'msg' : IDL.Text }) });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : Vessel, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : Voyage, 'Err' : Error });
  return IDL.Service({
    'add_vessel' : IDL.Func([Vessel], [IDL.Opt(Vessel)], []),
    'add_voyage' : IDL.Func([Voyage], [IDL.Opt(Voyage)], []),
    'delete_vessel' : IDL.Func([IDL.Nat64], [Result], []),
    'delete_voyage' : IDL.Func([IDL.Nat64], [Result], []),
    'get_vessel' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'get_voyage' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'update_vessel' : IDL.Func([IDL.Nat64, Vessel], [Result], []),
    'update_voyage' : IDL.Func([IDL.Nat64, Voyage], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
