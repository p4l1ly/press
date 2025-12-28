// Template binary for debugging your press object
// Replace `press_template` with your actual crate name (e.g., `press_sphere`)

use press_hut::MyObject;

fn main() {
    let obj = MyObject::default();
    println!("brick_count {}", obj.brick_rows.iter().map(|row| row.count * 2 + row.odd as usize).sum::<usize>());
    
    // Example usage:
    // let cfg = Config::default();
    // println!("Config: {:?}", cfg);
    
    // Add your debug code here
}

