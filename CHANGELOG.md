# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The `paperless-ngx` upstream api is subject to changes that are not reflected in the api version or software version number. At least on in any
meaningful way according to semver! 

This changelog is to keep track of changes to this crate and their compatibilty to paperless-ngx api versions

## Unreleased

## 2026-06-27 paperless-api-client 6.2.0

No new paperles api version sync, keep major version 6, but increase to minor version 2 because introduciton of enums is a breaking change
in regards to the rust client api.

### Added
- introduce enums for all interger select fields

### Fixed
- update all doc tests and get them working

## 2026-05-19 paperless-api-client 6.1.0

Sync api client with paperless version v2.20.15 api version according to openapi-spec 6.0.0 (9)

### Added
- log retrieve api call to include new limit parameter
- add processed_mail api module
- new custom_field type longtext
- tag parent-child relation fields
- workflow api type with new filter keys

### Changed
- changed PostDocmuentRequest field format for custom_field creation

### Removed
- remove share_link (partial-)update api calls
- remove document notes related pageination logic


## 2025-11-09 paperless-api-client 6.0.1
- inital version with patches to make most used api points reliable



