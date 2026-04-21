use rust_sc2::prelude::*;
use sc2pathlib::{PathFind, Map, VisionUnit};

// This example demonstrates advanced features like influence mapping, 
// vision, and choke detection with rust_sc2

fn main() {
    println!("Advanced sc2pathlib features example");
    
    // Create a larger, more complex map
    let width = 50;
    let height = 50;
    let mut pathing_grid = vec![vec![1; height]; width];
    let mut placement_grid = vec![vec![1; height]; width];
    let mut height_map = vec![vec![0; height]; width];
    
    // Create a simple choke by blocking most of the middle
    for x in 20..30 {
        for y in 10..40 {
            if y < 24 || y > 26 {  // Leave a 3-tile wide choke
                pathing_grid[x][y] = 0;
                placement_grid[x][y] = 0;
            }
        }
    }
    
    // Add some height variation
    for x in 0..25 {
        for y in 0..height {
            height_map[x][y] = 1;
        }
    }
    for x in 25..width {
        for y in 0..height {
            height_map[x][y] = 2;
        }
    }
    
    // Create the map
    let mut game_map = Map::new(
        pathing_grid,
        placement_grid,
        height_map,
        0, 0, width, height
    );
    
    // Example 1: Calculate zones (useful for expansion detection)
    let base_locations = vec![
        (10.0, 25.0),  // Left base
        (40.0, 25.0),  // Right base
    ];
    game_map.calculate_zones(base_locations);
    
    println!("Zones calculated for base locations");
    
    // Example 2: Detect chokes
    let chokes = game_map.chokes();
    println!("Found {} chokes:", chokes.len());
    for (i, choke) in chokes.iter().enumerate() {
        let main_line = choke.main_line();
        println!("  Choke {}: from ({:.1}, {:.1}) to ({:.1}, {:.1}), min width: {:.2}",
                i, main_line.0.0, main_line.0.1, main_line.1.0, main_line.1.1, choke.min_length);
    }
    
    // Example 3: Add influence mapping for enemy units
    let mut pathfinder = PathFind::new(vec![vec![1; height]; width]);
    
    // Simulate enemy unit positions using Point2
    let enemy_positions = vec![
        Point2::new(35.0, 25.0),  // Enemy near right base
        Point2::new(30.0, 20.0),  // Enemy controlling choke area
    ];
    
    // Add influence around enemy positions (high cost to path near enemies)
    pathfinder.add_influence_point2(enemy_positions, 10.0, 8.0);
    
    // Example 4: Find safe path avoiding enemy influence
    let start = Point2::new(5.0, 25.0);
    let end = Point2::new(45.0, 25.0);
    
    // Path without considering influence
    let (normal_path, normal_distance) = pathfinder.find_path_point2(
        start, end, false, false, Some(1), None, None
    );
    
    // Path considering influence (safer but possibly longer)
    let (safe_path, safe_distance) = pathfinder.find_path_point2(
        start, end, false, true, Some(1), None, None
    );
    
    println!("\nPathfinding comparison:");
    println!("Normal path: {} steps, {:.2} distance", normal_path.len(), normal_distance);
    println!("Safe path (with influence): {} steps, {:.2} distance", safe_path.len(), safe_distance);
    
    // Example 5: Vision and detection
    game_map.clear_vision();
    
    // Add vision units (e.g., your own units providing sight)
    game_map.add_vision_unit_point2(false, false, Point2::new(10.0, 25.0), 9.0); // Ground unit
    game_map.add_vision_unit_point2(true, true, Point2::new(25.0, 25.0), 11.0);  // Flying detector
    
    game_map.calculate_vision_map();
    
    // Check vision status at various points
    let test_points = vec![
        Point2::new(10.0, 25.0),  // Should be visible
        Point2::new(15.0, 25.0),  // Should be visible
        Point2::new(40.0, 25.0),  // Might not be visible
    ];
    
    println!("\nVision status:");
    for point in test_points {
        let status = game_map.vision_status_point2(point);
        let status_str = match status {
            0 => "Not seen",
            1 => "Seen",
            2 => "Detected (by detector)",
            _ => "Unknown",
        };
        println!("  Point ({:.1}, {:.1}): {}", point.x, point.y, status_str);
    }
    
    // Example 6: Overlord spots (good for Zerg overlord placement)
    let overlord_spots = game_map.overlord_spots();
    println!("\nFound {} overlord spots:", overlord_spots.len());
    for (i, spot) in overlord_spots.iter().take(5).enumerate() {
        println!("  Spot {}: ({:.1}, {:.1})", i, spot.0, spot.1);
    }
    
    println!("\nAdvanced features demonstration complete!");
}

// Example integration with a rust_sc2 bot showing how to use influence mapping
// for unit positioning and threat avoidance:

/*
use rust_sc2::prelude::*;
use sc2pathlib::{Map, PathFind};

#[bot]
impl AdvancedBot {
    async fn on_step(&mut self) -> SC2Result<()> {
        self.update_influence_map().await?;
        self.position_units_safely().await?;
        Ok(())
    }
    
    async fn update_influence_map(&mut self) -> SC2Result<()> {
        if let Some(ref mut pathfinder) = self.ground_pathfinder {
            // Reset influence
            pathfinder.reset();
            
            // Add negative influence for enemy units
            let enemy_positions: Vec<Point2> = self.units.enemy.all
                .iter()
                .filter(|u| u.is_combat_unit()) // You'd implement this filter
                .map(|u| u.position())
                .collect();
            
            if !enemy_positions.is_empty() {
                pathfinder.add_influence_point2(enemy_positions, 15.0, 10.0);
            }
            
            // Add positive influence for friendly defensive structures
            let defensive_positions: Vec<Point2> = self.units.my.structures
                .iter()
                .filter(|u| u.type_id().is_defensive()) // You'd implement this
                .map(|u| u.position())
                .collect();
            
            if !defensive_positions.is_empty() {
                // Add negative influence (making paths more attractive near defenses)
                pathfinder.add_influence_point2(defensive_positions, -5.0, 8.0);
            }
        }
        
        Ok(())
    }
    
    async fn position_units_safely(&mut self) -> SC2Result<()> {
        if let Some(ref pathfinder) = self.ground_pathfinder {
            for unit in &self.units.my.units {
                if unit.is_idle() && unit.is_combat_unit() {
                    // Find a safe position for this unit
                    let current_pos = unit.position();
                    
                    // Look for low-influence positions within a reasonable distance
                    let destinations = pathfinder.find_destinations_in(
                        (current_pos.x as usize, current_pos.y as usize), 
                        15.0
                    );
                    
                    if let Some(best_pos) = destinations.iter()
                        .min_by_key(|(pos, _distance)| {
                            pathfinder.current_influence(*pos)
                        }) {
                        
                        let target = Point2::new(best_pos.0.0 as f32, best_pos.0.1 as f32);
                        if current_pos.distance(target) > 2.0 {
                            unit.move_to(target, false);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

struct AdvancedBot {
    ground_pathfinder: Option<PathFind>,
    game_map: Option<Map>,
}
*/