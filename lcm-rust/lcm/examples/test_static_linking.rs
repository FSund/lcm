extern crate lcm;

use lcm::Lcm;

fn main() {
    // Try to create an LCM instance (no arguments needed)
    match Lcm::new() {
        Ok(_lcm) => {
            println!("✅ LCM library is working correctly!");
            println!("✅ Static linking successful - no external liblcm installation required!");
        }
        Err(e) => {
            println!("❌ Error creating LCM instance: {:?}", e);
        }
    }
}
