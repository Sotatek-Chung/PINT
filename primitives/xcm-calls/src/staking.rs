// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Xcm support for `pallet_staking` calls

use codec::{Compact, Decode, Encode, Output};
use frame_support::{
    sp_std::vec::Vec,
    weights::{constants::RocksDbWeight, Weight},
    RuntimeDebug,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::PalletCallEncoder;
use crate::{CallEncoder, EncodeWith, PalletCall};

/// The index of `pallet_staking` in the polkadot runtime
pub const POLKADOT_PALLET_STAKING_INDEX: u8 = 7u8;

/// Provides encoder types to encode the associated types of the  `pallet_staking::Config` trait depending on the configured Context.
pub trait StakingCallEncoder<Source, Balance, AccountId>: PalletCallEncoder {
    /// Encodes the `<pallet_staking::Config>::Balance` depending on the context
    type CompactBalanceEncoder: EncodeWith<Balance, Self::Context>;

    /// Encodes the `<pallet_staking::Config>::Source` depending on the context
    type SourceEncoder: EncodeWith<Source, Self::Context>;

    /// Encodes the `<pallet_staking::Config>::AccountId` depending on the context
    type AccountIdEncoder: EncodeWith<AccountId, Self::Context>;
}

impl<'a, 'b, Source, Balance, AccountId, Config> Encode
    for CallEncoder<'a, 'b, StakingCall<Source, Balance, AccountId>, Config>
where
    Config: StakingCallEncoder<Source, Balance, AccountId>,
{
    fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
        // include the pallet identifier
        dest.push_byte(self.call.pallet_call_index());
        match self.call {
            StakingCall::Bond(bond) => {
                Config::SourceEncoder::encode_to_with(&bond.controller, self.ctx, dest);
                Config::CompactBalanceEncoder::encode_to_with(&bond.value, self.ctx, dest);

                match &bond.payee {
                    RewardDestination::Staked => {
                        dest.push_byte(0);
                    }
                    RewardDestination::Stash => {
                        dest.push_byte(1);
                    }
                    RewardDestination::Controller => {
                        dest.push_byte(2);
                    }
                    RewardDestination::Account(ref account) => {
                        dest.push_byte(3);
                        Config::AccountIdEncoder::encode_to_with(account, self.ctx, dest);
                    }
                    _ => {}
                }
            }
            StakingCall::BondExtra(val) => {
                Config::CompactBalanceEncoder::encode_to_with(val, self.ctx, dest);
            }
            StakingCall::Unbond(val) => {
                Config::CompactBalanceEncoder::encode_to_with(val, self.ctx, dest);
            }
            StakingCall::WithdrawUnbonded(val) => val.encode_to(dest),
            StakingCall::Nominate(sources) => {
                Compact(sources.len() as u32).encode_to(dest);
                for source in sources {
                    Config::SourceEncoder::encode_to_with(source, self.ctx, dest);
                }
            }
        }
    }
}

/// Represents dispatchable calls of the FRAME `pallet_staking` pallet.
///
/// *NOTE*: `Balance` is expected to encode with `HasCompact`
pub enum StakingCall<Source, Balance, AccountId> {
    /// The [`bond_extra`](https://crates.parity.io/pallet_staking/enum.Call.html#variant.bond_extra) extrinsic.
    ///
    /// The dispatch origin for this call must be _Signed_ by the stash account.
    // #[codec(index = 0)]
    Bond(Bond<Source, Balance, AccountId>),

    /// The [`bond_extra`](https://crates.parity.io/pallet_staking/enum.Call.html#variant.bond_extra) extrinsic.
    ///
    /// The dispatch origin for this call must be _Signed_ by the stash, not the controller.
    // #[codec(index = 1)]
    BondExtra(Balance),
    /// The [`unbond`](https://crates.parity.io/pallet_staking/enum.Call.html#variant.unbond) extrinsic.
    ///
    /// The dispatch origin for this call must be _Signed_ by the controller, not the stash.
    // #[codec(index = 2)]
    Unbond(Balance),
    /// The [`withdraw_unbonded`](https://crates.parity.io/pallet_staking/enum.Call.html#variant.withdraw_unbonded) extrinsic.
    ///
    /// The dispatch origin for this call must be _Signed_ by the controller, not the stash.
    // #[codec(index = 3)]
    WithdrawUnbonded(u32),
    /// The [`nominate`](https://crates.parity.io/pallet_staking/enum.Call.html#variant.nominate) extrinsic.
    ///
    /// The dispatch origin for this call must be _Signed_ by the controller, not the stash.
    // #[codec(index = 5)]
    Nominate(Vec<Source>),
}

