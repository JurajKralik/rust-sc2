mod common;

use sc2pathlib::Map;

// Helper function to load test map data
fn load_choke_height_map() -> Map {
    let pathing = common::read_vec_from_file("tests/choke_height.txt");
    let height_map = common::read_vec_from_file("tests/choke_height.txt");
    let placement = pathing.clone();
    
    let width = pathing.len();
    let height = pathing[0].len();
    
    Map::new(
        pathing,
        placement,
        height_map,
        0,
        0,
        width,
        height,
    )
}

#[test]
fn test_tactical_positions_calculated() {
    let map = load_choke_height_map();
    
    let tactical_positions = map.tactical_positions();
    
    // Should have found some tactical positions
    assert!(!tactical_positions.is_empty(), "Should find tactical positions on the map");
    
    println!("Found {} tactical positions", tactical_positions.len());
    
    // Show top 5 positions
    for (i, pos) in tactical_positions.iter().take(5).enumerate() {
        println!(
            "#{}: pos=({:.1}, {:.1}), score={:.1}, high_ground={}, walkable_neighbors={}, chokes={}",
            i + 1,
            pos.position.0,
            pos.position.1,
            pos.score,
            pos.on_high_ground,
            pos.walkable_neighbors,
            pos.chokes_in_range
        );
    }
}

#[test]
fn test_tactical_positions_sorted() {
    let map = load_choke_height_map();
    
    let tactical_positions = map.tactical_positions();
    
    // Verify positions are sorted by score (descending)
    for i in 1..tactical_positions.len() {
        assert!(
            tactical_positions[i - 1].score >= tactical_positions[i].score,
            "Tactical positions should be sorted by score (highest first)"
        );
    }
}

#[test]
fn test_tactical_positions_on_walkable() {
    let map = load_choke_height_map();
    
    let tactical_positions = map.tactical_positions();
    
    // All tactical positions should be on walkable ground
    for pos in tactical_positions {
        let x = pos.position.0 as usize;
        let y = pos.position.1 as usize;
        
        assert!(
            x < map.ground_pathing.map.len() && y < map.ground_pathing.map[0].len(),
            "Position should be within map bounds"
        );
        
        assert!(
            map.ground_pathing.map[x][y] > 0,
            "Tactical position should be on walkable ground"
        );
    }
}

#[test]
fn test_custom_tactical_calculation() {
    let map = load_choke_height_map();
    
    // Calculate positions for a different unit (e.g., Liberator in siege mode)
    let liberator_positions = map.calculate_tactical_positions(1.5, 5.0);
    
    assert!(!liberator_positions.is_empty(), "Should find positions for custom unit specs");
    
    // Verify sorted
    for i in 1..liberator_positions.len() {
        assert!(
            liberator_positions[i - 1].score >= liberator_positions[i].score,
            "Custom tactical positions should be sorted by score"
        );
    }
}

#[test]
fn test_tactical_positions_have_good_scores() {
    let map = load_choke_height_map();
    
    let tactical_positions = map.tactical_positions();
    
    // All returned positions should have reasonable scores (threshold is 30.0)
    for pos in tactical_positions {
        assert!(
            pos.score > 30.0,
            "Tactical positions should have score > 30.0, got {}",
            pos.score
        );
    }
}

#[test]
fn test_high_ground_positions_score_better() {
    let map = load_choke_height_map();
    
    let tactical_positions = map.tactical_positions();
    
    // Find high ground and low ground positions
    let high_ground_positions: Vec<_> = tactical_positions
        .iter()
        .filter(|p| p.on_high_ground)
        .collect();
    
    let low_ground_positions: Vec<_> = tactical_positions
        .iter()
        .filter(|p| !p.on_high_ground)
        .collect();
    
    if !high_ground_positions.is_empty() && !low_ground_positions.is_empty() {
        // Average score of high ground should generally be higher
        let high_avg: f32 = high_ground_positions.iter().map(|p| p.score).sum::<f32>()
            / high_ground_positions.len() as f32;
        let low_avg: f32 = low_ground_positions.iter().map(|p| p.score).sum::<f32>()
            / low_ground_positions.len() as f32;
        
        println!("High ground avg score: {:.1}", high_avg);
        println!("Low ground avg score: {:.1}", low_avg);
        
        // High ground bonus should make high ground positions score better on average
        assert!(
            high_avg > low_avg,
            "High ground positions should score better on average"
        );
    }
}
