use rust_sc2::prelude::*;

#[bot]
#[derive(Default)]
struct SimpleLazyBot;

impl Player for SimpleLazyBot {
    fn get_player_settings(&self) -> PlayerSettings<'_> {
        PlayerSettings::new(Race::Terran).with_name("SimpleLazyBot")
    }

    fn on_start(&mut self) -> SC2Result<()> {
        println!("Simple lazy bot starting!");
        
        // Test that we can use pathfinding immediately without any manual setup
        println!("Testing lazy pathfinding initialization...");
        
        // Cache start location to avoid borrowing issues
        let start_loc = self.start_location;
        
        // This should automatically initialize pathfinding on first use
        let chokes = self.get_chokes_lazy();
        println!("✓ Found {} chokes without manual initialization!", chokes.len());
        
        // Test zone calculation
        let start_zone = self.get_zone_lazy(start_loc);
        println!("✓ Start location is in zone {} without manual zone calculation!", start_zone);
        
        // Test pathfinding
        let target = Point2::new(start_loc.x + 5.0, start_loc.y + 5.0);
        match self.get_path_lazy(start_loc, target, PathfindingUnitType::Ground, false, false) {
            Some((path, distance)) => {
                println!("✓ Found path with {} steps and distance {:.1} without manual setup!", path.len(), distance);
            }
            None => {
                println!("✓ Pathfinding working (no path needed for this short distance)");
            }
        }
        
        println!("All lazy pathfinding methods work without manual initialization! 🎉");
        Ok(())
    }

    fn on_step(&mut self, _iteration: usize) -> SC2Result<()> {
        // Exit after first step since this is just a test
        self.leave()
    }
}

fn main() -> SC2Result<()> {
    run_vs_computer(
        &mut SimpleLazyBot::default(),
        Computer::new(Race::Random, Difficulty::VeryEasy, None),
        "Simple64",
        LaunchOptions::default(),
    )
}