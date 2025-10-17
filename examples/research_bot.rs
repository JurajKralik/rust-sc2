use rust_sc2::prelude::*;

#[bot]
#[derive(Default)]
struct ResearchBot;

impl Player for ResearchBot {
    fn get_player_settings(&self) -> PlayerSettings<'_> {
        PlayerSettings::new(Race::Terran).with_name("ResearchBot")
    }

    fn on_start(&mut self) -> SC2Result<()> {
        println!("Research bot starting!");
        
        // Test the new can_afford_ability_research method
        println!("Testing ability-to-upgrade research affordability checks...");
        
        // Test some common research abilities
        let test_abilities = vec![
            AbilityId::BarracksTechLabResearchStimpack,
            AbilityId::ResearchCombatShield,
            AbilityId::EngineeringBayResearchTerranInfantryWeaponsLevel1,
            AbilityId::ArmoryResearchTerranVehicleWeaponsLevel1,
            AbilityId::ResearchBansheeCloakingField,
        ];
        
        for ability in test_abilities {
            match self.can_afford_ability_research(ability) {
                Some(true) => println!("✓ Can afford research for ability {:?}", ability),
                Some(false) => println!("✗ Cannot afford research for ability {:?}", ability),
                None => println!("? No upgrade mapping found for ability {:?}", ability),
            }
        }
        
        // Compare with direct upgrade checks
        println!("\nComparing with direct upgrade checks:");
        if let Some(can_afford_stimpack) = self.can_afford_ability_research(AbilityId::BarracksTechLabResearchStimpack) {
            let direct_check = self.can_afford_upgrade(UpgradeId::Stimpack);
            println!("Stimpack via ability: {}, via upgrade: {} (should match: {})", 
                can_afford_stimpack, direct_check, can_afford_stimpack == direct_check);
        }
        
        Ok(())
    }

    fn on_step(&mut self, iteration: usize) -> SC2Result<()> {
        // Demonstrate periodic research checks
        if iteration % 100 == 0 {
            println!("\n--- Iteration {} Research Status ---", iteration);
            
            // Check if we can afford some basic upgrades
            let research_priorities = vec![
                (AbilityId::BarracksTechLabResearchStimpack, "Stimpack"),
                (AbilityId::ResearchCombatShield, "Combat Shield"),
                (AbilityId::EngineeringBayResearchTerranInfantryWeaponsLevel1, "Infantry Weapons +1"),
            ];
            
            for (ability, name) in research_priorities {
                match self.can_afford_ability_research(ability) {
                    Some(true) => println!("💰 Can research: {}", name),
                    Some(false) => println!("💸 Need more resources for: {}", name),
                    None => println!("❓ Unknown research: {}", name),
                }
            }
            
            println!("Current resources: {} minerals, {} vespene", self.minerals, self.vespene);
        }
        
        // Exit after a reasonable time to keep the demo short
        if iteration > 500 {
            self.leave()
        } else {
            Ok(())
        }
    }

    fn on_end(&self, result: GameResult) -> SC2Result<()> {
        println!("Research bot ended with result: {:?}", result);
        Ok(())
    }
}

fn main() -> SC2Result<()> {
    run_vs_computer(
        &mut ResearchBot::default(),
        Computer::new(Race::Random, Difficulty::VeryEasy, None),
        "Simple64",
        LaunchOptions::default(),
    )
}