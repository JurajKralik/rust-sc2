# Advanced Pathfinding Features for rust-sc2

The rust-sc2 library has been enhanced with advanced pathfinding capabilities through integration with the sc2-pathfinding library. This provides sophisticated navigation features that go far beyond the basic SC2 API.

## New Bot Methods Added

### Zone Management

#### `calculate_zones(base_locations: Vec<Point2>)`
Calculates strategic zones based on provided base locations. Zones are connected areas separated by obstacles.

```rust
// Calculate zones using custom locations
let strategic_points = vec![
    Point2::new(50.0, 50.0),
    Point2::new(100.0, 100.0),
];
bot.calculate_zones(strategic_points);
```

#### `calculate_zones_from_start_locations()`
Convenience method that automatically calculates zones using all start locations from game info.

```rust
bot.calculate_zones_from_start_locations();
```

#### `get_zone(position: Point2) -> Option<i8>`
Returns the zone ID for a given position.

```rust
if let Some(zone) = bot.get_zone(unit.position()) {
    println!("Unit is in zone {}", zone);
}
```

### Choke Point Detection

#### `get_chokes() -> Option<&Vec<sc2pathfinding::Choke>>`
Returns all detected choke points on the map.

```rust
if let Some(chokes) = bot.get_chokes() {
    println!("Found {} choke points", chokes.len());
    for (i, choke) in chokes.iter().enumerate() {
        let center = choke.center();
        println!("Choke {}: center ({:.1}, {:.1}), width {:.1}", 
            i, center.0, center.1, choke.get_min_length());
    }
}
```

#### `get_chokes_near(position: Point2, max_distance: f32) -> Option<Vec<(usize, f32)>>`
Finds choke points within a certain distance, sorted by proximity.

```rust
if let Some(nearby_chokes) = bot.get_chokes_near(unit.position(), 20.0) {
    for (choke_idx, distance) in nearby_chokes {
        println!("Choke {} is {:.1} units away", choke_idx, distance);
    }
}
```

#### `get_closest_choke(position: Point2) -> Option<(usize, f32)>`
Returns the index and distance to the closest choke point.

```rust
if let Some((choke_idx, distance)) = bot.get_closest_choke(unit.position()) {
    println!("Closest choke is #{} at distance {:.1}", choke_idx, distance);
}
```

### Vision System

#### `add_vision_unit(position: Point2, sight_range: f32, detector: bool, flying: bool)`
Adds a unit to the vision calculation system.

```rust
bot.add_vision_unit(
    unit.position(),
    unit.sight_range(),
    unit.is_detector(),
    unit.is_flying()
);
```

#### `clear_vision()` & `calculate_vision_map()`
Clears all vision units and recalculates the complete vision map.

```rust
bot.clear_vision();
// Add vision units...
bot.calculate_vision_map();
```

#### `vision_status_at(position: Point2) -> Option<usize>`
Checks the vision level at a specific position:
- `0`: Not visible
- `1`: Visible (has sight)
- `2`: Detected (has detection for cloaked units)

```rust
match bot.vision_status_at(position) {
    Some(0) => println!("Position is not visible"),
    Some(1) => println!("Position is visible"), 
    Some(2) => println!("Position has detection"),
    _ => println!("Vision check failed"),
}
```

#### `update_vision_with_units(include_workers: bool, include_structures: bool)`
Convenience method to automatically update vision with all your units.

```rust
// Update vision with all units including workers and structures
bot.update_vision_with_units(true, true);
```

#### `get_visible_positions(sample_distance: f32) -> Option<Vec<Point2>>`
Returns all positions that currently have vision coverage.

```rust
if let Some(visible_positions) = bot.get_visible_positions(5.0) {
    println!("Have vision of {} positions", visible_positions.len());
}
```

#### `get_fog_of_war_positions(center: Point2, radius: f32) -> Option<Vec<Point2>>`
Finds positions in fog of war (previously visible but not currently visible).

```rust
if let Some(fog_positions) = bot.get_fog_of_war_positions(base_location, 15.0) {
    println!("Found {} positions in fog of war", fog_positions.len());
}
```

## Complete Example

Here's a comprehensive example showing all the new features:

```rust
use rust_sc2::prelude::*;

#[bot]
#[derive(Default)]
struct AdvancedPathfindingBot;

impl Player for AdvancedPathfindingBot {
    fn on_start(&mut self) -> SC2Result<()> {
        // Initialize pathfinding system
        self.init_pathfinding();
        
        // Calculate strategic zones
        self.calculate_zones_from_start_locations();
        
        // Print choke information
        if let Some(chokes) = self.get_chokes() {
            println!("Map has {} choke points", chokes.len());
            
            for (i, choke) in chokes.iter().enumerate().take(3) {
                let center = choke.center();
                println!("Choke {}: center ({:.1}, {:.1}), width {:.1}",
                    i, center.0, center.1, choke.get_min_length());
            }
        }
        
        Ok(())
    }
    
    fn on_step(&mut self, iteration: usize) -> SC2Result<()> {
        // Update vision every 100 iterations
        if iteration % 100 == 0 {
            self.update_vision_with_units(true, true);
        }
        
        // Analyze strategic positions
        if iteration % 50 == 0 && !self.units.my.workers.is_empty() {
            let worker = self.units.my.workers.first().unwrap();
            let pos = worker.position();
            
            // Check what zone the worker is in
            if let Some(zone) = self.get_zone(pos) {
                println!("Worker is in zone {}", zone);
                
                // Find nearest choke
                if let Some((choke_idx, distance)) = self.get_closest_choke(pos) {
                    println!("Nearest choke is #{} at distance {:.1}", choke_idx, distance);
                }
            }
            
            // Check vision status
            match self.vision_status_at(pos) {
                Some(0) => println!("Worker position has no vision coverage"),
                Some(1) => println!("Worker position is visible"),
                Some(2) => println!("Worker position has detection"),
                _ => {},
            }
        }
        
        Ok(())
    }
}
```

## Performance Considerations

- Zone calculation should be done once at game start
- Vision updates can be expensive; update only when needed
- Use `get_chokes_near()` instead of iterating through all chokes
- Sample distances for `get_visible_positions()` affect performance vs accuracy

## Integration with Existing Features

These advanced features work seamlessly with the existing pathfinding system:

- Use zones for strategic decision making
- Use chokes for defensive positioning
- Use vision for intelligence gathering
- Combine with `get_path()` and `get_path_advanced()` for complete navigation solutions

The pathfinding integration is now complete and provides a comprehensive suite of tools for sophisticated bot navigation and map analysis!