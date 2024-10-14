use geohasher::encode;

fn main() {
    let geohash = encode(45.0, 120.0, 8);
    println!("{}", geohash);
}   