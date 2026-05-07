#![no_std]
#![no_main]


use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    loop {}
}






// // Array size HAS to be defined by const or panic will not happen
pub const MY_ARRAY_SIZE: usize = 5;

struct TestStuct {
    my_array: [u32; MY_ARRAY_SIZE],
}

impl TestStuct {
    // function has to be async or panic will not exist
    pub async fn error(&mut self) {
        let _ = self.my_array[0];
    }
}