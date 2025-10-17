use rust_sc2::prelude::*;

#[bot]
#[derive(Default)]
struct SiegeTankBot {
    siege_positions_evaluated: bool,
}

impl Player for SiegeTankBot {
    fn get_player_settings(&self) -> PlayerSettings<'_> {
        PlayerSettings::new(Race::Terran)
    }

    fn on_start(&mut self) -> SC2Result<()> {
        println!("=== Siege Tank Tactical Positioning Bot ===");
        println!("This bot demonstrates the tactical positioning system for siege units.\n");
        Ok(())
    }

    fn on_step(&mut self, iteration: usize) -> SC2Result<()> {
        // Evaluate tactical positions once at the start
        if !self.siege_positions_evaluated && iteration == 10 {
            self.evaluate_tactical_positions();
            self.siege_positions_evaluated = true;
        }

        // Basic bot logic: build workers and supply
        if self.can_afford(UnitTypeId::SCV, false) && self.counter().count(UnitTypeId::SCV) < 20 {
            if let Some(cc) = self.units.my.townhalls.first() {
                cc.train(UnitTypeId::SCV, false);
            }
        }

        if self.supply_left < 4 && self.can_afford(UnitTypeId::SupplyDepot, false) {
            if let Some(worker) = self.units.my.workers.first() {
                if let Some(location) = self.find_placement(
                    UnitTypeId::SupplyDepot,
                    self.start_location,
                    PlacementOptions::default(),
                ) {
                    worker.build(UnitTypeId::SupplyDepot, location, false);
                }
            }
        }

        // Build Factory if we can afford it
        if self.counter().count(UnitTypeId::Factory) == 0
            && self.counter().count(UnitTypeId::Barracks) > 0
            && self.can_afford(UnitTypeId::Factory, false)
        {
            if let Some(worker) = self.units.my.workers.first() {
                if let Some(location) = self.find_placement(
                    UnitTypeId::Factory,
                    self.start_location,
                    PlacementOptions::default(),
                ) {
                    worker.build(UnitTypeId::Factory, location, false);
                }
            }
        }

        // Train Siege Tanks
        if let Some(factory) = self.units.my.structures.of_type(UnitTypeId::Factory).first() {
            if factory.is_ready() && self.can_afford(UnitTypeId::SiegeTank, false) {
                factory.train(UnitTypeId::SiegeTank, false);
            }
        }

        // Position Siege Tanks at tactical locations
        self.position_siege_tanks();

        Ok(())
    }
}

impl SiegeTankBot {
    fn evaluate_tactical_positions(&mut self) {
        println!("\n=== Evaluating Tactical Siege Positions ===\n");

        // Get all tactical positions (auto-initialized)
        let tactical_positions = self.get_tactical_positions_lazy();

        println!("Found {} tactical positions on the map\n", tactical_positions.len());

        // Display top 10 positions
        println!("Top 10 Tactical Positions:");
        println!("{:-<100}", "");
        println!(
            "{:<5} {:<15} {:<10} {:<12} {:<18} {:<15} {:<10}",
            "#", "Position", "Score", "High Ground", "Walkable Nearby", "Chokes in Range", "Height"
        );
        println!("{:-<100}", "");

        for (i, pos) in tactical_positions.iter().take(10).enumerate() {
            println!(
                "{:<5} ({:>5.1}, {:>5.1})  {:<10.1} {:<12} {:<18} {:<15} {:<10}",
                i + 1,
                pos.position.0,
                pos.position.1,
                pos.score,
                if pos.on_high_ground { "Yes" } else { "No" },
                pos.walkable_neighbors,
                pos.chokes_in_range,
                pos.height
            );
        }
        println!("{:-<100}\n", "");

        // Find tactical positions near our main base
        if let Some(nearby_positions) = 
            self.get_tactical_positions_near(self.start_location, 30.0) 
        {
            println!("\nFound {} tactical positions near main base (within 30.0 range)", 
                nearby_positions.len());
            
            for (i, pos) in nearby_positions.iter().take(3).enumerate() {
                println!(
                    "  {}. Position ({:.1}, {:.1}) - Score: {:.1}",
                    i + 1,
                    pos.position.0,
                    pos.position.1,
                    pos.score
                );
            }
        }

        // Calculate custom positions for a different unit (example)
        if let Some(custom_positions) = self.calculate_tactical_positions(2.0, 9.0) {
            println!(
                "\nCalculated {} custom tactical positions for unit with radius=2.0, range=9.0",
                custom_positions.len()
            );
        }

        println!("\n=== Evaluation Complete ===\n");
    }

    fn position_siege_tanks(&mut self) {
        // Get our siege tanks
        let siege_tanks: Vec<_> = self
            .units
            .my
            .units
            .iter()
            .filter(|u| u.type_id() == UnitTypeId::SiegeTank || u.type_id() == UnitTypeId::SiegeTankSieged)
            .collect();

        if siege_tanks.is_empty() {
            return;
        }

        // Get tactical positions
        let tactical_positions = match self.get_tactical_positions() {
            Some(positions) => positions,
            None => return,
        };

        if tactical_positions.is_empty() {
            return;
        }

        // For each unsieged tank, find a good position
        for tank in siege_tanks.iter() {
            // Skip if already sieged
            if tank.type_id() == UnitTypeId::SiegeTankSieged {
                continue;
            }

            // Find closest unoccupied tactical position
            let mut best_position: Option<&sc2pathfinding::TacticalPosition> = None;
            let mut best_distance = f32::MAX;

            for tac_pos in tactical_positions.iter().take(20) {
                // Check if position is not already occupied by another tank
                let occupied = siege_tanks.iter().any(|t| {
                    t.position().distance(tac_pos.position) < 3.0
                });

                if !occupied {
                    let distance = tank.position().distance(tac_pos.position);
                    if distance < best_distance {
                        best_distance = distance;
                        best_position = Some(tac_pos);
                    }
                }
            }

            // Move to the position if found and not already there
            if let Some(target_pos) = best_position {
                let target_point = Point2::new(target_pos.position.0, target_pos.position.1);
                
                if tank.position().distance(target_pos.position) > 2.0 {
                    // Move to position
                    tank.move_to(Target::Pos(target_point), false);
                } else {
                    // We're at the position, siege up
                    tank.command(AbilityId::SiegeModeSiegeMode, Target::None, false);
                }
            }
        }
    }
}

fn main() -> SC2Result<()> {
    let mut bot = SiegeTankBot::default();
    run_vs_computer(
        &mut bot,
        Computer::new(Race::Random, Difficulty::Easy, None),
        "AutomatonLE",
        LaunchOptions::default(),
    )
}
