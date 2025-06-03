# Part 1 - Data Structure and Input

## Overview
- Create CID for any Binary Large Object (BLOB), eg. photos, videos, etc.
- Allow for sharding of large BLOBs into 256MB chunks
- CID used as global identifier for each asset

Types of data input:
- BLOBs for photos (JPG), videos (MP4), 3D models (PLY)
- Metadata (eg. EXIF for photos), standardized as JSON
- Key-Value pairs for Individual attestations from [Starling Lab Authenticated Attributes](https://github.com/starlinglab/authenticated-attributes/)

