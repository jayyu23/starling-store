# Guttenfelder Photos Storage Toolkit

### EE 292J Final Project 

By Jay Yu and Ihsaan Doola


## Overview

Within this project, we present a storage toolkit for David Guttenfelder’s North Korea archive, featuring photos taken from 2015 in the Democratic People Republic of Korea and derived 3D reconstructions. Our project seeks to design a storage pipeline for these artifacts using emerging web3 technologies, addressing questions such as:

* What are the different types of data that we need to store from the Guttenfelder archive, and how should we process this data for storage processes?  
* What are the web3 storage technologies that may be available for this task? What are each of their strengths and limitations?  
* How do we use cryptographic techniques to ensure that the data remains secure and tamper-resistant?  
* How do we address privacy issues surrounding what can be revealed to the public, and what must remain confidential to protect journalists and subjects’ identities?  
* How does this project compose with existing initiatives by Starling Lab, including the Authenticated Attributes project, as well as wider industry efforts such as C2PA?

In our proposed solution, we will highlight three key parts in our toolkit solution:

* **1 \- Data Input & Processing** \- We present a solution to standardize data input from multiple sources, including photos, videos, 3D models, and related metadata, as IPFS-compatible Binary Large Objects (BLOBs), and create prototype for BLOB data sharding. We also take as input signed metadata attestations following the Starling Authenticated Attributes repository.  
    
* **2 \- Data Storage Solutions** \- We examine various web3 technologies that may be suitable for the storage of various components, including: IPFS via Pinata, Arweave, Data Availability (DA) solutions, Akave’s S3-compatible decentralized database solution, on-chain smart contract state (Ethereum Virtual Machine SSTORE), and as Move-language objects on Sui Walrus. We demonstrate a prototype pipeline for BLOB storage on IPFS via Pinata

* **3 \- Data Security & Privacy** \- We consider a pipeline for data security and privacy of the image data and metadata. We read EXIF metadata, provide a toolkit to store the metadata and related attributes as a Merkle tree, and verification of metadata attributes through the Zero Knowledge Virtual Machine (ZKVM) by Nexus Labs.
