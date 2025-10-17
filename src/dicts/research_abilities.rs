// THIS FILE WAS AUTOMATICALLY GENERATED.
// Source: python-sc2 sc2/dicts/unit_research_abilities.py

//! Research abilities to upgrade mappings.
//! 
//! This module provides a mapping from research abilities (like `BarracksTechLabResearchStimpack`)
//! to their corresponding upgrade IDs (like `UpgradeId::Stimpack`). This allows you to check
//! if you can afford a research by using the ability ID instead of needing to know the upgrade ID.
//!
//! # Example
//! ```no_run
//! use rust_sc2::prelude::*;
//! use rust_sc2::dicts::get_upgrade_for_ability;
//!
//! # struct Bot;
//! # impl Bot {
//! #   fn minerals(&self) -> u32 { 1000 }
//! #   fn vespene(&self) -> u32 { 1000 }
//! #   fn can_afford_upgrade(&self, _: UpgradeId) -> bool { true }
//! #   fn can_afford_ability_research(&self, ability: AbilityId) -> Option<bool> {
//! #       get_upgrade_for_ability(ability).map(|upgrade| self.can_afford_upgrade(upgrade))
//! #   }
//! # }
//! # let bot = Bot;
//! // Check if we can afford stimpack research using the ability ID
//! if let Some(can_afford) = bot.can_afford_ability_research(AbilityId::BarracksTechLabResearchStimpack) {
//!     if can_afford {
//!         println!("Can afford stimpack research!");
//!     }
//! }
//! ```

