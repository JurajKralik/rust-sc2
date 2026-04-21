use sc2pathfinding::mapping::map::Map;
use sc2pathfinding::path_find;
use std::fs::File;
use std::io::{BufRead, BufReader};
use image::{RgbImage, Rgb};
use std::fs;
use std::io::Read;
use numpy::{PyArray2, PyArrayMethods};
use pyo3::prelude::*;

fn rot90(vec: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let new_height = vec.len();
    let new_width = vec[0].len();
    let mut new_arr: Vec<Vec<usize>> = vec![vec![0; new_height]; new_width];
    // Traverse each cycle
    for i in 0..new_width {
        for j in 0..new_height {
            new_arr[i][j] = vec[new_height - 1 - j][i];
        }
    }
    new_arr
}

pub fn read_vec_from_file(file_path: &str) -> Vec<Vec<usize>> {
    let f = BufReader::new(File::open(file_path).unwrap());
    let mut arr = Vec::<Vec<usize>>::new();

    for line in f.lines().map(|x| x.unwrap()) {
        let mut maze_line = vec![];
        for char in line.chars() {
            if !char.is_digit(10) {
                break;
            }
            let value = char.to_digit(10).unwrap() as usize;
            maze_line.push(value)
        }

        arr.push(maze_line);
    }
    rot90(arr)
}

pub fn get_pathfind(file: &str) -> path_find::PathFind {
    let map = read_vec_from_file(file);
    path_find::PathFind::new_internal(map)
}

pub fn get_choke_map() -> Map {
    let grid = read_vec_from_file("tests/choke.txt");
    let grid2 = read_vec_from_file("tests/choke.txt");
    let grid_height = read_vec_from_file("tests/choke_height.txt");

    let map = Map::new(grid, grid2, grid_height, 2, 2, 38, 38);
    map
}

/// Load a numpy .npy file and convert to Vec<Vec<usize>>
fn load_npy_as_grid(file_path: &str) -> Result<Vec<Vec<usize>>, Box<dyn std::error::Error>> {
    pyo3::prepare_freethreaded_python();
    
    let result = Python::with_gil(|py| -> PyResult<Vec<Vec<usize>>> {
        let numpy = py.import_bound("numpy")?;
        let array = numpy.call_method1("load", (file_path,))?;
        
        // Try uint8 first (most common for SC2 maps)
        if let Ok(array) = array.downcast::<PyArray2<u8>>() {
            let array_view = unsafe { array.as_array() };
            let shape = array_view.shape();
            let rows = shape[0];
            let cols = shape[1];
            
            let mut grid = Vec::new();
            for row_idx in 0..rows {
                let mut grid_row = Vec::new();
                for col_idx in 0..cols {
                    let val = array_view[[row_idx, col_idx]];
                    // uint8 values should be 0 or 1 already
                    grid_row.push(val as usize);
                }
                grid.push(grid_row);
            }
            Ok(grid)
        } else if let Ok(array) = array.downcast::<PyArray2<f32>>() {
            let array_view = unsafe { array.as_array() };
            let shape = array_view.shape();
            let rows = shape[0];
            let cols = shape[1];
            
            let mut grid = Vec::new();
            for row_idx in 0..rows {
                let mut grid_row = Vec::new();
                for col_idx in 0..cols {
                    let val = array_view[[row_idx, col_idx]];
                    // Convert float values to usize (0 or 1 for pathable/non-pathable)
                    grid_row.push(if val > 0.5 { 1 } else { 0 });
                }
                grid.push(grid_row);
            }
            Ok(grid)
        } else {
            Err(pyo3::PyErr::new::<pyo3::exceptions::PyTypeError, _>("Unsupported array type"))
        }
    })?;
    
    Ok(result)
}

