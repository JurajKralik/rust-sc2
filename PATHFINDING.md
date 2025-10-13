# Advanced Pathfinding with sc2-pathfinding

This document explains how to use the advanced pathfinding features added to rust-sc2 through integration with the sc2-pathfinding library.

## Overview

The rust-sc2 library now includes advanced pathfinding capabilities that go beyond the basic SC2 API pathfinding queries. This integration provides:

- **Multi-unit type pathfinding**: Different pathfinding maps for ground units, reapers, colossi, and air units
- **Dynamic obstacle updates**: Real-time updates when buildings are constructed/destroyed
- **Influence mapping**: Avoid dangerous areas or preferred routes
- **Advanced pathfinding options**: Custom heuristics, search windows, and distance limits
- **Choke detection**: Identify bottlenecks and strategic positions
- **Vision integration**: Pathfinding that considers unit sight and detection

## Getting Started

### 1. Initialize Pathfinding

Call `init_pathfinding()` once at the start of your bot, typically in the `on_start()` method:

```rust
impl Player for MyBot {
    fn on_start(&mut self) -> SC2Result<()> {
        // Initialize pathfinding from game map data
        self.init_pathfinding();
        
        if self.is_pathfinding_initialized() {
            println!("Pathfinding ready!");
        }
        
        Ok(())
    }
}
```

### 2. Basic Path Finding

Use `get_path()` to find a path between two points:

```rust
let start = self.start_location;
let end = self.enemy_start;

match self.get_path(start, end, PathfindingUnitType::Ground, false, false) {
    Some((path, distance)) => {
        println!("Found path: {} waypoints, {:.2} distance", path.len(), distance);
        
        // Path is a Vec<Point2> of waypoints from start to end
        for (i, waypoint) in path.iter().enumerate() {
            println!("Waypoint {}: ({:.1}, {:.1})", i, waypoint.x, waypoint.y);
        }
    }
    None => {
        println!("No path found!");
    }
}
```

## Unit Types

The pathfinding system supports different unit movement types:

### PathfindingUnitType::Ground
- Standard ground units (marines, zealots, etc.)
- Cannot traverse cliffs or fly over obstacles
- Most restrictive pathing

### PathfindingUnitType::Air  
- Flying units (vikings, mutalisks, etc.)
- Can fly over all terrain and obstacles
- Least restrictive pathing

### PathfindingUnitType::Reaper
- Reaper-like units that can climb cliffs
- Can traverse some terrain that normal ground units cannot
- More flexible than ground, less than air

### PathfindingUnitType::Colossus
- Large units that can step over small obstacles
- Can walk over some units and small terrain features
- Between ground and air in flexibility

## Advanced Pathfinding

For more control, use `get_path_advanced()`:

```rust
let path_result = self.get_path_advanced(
    start_pos,
    end_pos,
    PathfindingUnitType::Ground,
    false,                    // large_unit: false for normal sized units
    true,                     // use_influence: avoid dangerous areas
    Some(1),                  // heuristic: 0=Manhattan, 1=Octile, 2=Euclidean
    Some((min_corner, max_corner)), // search_window: restrict search area
    Some(50.0),               // max_distance: stop search early if closer than this
);
```

### Parameters Explained

- **large_unit**: `true` for large units that need more space to navigate
- **use_influence**: `true` to consider influence maps (avoid dangerous areas)
- **heuristic**: Distance calculation method
  - `0`: Manhattan distance (fastest, less accurate)
  - `1`: Octile distance (good balance)
  - `2`: Euclidean distance (most accurate, slower)
- **search_window**: Optional bounding box to restrict pathfinding search
- **max_distance**: Stop pathfinding early when within this distance of target

## Dynamic Updates

### Building Updates

Update pathfinding when buildings change:

```rust
// Call this when buildings are constructed or destroyed
self.update_pathfinding_buildings();
```

This automatically:
1. Resets pathfinding maps to original state
2. Adds current buildings as obstacles
3. Updates all unit type maps accordingly

### Manual Obstacle Management

For more control, access the pathfinding map directly:

```rust
if let Some(pathfinding_map) = self.pathfinding_map_mut() {
    // Add obstacles
    let building_positions = vec![(32.5, 24.5), (35.5, 24.5)];
    pathfinding_map.create_blocks(building_positions, (3, 3)); // 3x3 size
    
    // Remove obstacles  
    pathfinding_map.remove_blocks(building_positions, (3, 3));
    
    // Reset to original map
    pathfinding_map.reset();
}
```

