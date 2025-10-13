use rust_sc2::prelude::*;

#[test]
fn test_pathfinding_unit_type() {
    // Test that PathfindingUnitType enum is accessible
    let ground = PathfindingUnitType::Ground;
    let air = PathfindingUnitType::Air;
    let reaper = PathfindingUnitType::Reaper;
    let colossus = PathfindingUnitType::Colossus;
    
    // Just verify they can be used (this will compile if the enum is properly exported)
    println!("PathfindingUnitType variants: {:?} {:?} {:?} {:?}", ground, air, reaper, colossus);
}

#[cfg(test)]
mod pathfinding_tests {
    use super::*;

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
            assert!(!debug_str.is_empty());
        }
    }
}