use rust_sc2::prelude::*;

// Mock bot for testing affordability methods
#[derive(Default)]
struct TestBot {
    minerals: u32,
    vespene: u32,
}

impl TestBot {
    fn new_with_resources(minerals: u32, vespene: u32) -> Self {
        Self { minerals, vespene }
    }
    
    // Simple mock implementation for testing
    fn can_afford_upgrade(&self, upgrade: UpgradeId) -> bool {
        // Mock upgrade costs for testing
        let (min_cost, gas_cost) = match upgrade {
            UpgradeId::Stimpack => (100, 100),
            UpgradeId::ShieldWall => (100, 0),
            UpgradeId::TerranInfantryWeaponsLevel1 => (100, 100),
            _ => (200, 200), // Default high cost
        };
        
        self.minerals >= min_cost && self.vespene >= gas_cost
    }
    
    fn can_afford_ability_research(&self, ability: AbilityId) -> Option<bool> {
        use rust_sc2::dicts::get_upgrade_for_ability;
        get_upgrade_for_ability(ability).map(|upgrade| self.can_afford_upgrade(upgrade))
    }
}

#[test]
fn test_can_afford_ability_research_with_resources() {
    let bot = TestBot::new_with_resources(200, 200);
    
    // Should be able to afford stimpack research (100/100)
    assert_eq!(
        bot.can_afford_ability_research(AbilityId::BarracksTechLabResearchStimpack),
        Some(true)
    );
    
    // Should be able to afford combat shield (100/0)
    assert_eq!(
        bot.can_afford_ability_research(AbilityId::ResearchCombatShield),
        Some(true)
    );
}

#[test]
fn test_can_afford_ability_research_without_resources() {
    let bot = TestBot::new_with_resources(50, 50);
    
    // Should not be able to afford stimpack research (100/100)
    assert_eq!(
        bot.can_afford_ability_research(AbilityId::BarracksTechLabResearchStimpack),
        Some(false)
    );
    
    // Should not be able to afford combat shield (100/0) - not enough minerals
    assert_eq!(
        bot.can_afford_ability_research(AbilityId::ResearchCombatShield),
        Some(false)
    );
}

#[test]
fn test_can_afford_ability_research_partial_resources() {
    let bot = TestBot::new_with_resources(150, 50);
    
    // Should be able to afford combat shield (100/0)
    assert_eq!(
        bot.can_afford_ability_research(AbilityId::ResearchCombatShield),
        Some(true)
    );
    
    // Should not be able to afford stimpack (100/100) - not enough gas
    assert_eq!(
        bot.can_afford_ability_research(AbilityId::BarracksTechLabResearchStimpack),
        Some(false)
    );
}

#[test]
fn test_can_afford_ability_research_unknown_ability() {
    let bot = TestBot::new_with_resources(1000, 1000);
    
    // Move is not a research ability, should return None
    assert_eq!(
        bot.can_afford_ability_research(AbilityId::Move),
        None
    );
    
    // Attack is not a research ability, should return None  
    assert_eq!(
        bot.can_afford_ability_research(AbilityId::Attack),
        None
    );
}