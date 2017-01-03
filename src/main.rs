//! Doctrine scraper for EVE Online

#[macro_use]
extern crate itertools;

extern crate zkill;
extern crate eve_type_id;

use itertools::Itertools;

use std::collections::btree_map::BTreeMap;
use std::iter::FromIterator;

fn main() {
    let mut count = BTreeMap::new();
    let of_sound_mind_alliance_id = 99000739;
    let request = zkill::ZkillRequest::new(of_sound_mind_alliance_id,
                                           zkill::ZkillRequestType::Losses);
    let losses = zkill::kills(request).into_iter().filter(ship_type_filter);
    let mut type_name_client = eve_type_id::TypeNameClient::new();
    for (index, loss) in losses.enumerate() {
        let hull_name = type_name_client.name(loss.victim_ship_type_id);
        println!("Loss {:?}: {:?}", index, loss);
        *count.entry(hull_name).or_insert(0) += 1;
        loss.victim_items
            .into_iter()
            .map(|item| {
                (type_name_client.name(item.type_id),
                 item.quantity_dropped + item.quantity_destroyed)
            })
            .foreach(|(item_name, quantity)| {
                *count.entry(item_name).or_insert(0) += quantity;
            });
    }
    let mut v = Vec::from_iter(count.clone());
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    for (item, count) in v {
        println!("{},{}", item, count);
    }
}

fn ship_type_filter(kill: &zkill::Kill) -> bool {
    !is_pod(kill)
}

fn is_pod(kill: &zkill::Kill) -> bool {
    let basic_pod_ship_type_id = 670;
    let genolution_pod_ship_type_id = 33328;
    kill.victim_ship_type_id == basic_pod_ship_type_id ||
    kill.victim_ship_type_id == genolution_pod_ship_type_id
}
