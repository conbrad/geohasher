use crate::constants::BASE32;

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
/// * `precision` - The number of characters in the resulting geohash string
/// # Returns
///
/// A String containing the geohash representation of the given coordinates.
///
/// # Example
///
/// ```
/// use geohasher::encode;
///
/// let geohash = encode(57.64911, 10.40744, 8);
/// assert_eq!(geohash, "u4pruydq");
/// ```

pub fn encode(lat: f64, lng: f64, precision: usize) -> String {
    let mut z_coordinate = Vec::with_capacity(60);
    let mut lat_interval = (-90.0, 90.0);
    let mut lng_interval = (-180.0, 180.0);

    for _ in 0..60 {
        let mid = (lng_interval.0 + lng_interval.1) / 2.0;

        if lng > mid {
            z_coordinate.push(1);
            lng_interval.0 = mid;
        } else {
            z_coordinate.push(0);
            lng_interval.1 = mid;
        }

        let mid = (lat_interval.0 + lat_interval.1) / 2.0;

        if lat > mid {
            z_coordinate.push(1);
            lat_interval.0 = mid;
        } else {
            z_coordinate.push(0);
            lat_interval.1 = mid;
        }
    }

    z_coordinate.chunks(5)
    .take(precision)
    .map(|chunk| {
        let mut value = 0;
        for (i, &bit) in chunk.iter().enumerate() {
            value |= (bit as usize) << (4 - i);
        }
        BASE32[value]
    })
    .collect()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_basic() {
        // Basic encoding with 8 bits of precision
        assert_eq!(encode(57.64911, 10.40744, 8), "u4pruydq");
    }

    #[test]
    fn test_encode_low_precision() {
        // Low, 4 bit precision
        assert_eq!(encode(57.64911, 10.40744, 4), "u4pr");
    }

    #[test]
    fn test_encode_high_precision() {
        // High, 12 bit precision
        assert_eq!(encode(57.64911, 10.40744, 12), "u4pruydqqvj8");
    }

    #[test]
    fn test_extremes() {
        assert_eq!(encode(90.0, 180.0, 6), "zzzzzz");
        assert_eq!(encode(-90.0, -180.0, 6), "000000");
    }

    #[test]
    fn test_zero_coordinates() {
        assert_eq!(encode(0.0, 0.0, 12), "7zzzzzzzzzzz");
    }

    #[test]
    fn test_random_coordinates() {
        assert_eq!(encode(34.052235, -118.243683, 12), "9q5ctr18dkxw");
    }
}
