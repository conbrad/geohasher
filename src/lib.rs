static BASE32: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', 
    '8', '9', 'b', 'c', 'd', 'e', 'f', 'g', 
    'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];
/// Encodes a latitude and longitude into a geohash string.
///
/// This function takes a latitude and longitude as floating-point numbers
/// and returns a geohash string representation. The geohash is a hierarchical
/// spatial data structure which subdivides space into buckets of grid shape.
///
/// # Arguments
///
/// * `lat` - The latitude in decimal degrees (range: -90 to 90)
/// * `lng` - The longitude in decimal degrees (range: -180 to 180)
///
/// # Returns
///
/// A String containing the geohash representation of the given coordinates.
///
/// # Example
///
/// ```
/// use geohasher::encode;
///
/// let geohash = encode(57.64911, 10.40744);
/// assert_eq!(geohash, "u4pruydq");
/// ```

pub fn encode(lat: f64, lng: f64) -> String {
    let mut z_coordinate: Vec<i32> = Vec::new();
    let mut geohash = String::new();
    let mut lat_interval: (f64, f64) = (-90.0, 90.0);
    let mut lng_interval: (f64, f64) = (-180.0, 180.0);

    for _ in 0..12 {
        let mid = (lng_interval.0 + lng_interval.1) / 2.0;

        if lng >= mid {
            z_coordinate.push(1);
            lng_interval.0 = mid;
        } else {
            z_coordinate.push(0);
            lng_interval.1 = mid;
        }

        let mid = (lat_interval.0 + lat_interval.1) / 2.0;

        if lat >= mid {
            z_coordinate.push(1);
            lat_interval.0 = mid;
        } else {
            z_coordinate.push(0);
            lat_interval.1 = mid;
        }
    }

    let chunk_size = 5;
    for chunk in z_coordinate.chunks(chunk_size) {
        let mut base32: usize = 0;
        for (i, &bit) in chunk.iter().enumerate() {
            base32 |= (bit as usize) << ((chunk_size - 1) - i) as usize;
        }

        geohash.push(BASE32[base32]);
    }

    geohash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        // Test case 1: Basic encoding
        assert_eq!(encode(57.64911, 10.40744), "u4pru");
    }
}
