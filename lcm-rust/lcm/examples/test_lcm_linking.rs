extern crate lcm;

use lcm::Lcm;

fn main() {
    // Try to create an LCM instance (no arguments needed)
    match Lcm::new() {
        Ok(_lcm) => {
            println!("✅ LCM library is working correctly!");
        }
        Err(e) => {
            println!("❌ Error creating LCM instance: {:?}", e);
        }
    }
}
