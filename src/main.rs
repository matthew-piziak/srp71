//! Doctrine scraper for EVE Online

extern crate zkill;

fn main() {
    let request = zkill::ZkillRequest::new(0, zkill::ZkillRequestType::Losses);
    let kills = zkill::kills(request);
    for kill in kills {
        println!("{:?}", kill);
    }
}
