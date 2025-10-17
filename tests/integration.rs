use rust_sc2::prelude::*;

#[test]
fn test_basic_integration() {
    // Test that we can import and use basic types together
    
    // IDs should be accessible
    let _unit_id = UnitTypeId::Marine;
    let _ability_id = AbilityId::Move;
    let _upgrade_id = UpgradeId::Stimpack;
    
    // Geometry types should work
    let point = Point2::new(10.0, 20.0);
    assert_eq!(point.x, 10.0);
    assert_eq!(point.y, 20.0);
    
    // Player types should be accessible
    let _race = Race::Terran;
    let _difficulty = Difficulty::VeryEasy;
}

#[test]
fn test_prelude_exports() {
    // Verify that all expected types are available in prelude
    
    // Core gameplay types
    let _unit_type = UnitTypeId::SCV;
    let _ability = AbilityId::Attack;
    let _upgrade = UpgradeId::Stimpack;
    let _race = Race::Protoss;
    let _alliance = Alliance::Own;
    
    // Geometry
    let _point = Point2::new(0.0, 0.0);
    
    // Game state types
    let _result = GameResult::Victory;
    let _difficulty = Difficulty::Medium;
    
    // Pathfinding types
    let _unit_type_pf = PathfindingUnitType::Ground;
}

#[test]
fn test_ids_are_numbers() {
    // Test that IDs have numeric values (can be converted to numbers)
    use num_traits::ToPrimitive;
    
    let marine_id = UnitTypeId::Marine;
    let move_id = AbilityId::Move;
    let stimpack_id = UpgradeId::Stimpack;
    
    // Should be able to convert to numbers
    assert!(marine_id.to_u32().is_some());
    assert!(move_id.to_u32().is_some());
    assert!(stimpack_id.to_u32().is_some());
    
    // IDs should have reasonable values (not 0, not crazy large)
    assert!(marine_id.to_u32().unwrap() > 0);
    assert!(marine_id.to_u32().unwrap() < 10000);
}

#[test]
fn test_dicts_integration() {
    // Test that dicts module exports work correctly
    use rust_sc2::dicts::{UnitAbilities, get_upgrade_for_ability};
    
    // Unit abilities should be accessible
    assert!(!UnitAbilities.is_empty());
    
    // Research abilities function should work
    let stimpack_mapping = get_upgrade_for_ability(AbilityId::BarracksTechLabResearchStimpack);
    assert_eq!(stimpack_mapping, Some(UpgradeId::Stimpack));
    
    let non_research = get_upgrade_for_ability(AbilityId::Move);
    assert_eq!(non_research, None);
}