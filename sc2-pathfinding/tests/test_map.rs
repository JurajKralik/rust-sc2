use common::{get_choke_map, get_automaton_le_map, read_vec_from_file, save_pathfinding_image, save_choke_image};
use sc2pathfinding::mapping::{map::Map, vision::VisionUnit};
mod common;

#[test]
fn test_find_path_map() {
    let grid = read_vec_from_file("tests/maze4x4.txt");
    let grid2 = read_vec_from_file("tests/maze4x4.txt");
    let grid3 = read_vec_from_file("tests/maze4x4.txt");
    let map = Map::new(grid, grid2, grid3, 1, 1, 3, 3);
    let r = map.find_path_basic(0, (0f32, 0f32), (3f32, 3f32), Some(0));
    let (path, distance) = r;
    assert_eq!(distance, 6.0);
    
    // Save pathfinding visualization using the map's internal grid
    save_pathfinding_image(map.ground_pathing_map(), &path, "test_output/map_maze4x4_path.png");
}

#[test]
fn test_find_map_borders() {
    let map = get_choke_map();
    let r = map.get_borders();
    assert_eq!(r.len(), 102);
}

#[test]
fn test_find_map_chokes() {
    let map = get_choke_map();
    let r = map.chokes();
    assert_eq!(r.len(), 1);
    
    // Save choke detection visualization
    save_choke_image(&map, "test_output/choke_detection.png");
}

#[test]
fn test_ray_vision() {
    let mut map = get_choke_map();
    let vision_unit = VisionUnit::new(false, false, (18f32, 8f32), 10f32);
    map.add_vision_unit(vision_unit);
    map.calculate_vision_map();

    assert_eq!(map.vision_status((12f32, 8f32)), 1);
    assert_eq!(map.vision_status((19f32, 8f32)), 1);
    assert_eq!(map.vision_status((25f32, 8f32)), 0);
    assert_eq!(map.vision_status((27f32, 8f32)), 0);
}

#[test]
fn test_flying_vision() {
    let mut map = get_choke_map();
    let vision_unit = VisionUnit::new(false, true, (19f32, 8f32), 10f32);
    map.add_vision_unit(vision_unit);
    map.calculate_vision_map();

    assert_eq!(map.vision_status((21f32, 8f32)), 1);
    assert_eq!(map.vision_status((27f32, 8f32)), 1);
    assert_eq!(map.vision_status((31f32, 8f32)), 0);
}

#[test]
fn test_automaton_le_chokes() {
    println!("=== TESTING AUTOMATON LE CHOKE DETECTION ===");
    let map = get_automaton_le_map();
    let chokes = map.chokes();
    
    println!("Found {} chokes on AutomatonLE", chokes.len());
    
    // Print choke information for debugging
    for (i, choke) in chokes.iter().enumerate() {
        let center = choke.center();
        println!("Choke {}: Center at ({:.2}, {:.2}), {} pixels, min_length: {:.2}", 
                i, center.0, center.1, choke.pixels.len(), choke.get_min_length());
    }
    
    // Save choke detection visualization for AutomatonLE
    save_choke_image(&map, "test_output/automaton_le_chokes.png");
    
    // Basic assertion - AutomatonLE should have some chokes
    assert!(chokes.len() > 0, "AutomatonLE should have at least some chokes detected");
}
