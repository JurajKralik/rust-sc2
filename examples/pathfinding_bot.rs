use rust_sc2::prelude::*;

#[bot]
#[derive(Default)]
struct PathfindingBot;

impl Player for PathfindingBot {
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Terran)
            .with_name("PathfindingBot")
    }

    fn on_start(&mut self) -> SC2Result<()> {
        // Initialize pathfinding at the start of the game
        println!("Initializing pathfinding...");
        self.init_pathfinding();
        
        if self.is_pathfinding_initialized() {
            println!("Pathfinding initialized successfully!");
        } else {
            println!("Failed to initialize pathfinding!");
        }

        Ok(())
    }

    fn on_step(&mut self, iteration: usize) -> SC2Result<()> {
        // Example pathfinding usage
        if iteration == 10 && self.is_pathfinding_initialized() {
            // Find path from start location to enemy start location
            let start = self.start_location;
            let end = self.enemy_start;
            
            match self.get_path(start, end, PathfindingUnitType::Ground, false, false) {
                Some((path, distance)) => {
                    println!(
                        "Found path from start to enemy start: {} waypoints, {:.2} distance", 
                        path.len(), 
                        distance
                    );
                    
                    // Print first few waypoints
                    for (i, point) in path.iter().take(5).enumerate() {
                        println!("  Waypoint {}: ({:.1}, {:.1})", i, point.x, point.y);
                    }
                    if path.len() > 5 {
                        println!("  ... and {} more waypoints", path.len() - 5);
                    }
                }
                None => {
                    println!("No path found from start to enemy start!");
                }
            }
        }
        
        // Example: Find paths for workers to mineral patches
        if iteration == 20 && self.is_pathfinding_initialized() {
            for worker in self.units.my.workers.iter().take(3) {
                if let Some(mineral) = self.units.mineral_fields.closest(worker.position()) {
                    match self.get_path(
                        worker.position(),
                        mineral.position(),
                        PathfindingUnitType::Ground,
                        false,
                        false
                    ) {
                        Some((path, distance)) => {
                            println!(
                                "Worker {} -> Mineral: {} steps, {:.2} distance",
                                worker.tag(),
                                path.len(),
                                distance
                            );
                        }
                        None => {
                            println!("Worker {} cannot reach mineral!", worker.tag());
                        }
                    }
                }
            }
        }

        // Example: Update pathfinding with new buildings
        if iteration % 100 == 0 && iteration > 0 {
            self.update_pathfinding_buildings();
            println!("Updated pathfinding with current buildings (iteration {})", iteration);
        }

        // Example: Advanced pathfinding with different unit types
        if iteration == 50 && self.is_pathfinding_initialized() {
            let start = self.start_location;
            let end = self.enemy_start;
            
            // Test different unit types
            let unit_types = [
                ("Ground", PathfindingUnitType::Ground),
                ("Air", PathfindingUnitType::Air),
                ("Reaper", PathfindingUnitType::Reaper),
                ("Colossus", PathfindingUnitType::Colossus),
            ];
            
            for (name, unit_type) in &unit_types {
                match self.get_path(start, end, *unit_type, false, false) {
                    Some((path, distance)) => {
                        println!("{} path: {} waypoints, {:.2} distance", name, path.len(), distance);
                    }
                    None => {
                        println!("{} path: No path found!", name);
                    }
                }
            }
        }

        Ok(())
    }

    fn on_end(&self, result: GameResult) -> SC2Result<()> {
        println!("Game ended with result: {:?}", result);
        Ok(())
    }
}

fn main() -> SC2Result<()> {
    run_vs_computer(
        &mut PathfindingBot::default(),
        Computer::new(Race::Random, Difficulty::VeryEasy, None),
        "Simple64",
        LaunchOptions::default(),
    )
}