## Influence Mapping

Add influence to make certain areas less desirable:

```rust
if let Some(pathfinding_map) = self.pathfinding_map_mut() {
    // Make areas around enemy units dangerous
    for enemy in self.units.enemy.all.iter() {
        let ground_map = pathfinding_map.get_map_mut(0);
        ground_map.add_influence_spot(
            (enemy.position().x as usize, enemy.position().y as usize),
            1000, // High influence value to avoid
        );
    }
}
```

Then use `use_influence: true` in pathfinding calls to consider these areas.

## Practical Examples

### Worker to Mineral Pathfinding

```rust
for worker in self.units.my.workers.iter() {
    if let Some(mineral) = self.units.mineral_fields.closest(worker.position()) {
        match self.get_path(
            worker.position(),
            mineral.position(), 
            PathfindingUnitType::Ground,
            false, // not a large unit
            false, // no influence needed for workers
        ) {
            Some((path, distance)) => {
                // Move worker along the path
                if let Some(next_waypoint) = path.get(1) {
                    worker.move_to(Target::Pos(*next_waypoint), false);
                }
            }
            None => {
                // Direct path if no pathfinding path found
                worker.move_to(Target::Pos(mineral.position()), false);
            }
        }
    }
}
```

### Army Movement with Influence

```rust
// Add influence around enemy positions to avoid them when moving
if let Some(pathfinding_map) = self.pathfinding_map_mut() {
    pathfinding_map.reset(); // Clear previous influence
    
    for enemy in self.units.enemy.all.iter() {
        let ground_map = pathfinding_map.get_map_mut(0);
        ground_map.add_influence_spot(
            (enemy.position().x as usize, enemy.position().y as usize),
            500, // Moderate influence to discourage but not prevent passage
        );
    }
}

// Find safe path for army
let army_position = self.units.my.units.center();
match self.get_path(
    army_position,
    self.enemy_start,
    PathfindingUnitType::Ground,
    true,  // army is effectively large
    true,  // use influence to avoid enemies
) {
    Some((path, _)) => {
        for unit in self.units.my.units.iter() {
            if let Some(waypoint) = path.get(1) {
                unit.move_to(Target::Pos(*waypoint), false);
            }
        }
    }
    None => {
        // Fallback to direct movement
        for unit in self.units.my.units.iter() {
            unit.attack(Target::Pos(self.enemy_start), false);
        }
    }
}
```

### Air vs Ground Comparison

```rust
let start = self.start_location;
let end = self.enemy_start;

// Compare different pathfinding options
let ground_path = self.get_path(start, end, PathfindingUnitType::Ground, false, false);
let air_path = self.get_path(start, end, PathfindingUnitType::Air, false, false);

match (ground_path, air_path) {
    (Some((ground, ground_dist)), Some((air, air_dist))) => {
        println!("Ground path: {:.2} distance", ground_dist);
        println!("Air path: {:.2} distance", air_dist);
        println!("Air advantage: {:.1}%", (ground_dist / air_dist - 1.0) * 100.0);
    }
    _ => println!("Pathfinding failed"),
}
```

## Performance Considerations

1. **Initialize once**: Call `init_pathfinding()` only once at game start
2. **Update selectively**: Only call `update_pathfinding_buildings()` when buildings actually change
3. **Use appropriate unit types**: Don't use `Air` pathfinding for ground units
4. **Limit search areas**: Use search windows for local pathfinding
5. **Choose good heuristics**: Octile distance (1) is usually the best balance

## Troubleshooting

### No Path Found
- Check if start/end positions are on walkable terrain
- Verify the correct unit type is being used
- Consider if obstacles are blocking all paths
- Try with `Air` unit type to see if terrain is the issue

### Performance Issues  
- Reduce frequency of `update_pathfinding_buildings()` calls
- Use search windows to limit pathfinding area
- Use Manhattan distance heuristic for faster but less accurate paths

### Unexpected Paths
- Check if influence mapping is affecting the path
- Verify building obstacles are correctly updated
- Consider if the unit type matches your expectations

## Integration with SC2 API

The advanced pathfinding complements, but doesn't replace, the built-in SC2 pathfinding:

- Use **SC2 API pathfinding** (`query_pathing()`) for simple distance queries
- Use **Advanced pathfinding** for complex routing, influence avoidance, and strategic path planning

Both can be used together in the same bot for different purposes.

## Example Bot

See `examples/pathfinding_bot.rs` for a complete example demonstrating all pathfinding features.