/// Create AutomatonLE map using actual .npy data files
pub fn get_automaton_le_map_npy() -> Result<Map, Box<dyn std::error::Error>> {
    // Load the actual map data from .npy files
    let pathing_grid = load_npy_as_grid("tests/AutomatonLE_pathing.npy")?;
    let placement_grid = load_npy_as_grid("tests/AutomatonLE_placement.npy")?;
    let height_grid = load_npy_as_grid("tests/AutomatonLE_height.npy")?;
    
    // Get grid dimensions
    let height = pathing_grid.len();
    let width = if height > 0 { pathing_grid[0].len() } else { 0 };
    
    println!("AutomatonLE .npy grid dimensions: {}x{}", height, width);
    
    // AutomatonLE map dimensions with safer boundaries
    let x_start = 4;
    let y_start = 4;
    let x_end = if height > 8 { height - 4 } else { height.saturating_sub(1) };
    let y_end = if width > 8 { width - 4 } else { width.saturating_sub(1) };
    
    println!("Using map boundaries: x_start={}, y_start={}, x_end={}, y_end={}", x_start, y_start, x_end, y_end);
    
    let map = Map::new(pathing_grid, placement_grid, height_grid, x_start, y_start, x_end, y_end);
    Ok(map)
}

/// Create Submarine LE map using actual .npy data files
pub fn get_submarine_le_map_npy() -> Result<Map, Box<dyn std::error::Error>> {
    // Load the actual map data from .npy files
    let pathing_grid = load_npy_as_grid("tests/Submarine LE_pathing.npy")?;
    let placement_grid = load_npy_as_grid("tests/Submarine LE_placement.npy")?;
    let height_grid = load_npy_as_grid("tests/Submarine LE_height.npy")?;
    
    // Get grid dimensions
    let height = pathing_grid.len();
    let width = if height > 0 { pathing_grid[0].len() } else { 0 };
    
    println!("Submarine LE .npy grid dimensions: {}x{}", height, width);
    
    // Submarine LE map dimensions with safer boundaries
    let x_start = 4;
    let y_start = 4;
    let x_end = if height > 8 { height - 4 } else { height.saturating_sub(1) };
    let y_end = if width > 8 { width - 4 } else { width.saturating_sub(1) };
    
    println!("Using map boundaries: x_start={}, y_start={}, x_end={}, y_end={}", x_start, y_start, x_end, y_end);
    
    let map = Map::new(pathing_grid, placement_grid, height_grid, x_start, y_start, x_end, y_end);
    Ok(map)
}

pub fn get_automaton_le_map() -> Map {
    // For simplicity, we'll use the text file format for all three grids
    // In a real implementation, you might want to load the .npy files
    let pathing_grid = read_vec_from_file("tests/AutomatonLE.txt");
    let placement_grid = read_vec_from_file("tests/AutomatonLE.txt"); // Use same as pathing for now
    let height_grid = read_vec_from_file("tests/AutomatonLE.txt");    // Use same as pathing for now
    
    // Get grid dimensions
    let height = pathing_grid.len();
    let width = if height > 0 { pathing_grid[0].len() } else { 0 };
    
    println!("AutomatonLE grid dimensions: {}x{}", height, width);
    
    // AutomatonLE map dimensions with safer boundaries
    // Use small margins to avoid overflow issues and ensure x_end < width, y_end < height
    let x_start = 4;
    let y_start = 4;
    let x_end = if height > 8 { height - 4 } else { height.saturating_sub(1) };  // Note: swapped for grid[y][x] indexing
    let y_end = if width > 8 { width - 4 } else { width.saturating_sub(1) };     // Note: swapped for grid[y][x] indexing
    
    println!("Using map boundaries: x_start={}, y_start={}, x_end={}, y_end={}", x_start, y_start, x_end, y_end);
    
    let map = Map::new(pathing_grid, placement_grid, height_grid, x_start, y_start, x_end, y_end);
    map
}

