use rust_sc2::prelude::*;
use rust_sc2::dicts::get_upgrade_for_ability;

#[test]
fn test_ability_to_upgrade_mapping() {
    // Test some known mappings
    assert_eq!(
        get_upgrade_for_ability(AbilityId::BarracksTechLabResearchStimpack),
        Some(UpgradeId::Stimpack)
    );
    
    assert_eq!(
        get_upgrade_for_ability(AbilityId::ResearchCombatShield),
        Some(UpgradeId::ShieldWall)
    );
    
    assert_eq!(
        get_upgrade_for_ability(AbilityId::EngineeringBayResearchTerranInfantryWeaponsLevel1),
        Some(UpgradeId::TerranInfantryWeaponsLevel1)
    );
    
    assert_eq!(
        get_upgrade_for_ability(AbilityId::ForgeResearchProtossGroundWeaponsLevel1),
        Some(UpgradeId::ProtossGroundWeaponsLevel1)
    );
    
    // Test a non-research ability returns None
    assert_eq!(
        get_upgrade_for_ability(AbilityId::Move),
        None
    );
}

#[test]
fn test_ability_to_upgrade_coverage() {
    // Test that we have a reasonable number of mappings
    use rust_sc2::dicts::ABILITY_TO_UPGRADE;
    
    // Should have plenty of research abilities mapped
    assert!(ABILITY_TO_UPGRADE.len() > 50, "Expected more than 50 research ability mappings");
    
    // Test some Terran research abilities
    assert!(get_upgrade_for_ability(AbilityId::BarracksTechLabResearchStimpack).is_some());
    assert!(get_upgrade_for_ability(AbilityId::ResearchCombatShield).is_some());
    assert!(get_upgrade_for_ability(AbilityId::EngineeringBayResearchTerranInfantryWeaponsLevel1).is_some());
    
    // Test some Protoss research abilities
    assert!(get_upgrade_for_ability(AbilityId::ForgeResearchProtossGroundWeaponsLevel1).is_some());
    assert!(get_upgrade_for_ability(AbilityId::ResearchWarpGate).is_some());
    assert!(get_upgrade_for_ability(AbilityId::ResearchCharge).is_some());
    
    // Test some Zerg research abilities  
    assert!(get_upgrade_for_ability(AbilityId::ResearchGlialRegeneration).is_some());
    assert!(get_upgrade_for_ability(AbilityId::ResearchTunnelingClaws).is_some());
}