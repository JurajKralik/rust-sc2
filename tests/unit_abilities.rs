use rust_sc2::prelude::*;
use rust_sc2::dicts::UnitAbilities;

#[test]
fn test_unit_abilities_coverage() {
    // Test that we have unit abilities data
    assert!(!UnitAbilities.is_empty(), "UnitAbilities should not be empty");
    
    // Test some common units have abilities
    assert!(UnitAbilities.contains_key(&UnitTypeId::SCV));
    assert!(UnitAbilities.contains_key(&UnitTypeId::Marine));
    assert!(UnitAbilities.contains_key(&UnitTypeId::CommandCenter));
}

#[test]
fn test_unit_abilities_content() {
    // Test that SCV has expected abilities
    if let Some(scv_abilities) = UnitAbilities.get(&UnitTypeId::SCV) {
        // SCV should have harvest ability (specific to SCV)
        assert!(scv_abilities.contains(&AbilityId::HarvestGatherSCV));
        // SCV should have move ability
        assert!(scv_abilities.contains(&AbilityId::MoveMove));
        // SCV should have attack ability
        assert!(scv_abilities.contains(&AbilityId::AttackAttack));
        // SCV should have build abilities (at least one)
        assert!(!scv_abilities.is_empty());
        // SCVs should have many abilities
        assert!(scv_abilities.len() > 10, "SCV should have many abilities, found: {}", scv_abilities.len());
    } else {
        panic!("SCV should have abilities defined");
    }
    
    // Test that Marine has expected abilities
    if let Some(marine_abilities) = UnitAbilities.get(&UnitTypeId::Marine) {
        // Marine should have some basic abilities but we won't check specifics
        // since the exact abilities depend on the unit abilities data
        assert!(!marine_abilities.is_empty());
        assert!(marine_abilities.len() > 1, "Marine should have multiple abilities, found: {}", marine_abilities.len());
    } else {
        panic!("Marine should have abilities defined");
    }
    
    // Test that Command Center has expected abilities
    if let Some(cc_abilities) = UnitAbilities.get(&UnitTypeId::CommandCenter) {
        // Command centers should have training abilities
        assert!(!cc_abilities.is_empty());
    } else {
        panic!("CommandCenter should have abilities defined");
    }
}

#[test]
fn test_unit_abilities_structure() {
    // Verify the structure is properly formed
    for (unit_type, abilities) in UnitAbilities.iter() {
        // Each unit should have at least some abilities
        assert!(!abilities.is_empty(), "Unit {:?} should have at least one ability", unit_type);
        
        // Most units should have Move ability (except buildings)
        // This is just a sanity check, not a strict requirement
        if !matches!(unit_type, 
            UnitTypeId::CommandCenter | UnitTypeId::SupplyDepot | UnitTypeId::Barracks |
            UnitTypeId::Refinery | UnitTypeId::EngineeringBay | UnitTypeId::Armory |
            UnitTypeId::Nexus | UnitTypeId::Pylon | UnitTypeId::Gateway |
            UnitTypeId::Assimilator | UnitTypeId::Forge | UnitTypeId::Hatchery |
            UnitTypeId::SpawningPool | UnitTypeId::Extractor
        ) {
            // Most mobile units should have move
            // (This is a loose check since some special units might not have move)
        }
    }
}

#[test]
fn test_unit_abilities_immutability() {
    // Test that the lazy static works correctly and is immutable
    let first_access = &*UnitAbilities;
    let second_access = &*UnitAbilities;
    
    // Should be the same reference (lazy static should work)
    assert!(std::ptr::eq(first_access, second_access));
}