/// Save pathfinding result as an image
pub fn save_pathfinding_image(
    grid: &Vec<Vec<usize>>,
    path: &Vec<(usize, usize)>,
    filename: &str,
) {
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };
    
    // Create image with scaling for better visibility
    let scale = 4;
    let mut img = RgbImage::new((width * scale) as u32, (height * scale) as u32);
    
    // Define colors
    let black = Rgb([0u8, 0u8, 0u8]);        // Blocked areas
    let white = Rgb([255u8, 255u8, 255u8]);  // Walkable areas
    let red = Rgb([255u8, 0u8, 0u8]);        // Path
    let green = Rgb([0u8, 255u8, 0u8]);      // Start point
    let blue = Rgb([0u8, 0u8, 255u8]);       // End point
    
    // Draw the base map
    for y in 0..height {
        for x in 0..width {
            let color = if grid[y][x] == 0 { black } else { white };
            
            // Fill scaled pixel area
            for dy in 0..scale {
                for dx in 0..scale {
                    img.put_pixel(
                        (x * scale + dx) as u32,
                        (y * scale + dy) as u32,
                        color
                    );
                }
            }
        }
    }
    
    // Draw the path with correct coordinate mapping
    for (i, &(px, py)) in path.iter().enumerate() {
        let color = if i == 0 {
            green  // Start
        } else if i == path.len() - 1 {
            blue   // End
        } else {
            red    // Path
        };
        
        // Use swapped coordinates: path coordinates (px,py) map to image coordinates (py,px)
        // This corrects the coordinate system mismatch from the grid rotation
        for dy in 0..scale {
            for dx in 0..scale {
                if py < width && px < height {
                    img.put_pixel(
                        (py * scale + dx) as u32,
                        (px * scale + dy) as u32,
                        color
                    );
                }
            }
        }
    }
    
    img.save(filename).expect("Failed to save image");
    println!("Saved pathfinding visualization to: {}", filename);
}

/// Debug version to understand coordinate system issues
pub fn save_pathfinding_image_debug(
    grid: &Vec<Vec<usize>>,
    path: &Vec<(usize, usize)>,
    filename: &str,
) {
    println!("Debug: Grid dimensions: {}x{}", grid.len(), if grid.len() > 0 { grid[0].len() } else { 0 });
    println!("Debug: Path length: {}", path.len());
    if !path.is_empty() {
        println!("Debug: Path start: ({}, {})", path[0].0, path[0].1);
        println!("Debug: Path end: ({}, {})", path[path.len()-1].0, path[path.len()-1].1);
    }
    
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };
    
    // Print first few rows of grid for debugging
    println!("Debug: First 5x5 of grid:");
    for y in 0..std::cmp::min(5, height) {
        for x in 0..std::cmp::min(5, width) {
            print!("{} ", grid[y][x]);
        }
        println!();
    }
    
    // Create image with larger scaling for debugging
    let scale = 8;
    let mut img = RgbImage::new((width * scale) as u32, (height * scale) as u32);
    
    // Define colors
    let black = Rgb([0u8, 0u8, 0u8]);        // Blocked areas
    let white = Rgb([255u8, 255u8, 255u8]);  // Walkable areas
    let red = Rgb([255u8, 0u8, 0u8]);        // Path
    let green = Rgb([0u8, 255u8, 0u8]);      // Start point
    let blue = Rgb([0u8, 0u8, 255u8]);       // End point
    
    // Draw the base map - try both coordinate interpretations
    for y in 0..height {
        for x in 0..width {
            let color = if grid[y][x] == 0 { black } else { white };
            
            // Fill scaled pixel area
            for dy in 0..scale {
                for dx in 0..scale {
                    img.put_pixel(
                        (x * scale + dx) as u32,
                        (y * scale + dy) as u32,
                        color
                    );
                }
            }
        }
    }
    
    // Draw the path with coordinate debugging
    for (i, &(px, py)) in path.iter().enumerate() {
        println!("Debug: Path point {}: ({}, {})", i, px, py);
        
        let color = if i == 0 {
            green  // Start
        } else if i == path.len() - 1 {
            blue   // End
        } else {
            red    // Path
        };
        
        // Try original coordinate interpretation
        if px < width && py < height {
            for dy in 0..scale {
                for dx in 0..scale {
                    img.put_pixel(
                        (px * scale + dx) as u32,
                        (py * scale + dy) as u32,
                        color
                    );
                }
            }
        }
    }
    
    img.save(filename).expect("Failed to save image");
    println!("Saved debug pathfinding visualization to: {}", filename);
}

