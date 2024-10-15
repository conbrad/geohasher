### Geohasher
Simple geohash encode/decode functions without any dependencies.


### Encoding

To encode a latitude and longitude into a geohash:

```rust
use geohasher::encode;

let geohash = encode(57.64911, 10.40744, 8);
assert_eq!(geohash, "u4pruydq");
```

### Decoding

To decode a geohash back into latitude and longitude:

```rust
use geohasher::decode;

let (lat, lng) = decode("u4pruydq");
assert!((lat - 57.64911).abs() < 0.001);
assert!((lng - 10.40744).abs() < 0.001);
```