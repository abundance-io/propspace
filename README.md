# Propspace

Propspace is a decentralized application (DApp) built on the Internet Computer Protocol (ICP) that facilitates the selling and co-ownership of housing units as NFTs (Non-Fungible Tokens). Users can purchase units of a property, co-own it, and earn profits based on its performance.

## Overview

Propspace leverages a custom implementation of the DIP-721 NFT standard and a Decentralized Autonomous Organization (DAO) on the ICP blockchain to achieve the following:

- **Unitization of Real Estate**: Properties are divided into units represented as NFTs.
- **Property Co-Ownership**: Users can buy and own units, collectively owning the entire property.
- **Profit Sharing**: Co-owners receive profits based on property performance.

## Features

- Unitizing Properties as NFTs
- Purchase and Ownership of Property Units
- Profit Distribution among Co-owners
- DAO Governance and Decision Making

## Getting Started

### Prerequisites

- Node.js (version 20.0.0)
- Rust (version 1.6)
- Internet Computer SDK (Canister SDK, Replica)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/abundance-io/propspace.git
   ```

2. **Navigate to the project directory:**
   ```bash
   cd propspace
   ```

3. **Install dependencies:**
   ```bash
   npm install
   ```

4. **Configure your Internet Computer environment:**
   - Set up your Internet Computer environment as per the SDK documentation.

5. **Build and deploy the canister:**
   ```bash
   dfx deploy
   ```

### Usage Examples


## Searching for Housing Units

To search for available housing units on Propspace, utilize the following methods with `dfx` queries:

### Query: `search_units`

- **Description**: This query enables users to search for housing units based on specific criteria, such as location, size, price range, and amenities.
- **Parameters**: Accepts query arguments like `location`, `priceRange`, `size`, etc.
- **Returns**: An array of housing units matching the specified criteria.

#### Example:

```bash
# Sample dfx query for searching housing units
dfx canister call propspace search_units '(record { location = "City A"; priceRange = "$100,000 - $200,000"; size = "2 bedrooms"; })'
```

## Purchasing Units

Once you've found the desired housing unit(s) through searching, proceed to purchase units using the following query:

### Command: `purchase_unit`

- **Description**: Allows users to purchase specific housing units by providing the unit ID and necessary payment details.
- **Parameters**: Requires `unitId` and `paymentDetails`.
- **Returns**: Confirmation of the successful purchase or relevant error message.

#### Example:

```bash
# Sample dfx command for purchasing housing units
dfx canister call propspace purchase_unit '(unit123, record { paymentMethod = "Credit Card"; amount = "$150,000"; })'
```

### Contributing

We welcome contributions! If you'd like to contribute to Propspace, please follow these steps:
1. Fork the repository.
2. Create a new branch (`git checkout -b feature/new-feature`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature/new-feature`).
6. Create a pull request.

### License

This project is licensed under the [MIT License](LICENSE).
