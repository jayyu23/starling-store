#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
EE 292J - Photo Metadata Merkle Proof

Command-line version of the Jupyter notebook for proving photo metadata authenticity
using Merkle trees.

Usage: python metadata_merkle.py <image_path>
"""

import sys
import argparse
from pprint import pprint
import os
from PIL import Image
from PIL.ExifTags import TAGS, GPSTAGS
import hashlib
from geopy.geocoders import Nominatim
from pymerkle import InmemoryTree as MerkleTree, verify_inclusion
from pymerkle.hasher import MerkleHasher


def get_exif_data(img_path):
    """Extract EXIF metadata from image."""
    image = Image.open(img_path)
    exif_data = image._getexif()
    metadata = {}
    if exif_data:
        for tag_id, value in exif_data.items():
            tag = TAGS.get(tag_id, tag_id)
            metadata[tag] = value
    return metadata


def dms_to_dd(dms, ref):
    """Convert degrees, minutes, seconds to decimal degrees."""
    degrees, minutes, seconds = dms
    dd = degrees + minutes / 60 + seconds / 3600
    return -dd if ref in ['S', 'W'] else dd


def get_lat_long(gps_info):
    """Extract latitude and longitude from GPS info."""
    lat = dms_to_dd(gps_info[2], gps_info[1])
    lon = dms_to_dd(gps_info[4], gps_info[3])
    return float(lat), float(lon)


def get_geocode(exif_data):
    """Get location information from GPS coordinates in EXIF data."""
    if 'GPSInfo' not in exif_data:
        print("No GPS information found in image metadata")
        return None
        
    try:
        lat, lon = get_lat_long(exif_data['GPSInfo'])
        geolocator = Nominatim(user_agent="metadata-reverse-geocoder")
        location = geolocator.reverse((lat, lon), exactly_one=True, language="en")
        if location and location.raw:
            address = location.raw.get("address", {})
            return {
                "city": address.get("city") or address.get("town") or address.get("hamlet"),
                "street": address.get("road"),
                "postal_code": address.get("postcode"),
                "county": address.get("county"),
                "state": address.get("state")
            }
    except Exception as e:
        print(f"Error getting location data: {e}")
    return None


def build_merkle_tree(exif_data):
    """Build Merkle tree from EXIF data."""
    tree = MerkleTree(algorithm='sha256')
    hasher = MerkleHasher(tree.algorithm, tree.security)
    key_index_map = {}

    # Build Merkle Tree
    for key, value in exif_data.items():
        entry_str = f"{key}:{value}"
        index = tree.append_entry(entry_str.encode())
        key_index_map[key] = index

    return tree, hasher, key_index_map


def prove_statement(tree, hasher, key_index_map, key, value):
    """Prove a statement about the metadata using Merkle proof."""
    if key not in key_index_map:
        print(f"Error: Key {key} not found")
        return False
    
    index = key_index_map[key]
    # Hash key value into leaf format
    leaf = hasher.hash_buff(f"{key}:{value}".encode())

    root = tree.get_state(tree.get_size())
    proof = tree.prove_inclusion(index, tree.get_size())
    
    print(f"Leaf: {leaf.hex()}")
    print(f"Root: {root.hex()}")
    print("Proof: ", [sibling.hex() for sibling in proof.path])
    
    try:
        verify_inclusion(leaf, root, proof)
        return True
    except:
        return False


def main():
    parser = argparse.ArgumentParser(description='Prove photo metadata authenticity using Merkle trees')
    parser.add_argument('image_path', help='Path to the image file')
    parser.add_argument('--show-metadata', action='store_true', help='Display all metadata')
    parser.add_argument('--show-image', action='store_true', help='Display the image (requires GUI)')
    
    args = parser.parse_args()
    
    if not os.path.exists(args.image_path):
        print(f"Error: Image file '{args.image_path}' not found")
        sys.exit(1)

    print("=" * 60)
    print("EE 292J - Photo Metadata Merkle Proof")
    print("=" * 60)
    
    # Step 1: Extract Image and Metadata
    print("\nStep 1: Extracting image metadata...")
    exif_data = get_exif_data(args.image_path)
    
    if args.show_metadata:
        print("\nEXIF Data:")
        pprint(exif_data)
    
    if args.show_image:
        try:
            image = Image.open(args.image_path)
            image.show()
        except Exception as e:
            print(f"Could not display image: {e}")

    # Step 2: Get Location Data
    print("\nStep 2: Getting location data from coordinates...")
    location_data = get_geocode(exif_data)
    if location_data:
        print("Location information:")
        pprint(location_data)
        # Append postal code to metadata
        exif_data["PostalCode"] = location_data["postal_code"]
    else:
        print("No location data available")

    # Step 3: Build Merkle Tree
    print("\nStep 3: Building Merkle tree...")
    tree, hasher, key_index_map = build_merkle_tree(exif_data)
    print(f"Merkle Tree Root: {tree.get_state().hex()}")

    # Step 4: Prove Statements
    print("\nStep 4: Proving statements about the metadata...")
    
    # Define test cases
    targets = [
        ("PostalCode", "94305"),  # Prove taken at Stanford
        ("DateTime", "2021:09:19 14:42:42"),  # Prove taken time
        ("Model", "iPhone 11 Pro Max"),  # Prove taken device
        ("PostalCode", "94102"),  # False Postal Code (for San Francisco)
        ('FakeData', 'FakeValue')  # Entry does not exist
    ]

    for target in targets:
        print(f"\nProving Statement: {target[0]} = {target[1]}")
        result = prove_statement(tree, hasher, key_index_map, target[0], target[1])
        print(f"Result: {'✓ VERIFIED' if result else '✗ FAILED'}")
        print("-" * 40)


if __name__ == "__main__":
    main()