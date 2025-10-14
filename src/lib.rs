/*!
# Rust StarCraft II API

A Rust implementation of the StarCraft II API for creating bots.

## Quick Start

```rust
use rust_sc2::prelude::*;

#[bot]
#[derive(Default)]
struct MyBot;

impl Player for MyBot {
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Random).with_name("MyBot")
    }
    
    fn on_step(&mut self, _iteration: usize) -> SC2Result<()> {
        // Bot logic here
        Ok(())
    }
}

fn main() -> SC2Result<()> {
    run_vs_computer(
        &mut MyBot::default(),
        Computer::new(Race::Random, Difficulty::VeryEasy, None),
        "Simple64",
        LaunchOptions::default(),
    )
}
```

See examples and documentation for more details.
*/
// #![warn(missing_docs)]
#![deny(broken_intra_doc_links)]
#![allow(clippy::upper_case_acronyms)]

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate sc2_macro;
#[macro_use]
extern crate itertools;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate log;

/// The most frequent used items and various traits here.
/// Prefered usage: `use rust_sc2::prelude::*;`.
pub mod prelude {
	#[cfg(feature = "rayon")]
	pub use crate::distance::rayon::{ParCenter, ParDistanceIterator, ParDistanceSlice};
	#[cfg(feature = "rayon")]
	pub use crate::units::rayon::ParUnitsIterator;
	pub use crate::{
		action::Target,
		bot::{PathfindingUnitType, PlacementOptions},
		client::{
			run_ladder_game, run_vs_computer, run_vs_human, LaunchOptions, RunnerMulti, RunnerSingle,
			SC2Result,
		},
		consts::{ALL_PRODUCERS, PRODUCERS, RESEARCHERS, TECH_REQUIREMENTS},
		distance::{Center, Distance, DistanceIterator, DistanceSlice},
		game_state::Alliance,
		geometry::Point2,
		ids::*,
		player::{AIBuild, Computer, Difficulty, GameResult, Race},
		unit::Unit,
		units::{iter::UnitsIterator, Units},
		Event, Player, PlayerSettings,
	};
	// Re-export pathfinding types
	pub use sc2pathfinding::Choke;
	pub use sc2pathfinding::VisionUnit;
	pub use sc2pathfinding::Map;
	pub use sc2pathfinding::PathFind;
	#[doc(no_inline)]
	pub use sc2_macro::{bot, bot_new};
}

mod paths;

pub mod action;
pub mod api;
pub mod bot;
pub mod client;
pub mod consts;
pub mod debug;
pub mod distance;
pub mod game_data;
pub mod game_info;
pub mod game_state;
pub mod geometry;
pub mod dicts;
pub mod ids;
pub mod pixel_map;
pub mod player;
pub mod ramp;
pub mod score;
pub mod unit;
pub mod units;
pub mod utils;

use game_state::Alliance;
use player::{GameResult, Race};

/// Implements `Deref` and `DerefMut` for accessing `Bot` fields directly.
#[doc(inline)]
pub use sc2_macro::bot;

/// Adds Bot initialization to constructor functions.
#[doc(inline)]
pub use sc2_macro::bot_new;

#[doc(inline)]
pub use client::SC2Result;
/**
Request to the SC2 API.

# Usage
```
let mut request = Request::new();

/* modify request through it's methods */

let response = self.api().send(request)?;
```
*/
pub use sc2_proto::sc2api::Request;

/// Settings that must be provided by a player when joining a game.
///
/// if name is `None`, it'll be shown as "foo(whatever)" in game.
///
/// if `raw_affects_selection` is `true`, bot will select units to which it gives orders.
///
/// if `raw_crop_to_playable_area` is `true`, maps will be crooped to the size of
/// [`self.game_info.playable_area`](game_info::GameInfo::playable_area).
///
/// Defaults:
/// `name`: `None`
/// `raw_affects_selection`: `false`
/// `raw_crop_to_playable_area`: `false`
pub struct PlayerSettings<'a> {
	pub race: Race,
	pub name: Option<&'a str>,
	pub raw_affects_selection: bool,
	pub raw_crop_to_playable_area: bool,
}
impl<'a> PlayerSettings<'a> {
	/// Constructs new settings with given `Race`.
	pub fn new(race: Race) -> Self {
		Self {
			race,
			name: None,
			raw_affects_selection: false,
			raw_crop_to_playable_area: false,
		}
	}
	/// Sets name of the player.
	pub fn with_name(mut self, name: &'a str) -> Self {
		self.name = Some(name);
		self
	}
	/// Sets `raw_affects_selection` to a given value.
	pub fn raw_affects_selection(mut self, val: bool) -> Self {
		self.raw_affects_selection = val;
		self
	}
	/// Sets `raw_crop_to_playable_area` to a given value.
	pub fn raw_crop_to_playable_area(mut self, val: bool) -> Self {
		self.raw_crop_to_playable_area = val;
		self
	}
}
impl Default for PlayerSettings<'_> {
	fn default() -> Self {
		Self {
			race: Race::Random,
			name: None,
			raw_affects_selection: false,
			raw_crop_to_playable_area: false,
		}
	}
}

/// Events that happen in game.
/// Passed to [`on_event`](Player::on_event).
#[derive(Debug, Clone, Copy)]
pub enum Event {
	/// Unit died or structure destroyed (all units: your, enemy, neutral).
	UnitDestroyed(u64, Option<Alliance>),
	/// Unit finished training (your only).
	UnitCreated(u64),
	/// Worker started to build a structure (your only).
	ConstructionStarted(u64),
	/// Construction of a structure finished (your only).
	ConstructionComplete(u64),
	/// Detected actual race of random opponent.
	RandomRaceDetected(Race),
}

/// Trait that bots must implement.
pub trait Player {
	/// Returns settings used to connect bot to the game.
	fn get_player_settings(&self) -> PlayerSettings;
	/// Called once on first step (i.e on game start).
	fn on_start(&mut self) -> SC2Result<()> {
		Ok(())
	}
	/// Called on every game step. (Main logic of the bot should be here)
	fn on_step(&mut self, _iteration: usize) -> SC2Result<()> {
		Ok(())
	}
	/// Called once on last step with a result for your bot.
	fn on_end(&self, _result: GameResult) -> SC2Result<()> {
		Ok(())
	}
	/// Called when different events happen.
	fn on_event(&mut self, _event: Event) -> SC2Result<()> {
		Ok(())
	}
}

trait FromProto<T>
where
	Self: Sized,
{
	fn from_proto(p: T) -> Self;
}

trait IntoSC2<T> {
	fn into_sc2(self) -> T;
}
impl<T, U: FromProto<T>> IntoSC2<U> for T {
	fn into_sc2(self) -> U {
		U::from_proto(self)
	}
}

trait TryFromProto<T>
where
	Self: Sized,
{
	fn try_from_proto(p: T) -> Option<Self>;
}

trait IntoProto<T> {
	fn into_proto(self) -> T;
}

/*trait FromSC2<T> {
	fn from_sc2(s: T) -> Self;
}
impl<T, U: IntoProto<T>> FromSC2<U> for T {
	fn from_sc2(s: U) -> T {
		s.into_proto()
	}
}*/
