





// https://github.com/hyperium/hyper/blob/master/examples/params.rs
// get all (un)successful payments for an event with admin or God access
// get all (un)successful payments for a user with user access
// TODO - after reservation (successful payment):
//          - update role_id and side_id with user_id inside the users collection
//          - insert new player role ability into player_role_ability_info collection
//          - insert the new user_id into the players field of the events collection
// ...