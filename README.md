# Anchor NFT Stake

## Overview

The **Anchor NFT Stake** repository contains a Solana program designed to facilitate staking mechanisms for NFTs. Built using the Anchor framework, this program allows users to stake their NFTs in exchange for rewards or other benefits. The program is highly modular and can be integrated into various NFT ecosystems to provide staking functionalities with customizable reward mechanisms.

## Features

- **NFT Staking**: Users can stake their NFTs for a set period and earn rewards.
- **Reward Distribution**: The program allows for automated or manual reward distribution based on staking duration.
- **Configurable Staking Parameters**: Stake duration, reward type, and other parameters can be adjusted by the program administrator.

## Programs

### 1. **Staking Program**

The Staking Program handles the core functionality of staking NFTs. It includes the following key components:

- **Stake Accounts**: Tracks individual staking sessions, including the staked NFT, staking duration, and user details.
- **Reward Mechanism**: Defines the reward structure, including calculation logic based on staking duration or other criteria.
- **Stake Management**: Allows users to stake, unstake, and claim rewards, with checks to ensure staking conditions are met.

### 2. **Admin Program**

The Admin Program provides tools for managing the staking system:

- **Configure Staking Parameters**: Set global parameters such as minimum stake duration, reward rates, and penalty for early withdrawal.
- **Manage Rewards**: Add, remove, or adjust reward types available in the system.
- **User Management**: Administer user access and roles, ensuring only authorized actions are performed.

## Installation

To get started with the repository, clone it and initialize the necessary submodules:

```bash
git clone https://github.com/kox/anchor-nft-stake.git
cd anchor-nft-stake
anchor build
```

## Usage

Once the programs are built, deploy them to your Solana cluster and interact with them using the provided scripts or via a frontend integrated with the program's API.
Contributing

## Contributions
Contributions are welcome! Please fork the repository, create a feature branch, and submit a pull request.