/// Alternative coordinate system visualization
pub fn save_pathfinding_image_alt(
    grid: &Vec<Vec<usize>>,
    path: &Vec<(usize, usize)>,
    filename: &str,
) {
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };
    
    // Create image with scaling for better visibility
    let scale = 4;
    let mut img = RgbImage::new((width * scale) as u32, (height * scale) as u32);
    
    // Define colors
    let black = Rgb([0u8, 0u8, 0u8]);        // Blocked areas
    let white = Rgb([255u8, 255u8, 255u8]);  // Walkable areas
    let red = Rgb([255u8, 0u8, 0u8]);        // Path
    let green = Rgb([0u8, 255u8, 0u8]);      // Start point
    let blue = Rgb([0u8, 0u8, 255u8]);       // End point
    
    // Draw the base map
    for y in 0..height {
        for x in 0..width {
            let color = if grid[y][x] == 0 { black } else { white };
            
            // Fill scaled pixel area
            for dy in 0..scale {
                for dx in 0..scale {
                    img.put_pixel(
                        (x * scale + dx) as u32,
                        (y * scale + dy) as u32,
                        color
                    );
                }
            }
        }
    }
    
    // Draw the path with SWAPPED coordinates
    for (i, &(px, py)) in path.iter().enumerate() {
        let color = if i == 0 {
            green  // Start
        } else if i == path.len() - 1 {
            blue   // End
        } else {
            red    // Path
        };
        
        // Try swapped coordinate interpretation: (py, px) instead of (px, py)
        if py < width && px < height {
            for dy in 0..scale {
                for dx in 0..scale {
                    img.put_pixel(
                        (py * scale + dx) as u32,
                        (px * scale + dy) as u32,
                        color
                    );
                }
            }
        }
    }
    
    img.save(filename).expect("Failed to save image");
    println!("Saved alternative coordinate pathfinding visualization to: {}", filename);
}

/// Save choke detection result as an image
pub fn save_choke_image(
    map: &Map,
    filename: &str,
) {
    let grid = map.ground_pathing_map();
    let chokes = map.chokes();
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };
    
    // Create image with scaling for better visibility
    let scale = 4;
    let mut img = RgbImage::new((width * scale) as u32, (height * scale) as u32);
    
    // Define colors
    let black = Rgb([0u8, 0u8, 0u8]);        // Blocked areas
    let white = Rgb([255u8, 255u8, 255u8]);  // Walkable areas
    let yellow = Rgb([255u8, 255u8, 0u8]);   // Choke points
    let red = Rgb([255u8, 0u8, 0u8]);        // Choke borders
    
    // Draw the base map
    for y in 0..height {
        for x in 0..width {
            let color = if grid[y][x] == 0 { black } else { white };
            
            // Fill scaled pixel area
            for dy in 0..scale {
                for dx in 0..scale {
                    img.put_pixel(
                        (x * scale + dx) as u32,
                        (y * scale + dy) as u32,
                        color
                    );
                }
            }
        }
    }
    
    // Draw chokes
    for choke in chokes {
        // Draw choke center with corrected coordinate mapping
        let center = choke.center();
        let center_x = center.0 as usize;
        let center_y = center.1 as usize;
        
        // Use swapped coordinates: choke coordinates (center_x,center_y) map to image coordinates (center_y,center_x)
        for dy in 0..scale {
            for dx in 0..scale {
                if center_y < width && center_x < height {
                    img.put_pixel(
                        (center_y * scale + dx) as u32,
                        (center_x * scale + dy) as u32,
                        yellow
                    );
                }
            }
        }
        
        // Draw choke pixels with corrected coordinate mapping
        for &(px, py) in &choke.pixels {
            for dy in 0..scale {
                for dx in 0..scale {
                    // Use swapped coordinates: choke coordinates (px,py) map to image coordinates (py,px)
                    if py < width && px < height {
                        img.put_pixel(
                            (py * scale + dx) as u32,
                            (px * scale + dy) as u32,
                            red
                        );
                    }
                }
            }
        }
    }
    
    img.save(filename).expect("Failed to save image");
    println!("Saved choke detection visualization to: {}", filename);
}
