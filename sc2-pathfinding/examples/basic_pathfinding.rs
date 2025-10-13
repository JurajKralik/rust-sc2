use rust_sc2::prelude::*;
use sc2pathlib::{PathFind, Map};

// This example demonstrates basic pathfinding using the sc2pathlib with rust_sc2
// Note: This is a simplified example showing the API usage

fn main() {
    // Example map data (normally you'd get this from rust_sc2's game_info)
    let width = 10;
    let height = 10;
    
    // Create a simple pathable map (1 = pathable, 0 = not pathable)
    let mut map_data = vec![vec![1; height]; width];
    
    // Add some obstacles
    map_data[4][4] = 0;
    map_data[4][5] = 0;
    map_data[5][4] = 0;
    map_data[5][5] = 0;
    
    // Create pathfinder
    let pathfinder = PathFind::new(map_data);
    
    // Find path using traditional coordinates
    let start = (1, 1);
    let end = (8, 8);
    let (path, distance) = pathfinder.find_path_basic(start, end, Some(1)); // Use octile distance
    
    println!("Path found with {} steps, distance: {:.2}", path.len(), distance);
    for (i, point) in path.iter().enumerate() {
        println!("Step {}: ({}, {})", i, point.0, point.1);
    }
    
    // Find path using rust_sc2::Point2
    let start_point2 = Point2::new(1.0, 1.0);
    let end_point2 = Point2::new(8.0, 8.0);
    let (path_point2, distance2) = pathfinder.find_path_point2(
        start_point2,
        end_point2,
        false,  // large unit
        false,  // influence
        Some(1), // octile distance heuristic
        None,   // no window restriction
        None    // no distance limit
    );
    
    println!("\nPath with Point2 found with {} steps, distance: {:.2}", path_point2.len(), distance2);
    for (i, point) in path_point2.iter().enumerate() {
        println!("Step {}: ({:.1}, {:.1})", i, point.x, point.y);
    }
}

// Example of how to integrate with rust_sc2 Bot
// This would be used within your bot implementation:

/*
use rust_sc2::prelude::*;
use sc2pathlib::Map;

#[bot]
impl MyBot {
    async fn on_start(&mut self) -> SC2Result<()> {
        // Initialize pathfinding map from game info
        let pathing_grid = &self.game_info.pathing;
        let placement_grid = &self.game_info.placement;
        let height_map = &self.game_info.terrain_height;
        let playable = &self.game_info.playable_area;
        
        // Convert numpy-like arrays to Vec<Vec<usize>>
        let pathing_vec = convert_grid_to_vec(pathing_grid);  // You'd implement this
        let placement_vec = convert_grid_to_vec(placement_grid);
        let height_vec = convert_grid_to_vec(height_map);
        
        self.pathfinding_map = Some(Map::new(
            pathing_vec,
            placement_vec, 
            height_vec,
            playable.x as usize,
            playable.y as usize,
            (playable.x + playable.width) as usize,
            (playable.y + playable.height) as usize,
        ));
        
        Ok(())
    }
    
    async fn on_step(&mut self) -> SC2Result<()> {
        if let Some(ref pathfinding_map) = self.pathfinding_map {
            // Example: Find path for a worker to a mineral patch
            if let Some(worker) = self.units.my.workers.first() {
                if let Some(mineral) = self.units.resources.minerals.closest(worker.position()) {
                    let (path, distance) = pathfinding_map.find_path_point2(
                        0, // ground pathing map
                        worker.position(),
                        mineral.position(),
                        false, // not large unit
                        false, // no influence
                        Some(1), // octile distance
                        None, // no window restriction
                        None  // no distance limit
                    );
                    
                    if !path.is_empty() {
                        println!("Found path from worker to mineral: {} steps, {:.2} distance", 
                                path.len(), distance);
                        
                        // Move worker to first path point
                        if let Some(next_point) = path.get(1) {
                            worker.move_to(*next_point, false);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

struct MyBot {
    pathfinding_map: Option<Map>,
}
*/