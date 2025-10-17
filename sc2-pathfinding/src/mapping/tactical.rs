use crate::mapping::map_point;
use crate::path_find::pos::{NormalPosAPI, Pos, PositionAPI};
use crate::path_find::PathFind;
use std::cmp::Ordering;

/// Represents a tactical position suitable for siege units (e.g., Siege Tanks)
#[derive(Debug, Clone)]
pub struct TacticalPosition {
    /// Position on the map (x, y in game coordinates)
    pub position: (f32, f32),
    /// Score indicating how good this position is (higher is better)
    pub score: f32,
    /// Whether this position is on high ground
    pub on_high_ground: bool,
    /// Number of walkable tiles around this position (fewer is better for defense)
    pub walkable_neighbors: usize,
    /// Number of choke points within siege range
    pub chokes_in_range: usize,
    /// Height level of this position
    pub height: usize,
}

impl TacticalPosition {
    pub fn new(
        position: (f32, f32),
        on_high_ground: bool,
        walkable_neighbors: usize,
        chokes_in_range: usize,
        height: usize,
    ) -> Self {
        // Calculate score based on tactical advantages
        let mut score = 0.0;
        
        // High ground bonus
        if on_high_ground {
            score += 50.0;
        }
        
        // Fewer walkable neighbors means better defensive position
        // Max walkable neighbors in 3x3 area is 8
        score += (8.0 - walkable_neighbors.min(8) as f32) * 5.0;
        
        // More chokes in range is better
        score += chokes_in_range as f32 * 15.0;
        
        // Height advantage (higher is better for vision)
        score += height as f32 * 2.0;
        
        TacticalPosition {
            position,
            score,
            on_high_ground,
            walkable_neighbors,
            chokes_in_range,
            height,
        }
    }
}

/// Calculates tactical positions for siege units like Siege Tanks
/// 
/// # Arguments
/// * `points` - Map points grid with terrain information
/// * `ground_pathing` - Ground pathfinding data
/// * `chokes` - List of choke points on the map
/// * `x_start`, `y_start`, `x_end`, `y_end` - Map boundaries
/// * `unit_radius` - Radius of the unit (1.25 for Siege Tank in Siege Mode)
/// * `siege_range` - Attack range when sieged (13.0 for Siege Tank)
/// 
/// # Returns
/// Vector of tactical positions sorted by score (best first)
pub fn calculate_tactical_positions(
    points: &Vec<Vec<map_point::MapPoint>>,
    ground_pathing: &PathFind,
    chokes: &[((usize, usize), (usize, usize))],
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
    unit_radius: f32,
    siege_range: f32,
) -> Vec<TacticalPosition> {
    let mut tactical_positions = Vec::new();
    
    // Grid spacing - check every 2 tiles for performance
    let spacing = 2;
    
    for x in (x_start..x_end).step_by(spacing) {
        for y in (y_start..y_end).step_by(spacing) {
            // Skip if not walkable
            if ground_pathing.map[x][y] == 0 {
                continue;
            }
            
            let point = &points[x][y];
            
            // Check if position can fit a siege tank (needs some space)
            if !is_position_suitable(points, ground_pathing, x, y, unit_radius) {
                continue;
            }
            
            // Determine if on high ground
            let on_high_ground = is_on_high_ground(points, x, y, x_start, y_start, x_end, y_end);
            
            // Count walkable neighbors (fewer is better for defense)
            let walkable_neighbors = count_walkable_neighbors(ground_pathing, x, y);
            
            // Count chokes in siege range
            let chokes_in_range = count_chokes_in_range(
                chokes,
                x,
                y,
                siege_range,
            );
            
            // Get height
            let height = point.height;
            
            // Create tactical position
            let tactical_pos = TacticalPosition::new(
                (x as f32, y as f32),
                on_high_ground,
                walkable_neighbors,
                chokes_in_range,
                height,
            );
            
            // Only include positions with reasonable scores
            if tactical_pos.score > 30.0 {
                tactical_positions.push(tactical_pos);
            }
        }
    }
    
    // Sort by score (highest first)
    tactical_positions.sort_by(|a, b| {
        b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal)
    });
    
    tactical_positions
}