impl<Source, Balance, AccountId> PalletCall for StakingCall<Source, Balance, AccountId> {
    /// the indices of the corresponding calls within the `pallet_staking`
    fn pallet_call_index(&self) -> u8 {
        match self {
            StakingCall::Bond(_) => 0,
            StakingCall::BondExtra(_) => 1,
            StakingCall::Unbond(_) => 2,
            StakingCall::WithdrawUnbonded(_) => 3,
            StakingCall::Nominate(_) => 5,
        }
    }
}

/// The [`bond_extra`](https://crates.parity.io/pallet_staking/enum.Call.html#variant.bond_extra) extrinsic.
///
/// The dispatch origin for this call must be _Signed_ by the stash account.
#[derive(PartialEq, Eq, Clone, RuntimeDebug)]
pub struct Bond<Source, Balance, AccountId> {
    /// The lookup type of the controller,
    pub controller: Source,
    /// The amount to bond.
    pub value: Balance,
    /// How to payout staking rewards
    pub payee: RewardDestination<AccountId>,
}

/// A destination account for payment. mirrored from `pallet_staking`
#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum RewardDestination<AccountId> {
    /// Pay into the stash account, increasing the amount at stake accordingly.
    Staked,
    /// Pay into the stash account, not increasing the amount at stake.
    Stash,
    /// Pay into the controller account.
    Controller,
    /// Pay into a specified account.
    Account(AccountId),
    /// Receive no reward.
    None,
}

/// The `pallet_staking` configuration for a particular chain
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct StakingConfig<AccountId, Balance> {
    /// The index of `pallet_index` within the parachain's runtime
    pub pallet_index: u8,
    /// The limitation to the number of fund-chunks that can be scheduled to be unlocked via `unbond`.
    ///
    /// If this is reached, the bonded account _must_ first wait until successful call to
    /// `withdraw_unbonded` to remove some of the chunks.
    pub max_unlocking_chunks: u32,
    /// Counter for the sent `unbond` calls.
    pub pending_unbond_calls: u32,
    /// The configured reward destination
    pub reward_destination: RewardDestination<AccountId>,
    /// The specified `minimum_balance` specified the parachain's `T::Currency`
    pub minimum_balance: Balance,
    /// The configured weights for `pallet_staking`
    pub weights: StakingWeights,
    // TODO add minumum (un)bond  that has to be met for executing XCM (un)bonding calls
}

/// Represents the bond state of the PINT's sovereign account on a chain
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct StakingBondState<Source, Balance> {
    /// The controller
    pub controller: Source,

    /// The amount bonded
    pub bonded: Balance,
}

/// Represents an excerpt from the `pallet_staking` weights
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct StakingWeights {
    /// Weight for `bond` extrinsic
    pub bond: Weight,
    /// Weight for `bond_extra` extrinsic
    pub bond_extra: Weight,
    /// Weight for `unbond` extrinsic
    pub unbond: Weight,
}

impl StakingWeights {
    /// The weights as defined in `pallet_staking` on polkadot
    // TODO: import pallet_staking weights directly?
    pub fn polkadot() -> Self {
        let weight = RocksDbWeight::get();
        Self {
            bond: (75_102_000 as Weight)
                .saturating_add(weight.reads(5 as Weight))
                .saturating_add(weight.writes(4 as Weight)),
            bond_extra: (57_637_000 as Weight)
                .saturating_add(weight.reads(3 as Weight))
                .saturating_add(weight.writes(2 as Weight)),
            unbond: (52_115_000 as Weight)
                .saturating_add(weight.reads(4 as Weight))
                .saturating_add(weight.writes(3 as Weight)),
        }
    }
}