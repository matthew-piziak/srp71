//! Doctrine scraper for EVE Online.

#[macro_use]
extern crate itertools;

extern crate time;

extern crate zkill;
extern crate eve_type_id;

use itertools::Itertools;

use std::collections::btree_map::BTreeMap;
use std::iter::FromIterator;

fn main() {
    let mut count = BTreeMap::new();
    let of_sound_mind_alliance_id = 99000739;
    let start_time = start_time();
    let mut type_name_client = eve_type_id::TypeNameClient::new();

    let page = 1;
    loop {
        let request = zkill::ZkillRequest::new(of_sound_mind_alliance_id,
                                               zkill::ZkillRequestType::Losses,
                                               start_time.clone(),
                                               page);
        let losses: Vec<_> = zkill::kills(request).into_iter().filter(ship_type_filter).collect();
        if losses.len() == 0 {
            break;
        }
        for (index, loss) in losses.into_iter().enumerate() {
            let hull_name = type_name_client.name(loss.victim_ship_type_id);
            println!("Page {}: Loss {}", page, index);
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

fn start_time() -> String {
    let start_time = time::now_utc() - time::Duration::days(90);

    // time::Tm has strange semantics. Refer to documentation.
    format!("{}{:02}{:02}{:02}{:02}",
            start_time.tm_year + 1900,
            start_time.tm_mon + 1,
            start_time.tm_mday,
            start_time.tm_hour,
            start_time.tm_min)
}
