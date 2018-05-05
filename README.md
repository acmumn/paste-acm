# paste-acm

[![Build Status](https://travis-ci.org/acmumn/paste-acm.svg?branch=master)](https://travis-ci.org/acmumn/paste-acm)
[![Dependency Status](https://deps.rs/repo/github/acmumn/paste-acm/status.svg)](https://deps.rs/repo/github/acmumn/paste-acm)
[![Docker Status](https://img.shields.io/docker/build/acmumn/paste-acm.svg)](https://hub.docker.com/r/acmumn/paste-acm)

## API Guide

### POST /

Uploads the body of the request as a paste.
Returns the paste-id.

### GET /paste-id

Returns the content of a paste.

## Example Usage

```
echo "This is my data" | curl --data-binary @- https://p.acm.umn.edu/

curl https://p.acm.umn.edu/AGABrHBWxlg=
```

## License

Licensed under either of

 * Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
