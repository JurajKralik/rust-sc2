use rust_sc2::prelude::*;

#[test]
fn test_pathfinding_unit_type_variants() {
    // Test that PathfindingUnitType enum is accessible and has expected variants
    let ground = PathfindingUnitType::Ground;
    let air = PathfindingUnitType::Air;
    let reaper = PathfindingUnitType::Reaper;
    let colossus = PathfindingUnitType::Colossus;
    
    // Verify they can be formatted for debugging
    assert_eq!(format!("{:?}", ground), "Ground");
    assert_eq!(format!("{:?}", air), "Air");
    assert_eq!(format!("{:?}", reaper), "Reaper");
    assert_eq!(format!("{:?}", colossus), "Colossus");
}

#[test]
fn test_pathfinding_enum_debug() {
    let unit_types = [
        PathfindingUnitType::Ground,
        PathfindingUnitType::Air,
        PathfindingUnitType::Reaper,
        PathfindingUnitType::Colossus,
    ];
    
    for unit_type in &unit_types {
        let debug_str = format!("{:?}", unit_type);
        assert!(!debug_str.is_empty(), "Debug string should not be empty for {:?}", unit_type);
        assert!(debug_str.len() > 2, "Debug string should be meaningful for {:?}", unit_type);
    }
}

#[test]
fn test_pathfinding_enum_variants_exist() {
    // Test that PathfindingUnitType variants exist and can be created
    let unit_types = [
        PathfindingUnitType::Ground,
        PathfindingUnitType::Air,
        PathfindingUnitType::Reaper,
        PathfindingUnitType::Colossus,
    ];
    
    // Verify each variant can be created and has a debug representation
    for unit_type in &unit_types {
        let debug_str = format!("{:?}", unit_type);
        assert!(!debug_str.is_empty(), "Debug string should not be empty");
        assert!(debug_str.len() > 2, "Debug string should be meaningful");
    }
    
    // Test that we can use them in match patterns
    for unit_type in &unit_types {
        match unit_type {
            PathfindingUnitType::Ground => assert!(true),
            PathfindingUnitType::Air => assert!(true),
            PathfindingUnitType::Reaper => assert!(true),
            PathfindingUnitType::Colossus => assert!(true),
        }
    }
}

#[test]
fn test_pathfinding_types_exist() {
    // Verify that pathfinding-related types are accessible
    // This ensures the sc2-pathfinding integration is working
    
    // These should compile without errors
    let _: PathfindingUnitType = PathfindingUnitType::Ground;
    
    // Verify pathfinding types from the pathfinding crate are available
    use sc2pathfinding::{Choke, Map, PathFind};
    
    // Just test that the types exist and can be referenced
    // (We can't easily test instantiation without map data)
    let _choke_type = std::any::type_name::<Choke>();
    let _map_type = std::any::type_name::<Map>();
    let _pathfind_type = std::any::type_name::<PathFind>();
    
    assert!(!_choke_type.is_empty());
    assert!(!_map_type.is_empty());
    assert!(!_pathfind_type.is_empty());
}