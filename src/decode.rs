use crate::constants::BASE32;
/// Decodes a geohash string into latitude and longitude coordinates.
///
/// This function takes a geohash string and converts it back into its corresponding
/// latitude and longitude coordinates. The geohash is a base32-encoded string that
/// represents a rectangular area on the Earth's surface.
///
/// # Arguments
///
/// * `geohash` - A string slice that holds the geohash to be decoded.
///
/// # Returns
///
/// A tuple containing two f64 values:
/// * The first value is the latitude in degrees.
/// * The second value is the longitude in degrees.
///
/// # Example
///
/// ```
/// use geohasher::decode;
///
/// let (lat, lng) = decode("u4pruydq");
/// assert!((lat - 57.64911).abs() < 0.001);
/// assert!((lng - 10.40744).abs() < 0.001);
/// ```
///
/// # Note
///
/// The precision of the returned coordinates depends on the length of the input geohash.
/// Longer geohashes provide more precise coordinates.

pub fn decode(geohash: &str) -> (f64, f64) {
    let mut lat_interval = (-90.0, 90.0);
    let mut lon_interval = (-180.0, 180.0);

    let mut lat_bit = false;
    for c in geohash.chars() {
        let char_index = BASE32.iter().position(|&ch| ch == c).unwrap();
        let bits = decimal_to_binary(char_index);
        
        for bit in bits {
            if lat_bit {
                lat_interval = refine_interval(lat_interval, bit);
            } else {
                lon_interval = refine_interval(lon_interval, bit);
            }
            lat_bit = !lat_bit;
        }
    }

    let lat = (lat_interval.0 + lat_interval.1) / 2.0;
    let lng = (lon_interval.0 + lon_interval.1) / 2.0;
    (lat, lng)
}

pub fn decimal_to_binary(mut n: usize) -> Vec<u8> {
    if n == 0 {
        return vec![0, 0, 0, 0, 0];
    }

    let mut bits = Vec::with_capacity(5);
    while n > 0 {
        bits.push((n & 1) as u8);
        n >>= 1;
    }

    bits.reverse();
    while bits.len() < 5 {
        bits.insert(0, 0);
    }
    bits
}

fn refine_interval(interval: (f64, f64), bit: u8) -> (f64, f64) {
    let mid = (interval.0 + interval.1) / 2.0;
    if bit == 1 {
        (mid, interval.1)
    } else {
        (interval.0, mid)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_basic() {
        // Basic decoding with 8 character geohash
        let (lat, lng) = decode("u4pruydq");
        assert!((lat - 57.64911).abs() < 0.001);
        assert!((lng - 10.40744).abs() < 0.001);
    }

    #[test]
    fn test_d2b_zero_case() {
        assert_eq!(decimal_to_binary(0), vec![0, 0, 0, 0, 0]);
    }



    #[test]
    fn test_d2b_basic() {
        assert_eq!(decimal_to_binary(BASE32.iter().position(|&c| c == 'u').unwrap()), vec![1, 1, 0, 1, 0]);
        assert_eq!(decimal_to_binary(BASE32.iter().position(|&c| c == '4').unwrap()), vec![0, 0, 1, 0, 0]);
        assert_eq!(decimal_to_binary(BASE32.iter().position(|&c| c == 'p').unwrap()), vec![1, 0, 1, 0, 1]);
        assert_eq!(decimal_to_binary(BASE32.iter().position(|&c| c == 'r').unwrap()), vec![1, 0, 1, 1, 1]);
        assert_eq!(decimal_to_binary(BASE32.iter().position(|&c| c == 'y').unwrap()), vec![1, 1, 1, 1, 0]);
        assert_eq!(decimal_to_binary(BASE32.iter().position(|&c| c == 'd').unwrap()), vec![0, 1, 1, 0, 0]);
        assert_eq!(decimal_to_binary(BASE32.iter().position(|&c| c == 'q').unwrap()), vec![1, 0, 1, 1, 0]);
    }
}
