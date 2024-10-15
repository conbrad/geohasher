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

### References:
According to [Wikipedia](https://en.wikipedia.org/wiki/Geohash):
"Geohash is a public domain geocode system invented in 2008 by Gustavo Niemeyer[1] which encodes a geographic location into a short string of letters and digits. Similar ideas were introduced by G.M. Morton in 1966[2]."

- 1 https://web.archive.org/web/20080305223755/http://blog.labix.org/#post-85
- 2 https://web.archive.org/web/20190125020453/https://domino.research.ibm.com/library/cyberdig.nsf/papers/0DABF9473B9C86D48525779800566A39/$File/Morton1966.pdf
