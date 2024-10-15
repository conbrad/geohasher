use geohasher::{encode, decode};

fn main() {
    let geohash = encode(45.0, 120.0, 8);
    println!("geohash: {}", geohash);
    let (lat, lng) = decode(&geohash);
    println!("lat: {}, lng: {}", lat, lng);
}   