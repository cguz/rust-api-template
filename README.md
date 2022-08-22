# Template to develop a RUST API

Template API implemented in Rust to request information

## Requirements

- Rust 1.58.0-nightly
- [Rocket](https://rocket.rs/) v0.4

## API endpoints

### STM

- https://<URL>/api/v1/data/stm/\<satellite\> : retrieve information of the satellite alongs with possible CDM
- https://<URL>/api/v1/data/stm/cdm/\<satellite\> : retrieve information of the CDM for the given satellite
- https://<URL>/api/v1/data/stm/cdm/\<first_object\>/\<second_object\> : compute the CDM for two objects, where objects can be either an space debris or satellites.
- https://<URL>/api/v1/data/stm/cdm/list : list all possible CDM

## Reference

- Rocket Guide : https://rocket.rs/v0.4/guide/getting-started/