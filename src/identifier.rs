static mut LAST_ID: u32 = 0;

pub struct Identifier;

impl Identifier {
    pub fn gen() -> u32 {
        unsafe {
            LAST_ID += 1;
            LAST_ID
        }
    }
}