/// Check if a position is suitable for placing a siege unit
fn is_position_suitable(
    points: &Vec<Vec<map_point::MapPoint>>,
    ground_pathing: &PathFind,
    x: usize,
    y: usize,
    unit_radius: f32,
) -> bool {
    let radius_tiles = unit_radius.ceil() as usize;
    
    // Check if there's enough space around the position
    for dx in 0..=radius_tiles {
        for dy in 0..=radius_tiles {
            for &sign_x in &[-1, 1] {
                for &sign_y in &[-1, 1] {
                    let check_x = (x as i32 + dx as i32 * sign_x) as usize;
                    let check_y = (y as i32 + dy as i32 * sign_y) as usize;
                    
                    if check_x >= ground_pathing.map.len() || check_y >= ground_pathing.map[0].len() {
                        return false;
                    }
                    
                    // Must be walkable and on same height
                    if ground_pathing.map[check_x][check_y] == 0 
                        || points[check_x][check_y].height != points[x][y].height {
                        return false;
                    }
                }
            }
        }
    }
    
    true
}

/// Determine if a position is on high ground relative to surroundings
fn is_on_high_ground(
    points: &Vec<Vec<map_point::MapPoint>>,
    x: usize,
    y: usize,
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
) -> bool {
    let current_height = points[x][y].height;
    let check_radius = 5;
    
    let mut lower_count = 0;
    let mut total_count = 0;
    
    for dx in -(check_radius as i32)..=(check_radius as i32) {
        for dy in -(check_radius as i32)..=(check_radius as i32) {
            let check_x = (x as i32 + dx) as usize;
            let check_y = (y as i32 + dy) as usize;
            
            if check_x < x_start || check_x >= x_end || check_y < y_start || check_y >= y_end {
                continue;
            }
            
            total_count += 1;
            if points[check_x][check_y].height < current_height {
                lower_count += 1;
            }
        }
    }
    
    // Consider high ground if at least 40% of nearby tiles are lower
    total_count > 0 && (lower_count as f32 / total_count as f32) > 0.4
}

/// Count walkable neighbors around a position
fn count_walkable_neighbors(
    ground_pathing: &PathFind,
    x: usize,
    y: usize,
) -> usize {
    let mut count = 0;
    
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            
            let check_x = (x as i32 + dx) as usize;
            let check_y = (y as i32 + dy) as usize;
            
            if check_x < ground_pathing.map.len() && check_y < ground_pathing.map[0].len() {
                if ground_pathing.map[check_x][check_y] > 0 {
                    count += 1;
                }
            }
        }
    }
    
    count
}

/// Count number of choke points within siege range
fn count_chokes_in_range(
    chokes: &[((usize, usize), (usize, usize))],
    x: usize,
    y: usize,
    range: f32,
) -> usize {
    let mut count = 0;
    let pos = Pos(x, y);
    
    for choke in chokes {
        // Check distance to choke midpoint
        let mid_x = (choke.0.0 + choke.1.0) / 2;
        let mid_y = (choke.0.1 + choke.1.1) / 2;
        let choke_pos = Pos(mid_x, mid_y);
        
        let distance = NormalPosAPI().euclidean_distance(&pos, &choke_pos) as f32 / 8.0;
        
        if distance <= range {
            count += 1;
        }
    }
    
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tactical_position_scoring() {
        // High ground with few neighbors and chokes should score well
        let pos = TacticalPosition::new(
            (100.0, 100.0),
            true,  // on high ground
            2,     // few walkable neighbors
            3,     // 3 chokes in range
            10,    // height 10
        );
        
        assert!(pos.score > 100.0, "High ground position with chokes should score >100");
        assert_eq!(pos.on_high_ground, true);
        assert_eq!(pos.chokes_in_range, 3);
    }

    #[test]
    fn test_walkable_neighbors_scoring() {
        // Fewer neighbors should give higher score
        let pos1 = TacticalPosition::new((100.0, 100.0), false, 2, 0, 5);
        let pos2 = TacticalPosition::new((100.0, 100.0), false, 7, 0, 5);
        
        assert!(pos1.score > pos2.score, "Fewer walkable neighbors should score better");
    }
}