use crate::ids::{AbilityId, UpgradeId};
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
	/// Maps research abilities to their corresponding upgrade IDs
	pub static ref ABILITY_TO_UPGRADE: HashMap<AbilityId, UpgradeId> = {
		let mut m = HashMap::new();
		m.insert(AbilityId::ResearchPhoenixAnionPulseCrystals, UpgradeId::PhoenixRangeUpgrade);
		m.insert(AbilityId::FleetBeaconResearchVoidRaySpeedUpgrade, UpgradeId::VoidRaySpeedUpgrade);
		m.insert(AbilityId::FleetBeaconResearchTempestResearchGroundAttackUpgrade, UpgradeId::EnhancedShockwaves);
		m.insert(AbilityId::ResearchGlialRegeneration, UpgradeId::GlialReconstitution);
		m.insert(AbilityId::ResearchTunnelingClaws, UpgradeId::TunnelingClaws);
		m.insert(AbilityId::ResearchAnabolicSynthesis, UpgradeId::AnabolicSynthesis);
		m.insert(AbilityId::ResearchChitinousPlating, UpgradeId::ChitinousPlating);
		m.insert(AbilityId::ResearchHiSecAutoTracking, UpgradeId::HiSecAutoTracking);
		m.insert(AbilityId::ResearchTerranStructureArmorUpgrade, UpgradeId::TerranBuildingArmor);
		m.insert(AbilityId::EngineeringBayResearchTerranInfantryWeaponsLevel1, UpgradeId::TerranInfantryWeaponsLevel1);
		m.insert(AbilityId::EngineeringBayResearchTerranInfantryWeaponsLevel2, UpgradeId::TerranInfantryWeaponsLevel2);
		m.insert(AbilityId::EngineeringBayResearchTerranInfantryWeaponsLevel3, UpgradeId::TerranInfantryWeaponsLevel3);
		m.insert(AbilityId::EngineeringBayResearchTerranInfantryArmorLevel1, UpgradeId::TerranInfantryArmorsLevel1);
		m.insert(AbilityId::EngineeringBayResearchTerranInfantryArmorLevel2, UpgradeId::TerranInfantryArmorsLevel2);
		m.insert(AbilityId::EngineeringBayResearchTerranInfantryArmorLevel3, UpgradeId::TerranInfantryArmorsLevel3);
		m.insert(AbilityId::BarracksTechLabResearchStimpack, UpgradeId::Stimpack);
		m.insert(AbilityId::ResearchCombatShield, UpgradeId::ShieldWall);
		m.insert(AbilityId::ResearchConcussiveShells, UpgradeId::PunisherGrenades);
		m.insert(AbilityId::ResearchInfernalPreigniter, UpgradeId::HighCapacityBarrels);
		m.insert(AbilityId::ResearchDrillingClaws, UpgradeId::DrillClaws);
		m.insert(AbilityId::ResearchSmartServos, UpgradeId::SmartServos);
		m.insert(AbilityId::ResearchCycloneLockOnDamage, UpgradeId::CycloneLockOnDamageUpgrade);
		m.insert(AbilityId::ResearchBansheeCloakingField, UpgradeId::BansheeCloak);
		m.insert(AbilityId::ResearchBansheeHyperflightRotors, UpgradeId::BansheeSpeed);
		m.insert(AbilityId::StarportTechLabResearchRavenInterferenceMatrix, UpgradeId::AmplifiedShielding);
		m.insert(AbilityId::ResearchPersonalCloaking, UpgradeId::PersonalCloaking);
		m.insert(AbilityId::ArmoryResearchTerranVehicleWeaponsLevel1, UpgradeId::TerranVehicleWeaponsLevel1);
		m.insert(AbilityId::ArmoryResearchTerranVehicleWeaponsLevel2, UpgradeId::TerranVehicleWeaponsLevel2);
		m.insert(AbilityId::ArmoryResearchTerranVehicleWeaponsLevel3, UpgradeId::TerranVehicleWeaponsLevel3);
		m.insert(AbilityId::ArmoryResearchTerranShipWeaponsLevel1, UpgradeId::TerranShipWeaponsLevel1);
		m.insert(AbilityId::ArmoryResearchTerranShipWeaponsLevel2, UpgradeId::TerranShipWeaponsLevel2);
		m.insert(AbilityId::ArmoryResearchTerranShipWeaponsLevel3, UpgradeId::TerranShipWeaponsLevel3);
		m.insert(AbilityId::ArmoryResearchTerranVehicleAndShipPlatingLevel1, UpgradeId::TerranVehicleAndShipArmorsLevel1);
		m.insert(AbilityId::ArmoryResearchTerranVehicleAndShipPlatingLevel2, UpgradeId::TerranVehicleAndShipArmorsLevel2);
		m.insert(AbilityId::ArmoryResearchTerranVehicleAndShipPlatingLevel3, UpgradeId::TerranVehicleAndShipArmorsLevel3);
		m.insert(AbilityId::ForgeResearchProtossGroundWeaponsLevel1, UpgradeId::ProtossGroundWeaponsLevel1);
		m.insert(AbilityId::ForgeResearchProtossGroundWeaponsLevel2, UpgradeId::ProtossGroundWeaponsLevel2);
		m.insert(AbilityId::ForgeResearchProtossGroundWeaponsLevel3, UpgradeId::ProtossGroundWeaponsLevel3);
		m.insert(AbilityId::ForgeResearchProtossGroundArmorLevel1, UpgradeId::ProtossGroundArmorsLevel1);
		m.insert(AbilityId::ForgeResearchProtossGroundArmorLevel2, UpgradeId::ProtossGroundArmorsLevel2);
		m.insert(AbilityId::ForgeResearchProtossGroundArmorLevel3, UpgradeId::ProtossGroundArmorsLevel3);
		m.insert(AbilityId::ForgeResearchProtossShieldsLevel1, UpgradeId::ProtossShieldsLevel1);
		m.insert(AbilityId::ForgeResearchProtossShieldsLevel2, UpgradeId::ProtossShieldsLevel2);
		m.insert(AbilityId::ForgeResearchProtossShieldsLevel3, UpgradeId::ProtossShieldsLevel3);
		m.insert(AbilityId::ResearchGraviticBooster, UpgradeId::ObserverGraviticBooster);
		m.insert(AbilityId::ResearchGraviticDrive, UpgradeId::GraviticDrive);
		m.insert(AbilityId::ResearchExtendedThermalLance, UpgradeId::ExtendedThermalLance);
		m.insert(AbilityId::ResearchPsiStorm, UpgradeId::PsiStormTech);
		m.insert(AbilityId::ResearchZergMeleeWeaponsLevel1, UpgradeId::ZergMeleeWeaponsLevel1);
		m.insert(AbilityId::ResearchZergMeleeWeaponsLevel2, UpgradeId::ZergMeleeWeaponsLevel2);
		m.insert(AbilityId::ResearchZergMeleeWeaponsLevel3, UpgradeId::ZergMeleeWeaponsLevel3);
		m.insert(AbilityId::ResearchZergGroundArmorLevel1, UpgradeId::ZergGroundArmorsLevel1);
		m.insert(AbilityId::ResearchZergGroundArmorLevel2, UpgradeId::ZergGroundArmorsLevel2);
		m.insert(AbilityId::ResearchZergGroundArmorLevel3, UpgradeId::ZergGroundArmorsLevel3);
		m.insert(AbilityId::ResearchZergMissileWeaponsLevel1, UpgradeId::ZergMissileWeaponsLevel1);
		m.insert(AbilityId::ResearchZergMissileWeaponsLevel2, UpgradeId::ZergMissileWeaponsLevel2);
		m.insert(AbilityId::ResearchZergMissileWeaponsLevel3, UpgradeId::ZergMissileWeaponsLevel3);
		m.insert(AbilityId::ResearchPneumatizedCarapace, UpgradeId::Overlordspeed);
		m.insert(AbilityId::ResearchBurrow, UpgradeId::Burrow);
		m.insert(AbilityId::ResearchZerglingAdrenalGlands, UpgradeId::Zerglingattackspeed);
		m.insert(AbilityId::ResearchZerglingMetabolicBoost, UpgradeId::Zerglingmovementspeed);
		m.insert(AbilityId::ResearchGroovedSpines, UpgradeId::EvolveGroovedSpines);
		m.insert(AbilityId::ResearchMuscularAugments, UpgradeId::EvolveMuscularAugments);
		m.insert(AbilityId::ResearchZergFlyerAttackLevel1, UpgradeId::ZergFlyerWeaponsLevel1);
		m.insert(AbilityId::ResearchZergFlyerAttackLevel2, UpgradeId::ZergFlyerWeaponsLevel2);
		m.insert(AbilityId::ResearchZergFlyerAttackLevel3, UpgradeId::ZergFlyerWeaponsLevel3);
		m.insert(AbilityId::ResearchZergFlyerArmorLevel1, UpgradeId::ZergFlyerArmorsLevel1);
		m.insert(AbilityId::ResearchZergFlyerArmorLevel2, UpgradeId::ZergFlyerArmorsLevel2);
		m.insert(AbilityId::ResearchZergFlyerArmorLevel3, UpgradeId::ZergFlyerArmorsLevel3);
		m.insert(AbilityId::ResearchNeuralParasite, UpgradeId::NeuralParasite);
		m.insert(AbilityId::ResearchCentrifugalHooks, UpgradeId::CentrificalHooks);
		m.insert(AbilityId::ResearchBattlecruiserWeaponRefit, UpgradeId::BattlecruiserEnableSpecializations);
		m.insert(AbilityId::FusionCoreResearchBallisticRange, UpgradeId::LiberatorAGRangeUpgrade);
		m.insert(AbilityId::FusionCoreResearchMedivacEnergyUpgrade, UpgradeId::MedivacCaduceusReactor);
		m.insert(AbilityId::CyberneticsCoreResearchProtossAirWeaponsLevel1, UpgradeId::ProtossAirWeaponsLevel1);
		m.insert(AbilityId::CyberneticsCoreResearchProtossAirWeaponsLevel2, UpgradeId::ProtossAirWeaponsLevel2);
		m.insert(AbilityId::CyberneticsCoreResearchProtossAirWeaponsLevel3, UpgradeId::ProtossAirWeaponsLevel3);
		m.insert(AbilityId::CyberneticsCoreResearchProtossAirArmorLevel1, UpgradeId::ProtossAirArmorsLevel1);
		m.insert(AbilityId::CyberneticsCoreResearchProtossAirArmorLevel2, UpgradeId::ProtossAirArmorsLevel2);
		m.insert(AbilityId::CyberneticsCoreResearchProtossAirArmorLevel3, UpgradeId::ProtossAirArmorsLevel3);
		m.insert(AbilityId::ResearchWarpGate, UpgradeId::WarpGateResearch);
		m.insert(AbilityId::ResearchCharge, UpgradeId::Charge);
		m.insert(AbilityId::ResearchBlink, UpgradeId::BlinkTech);
		m.insert(AbilityId::ResearchAdeptResonatingGlaives, UpgradeId::AdeptPiercingAttack);
		m.insert(AbilityId::ResearchShadowStrike, UpgradeId::DarkTemplarBlinkUpgrade);
		m.insert(AbilityId::ResearchAdaptiveTalons, UpgradeId::DiggingClaws);
		m.insert(AbilityId::LurkerDenResearchLurkerRange, UpgradeId::LurkerRange);
		m
	};
}

/// Get the upgrade ID associated with a research ability
pub fn get_upgrade_for_ability(ability: AbilityId) -> Option<UpgradeId> {
	ABILITY_TO_UPGRADE.get(&ability).copied()
}