//! Doctrine scraper for EVE Online

extern crate zkill;

extern crate eve_type_id;

// filter out item flag 5 for cargo
// filter out item flag 87 for drones
// filter corpses too

fn main() {
    let of_sound_mind_alliance_id = 99000739;
    let request = zkill::ZkillRequest::new(of_sound_mind_alliance_id,
                                           zkill::ZkillRequestType::Losses);
    let losses = zkill::kills(request).into_iter().filter(is_pod);
    let mut type_name_client = eve_type_id::TypeNameClient::new();
    for loss in losses {
        println!("{:?}", type_name_client.name(loss.victim_ship_type_id));
        let items: Vec<_> = loss.victim_items
                                .iter()
                                .map(|item| type_name_client.name(item.type_id))
                                .collect();
        println!("{:?}", items);
    }
}

fn is_pod(kill: &zkill::Kill) -> bool {
    let pod_ship_type_id = 670;
    kill.victim_ship_type_id != pod_ship_type_id
}
