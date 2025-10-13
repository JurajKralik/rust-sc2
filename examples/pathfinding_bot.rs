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
		println!("Pathfinding bot starting!");
		
		// Initialize pathfinding system
		self.init_pathfinding();
		println!("Pathfinding initialized for ground units");
		
		// Calculate zones based on start locations
		self.calculate_zones_from_start_locations();
		println!("Zones calculated based on {} start locations", 
			self.game_info.start_locations.len());

		// Print choke information
		if let Some(chokes) = self.get_chokes() {
			println!("Found {} choke points on the map", chokes.len());
			for (i, choke) in chokes.iter().enumerate().take(3) { // Print first 3 chokes
				let center = choke.center();
				println!("  Choke {}: center at ({:.1}, {:.1}), width {:.1}", 
					i, center.0, center.1, choke.get_min_length());
			}
		}
		
		Ok(())
	}

    fn on_step(&mut self, iteration: usize) -> SC2Result<()> {
		// Demonstrate pathfinding every 50 iterations
		if iteration % 50 == 0 && !self.units.my.workers.is_empty() {
			let worker = self.units.my.workers.first().unwrap();
			let start = worker.position();
			
			// Try to find path to the center of the map
			let map_center = Point2::new(
				(self.game_info.playable_area.x0 + self.game_info.playable_area.x1) as f32 / 2.0,
				(self.game_info.playable_area.y0 + self.game_info.playable_area.y1) as f32 / 2.0
			);
			
			match self.get_path(start, map_center, PathfindingUnitType::Ground, false, false) {
				Some((path, distance)) if !path.is_empty() => {
					println!("Found path from worker to map center with {} steps, distance {:.1}", path.len(), distance);
					
					// Demonstrate zone analysis
					if let Some(start_zone) = self.get_zone(start) {
						if let Some(end_zone) = self.get_zone(map_center) {
							println!("Worker is in zone {}, map center is in zone {}", 
								start_zone, end_zone);
							
							if start_zone != end_zone {
								// Find nearest choke
								if let Some((choke_idx, distance)) = self.get_closest_choke(start) {
									println!("Nearest choke is #{} at distance {:.1}", choke_idx, distance);
									
									// Find all chokes within 20 units
									if let Some(nearby_chokes) = self.get_chokes_near(start, 20.0) {
										println!("Found {} chokes within 20 units", nearby_chokes.len());
									}
								}
							}
						}
					}
					
					// Demonstrate advanced pathfinding with different options
					match self.get_path_advanced(start, map_center, PathfindingUnitType::Air, false, false, None, None, Some(50.0)) {
						Some((air_path, air_distance)) => {
							println!("Air path has {} steps, {:.1} distance (vs {} ground steps, {:.1} distance)", 
								air_path.len(), air_distance, path.len(), distance);
						}
						None => println!("No air path found"),
					}
				}
				_ => println!("No path found from worker to map center"),
			}
		}
		
		// Demonstrate vision system every 100 iterations
		if iteration % 100 == 0 {
			println!("Updating pathfinding with current buildings...");
			self.update_pathfinding_buildings();
			
			// Update vision map with all units
			self.update_vision_with_units(true, true);
			
			// Check vision status at a few key positions
			let map_center = Point2::new(
				(self.game_info.playable_area.x0 + self.game_info.playable_area.x1) as f32 / 2.0,
				(self.game_info.playable_area.y0 + self.game_info.playable_area.y1) as f32 / 2.0
			);
			let test_positions = vec![
				self.start_location,
				map_center,
			];
			
			for pos in test_positions {
				match self.vision_status_at(pos) {
					Some(0) => println!("Position {:.1},{:.1}: No vision", pos.x, pos.y),
					Some(1) => println!("Position {:.1},{:.1}: Visible", pos.x, pos.y),
					Some(2) => println!("Position {:.1},{:.1}: Detected", pos.x, pos.y),
					Some(level) => println!("Position {:.1},{:.1}: Vision level {}", pos.x, pos.y, level),
					None => println!("Position {:.1},{:.1}: Vision check failed", pos.x, pos.y),
				}
			}
			
			// Get all visible positions (sampled)
			if let Some(visible_positions) = self.get_visible_positions(5.0) {
				println!("Currently have vision of {} positions", visible_positions.len());
			}
			
			// Check for fog of war around main base
			if let Some(fog_positions) = self.get_fog_of_war_positions(self.start_location, 15.0) {
				println!("Found {} positions in fog of war near main base", fog_positions.len());
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