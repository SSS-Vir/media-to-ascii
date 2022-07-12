use windows::Win32::{
    Foundation::{BOOL, HANDLE},
    System::Console::{GetStdHandle, WriteConsoleOutputCharacterA, COORD, STD_OUTPUT_HANDLE},
};

#[derive(Clone, Copy)]
pub struct OUT {
    STD_HANDLE: HANDLE,
}

impl OUT {
    pub unsafe fn new() -> Self {
        OUT {
            STD_HANDLE: GetStdHandle(STD_OUTPUT_HANDLE).unwrap(),
        }
    }

    pub unsafe fn print(self, message: &[u8], coord: COORD) -> BOOL {
        static mut dw_bytes_written: u32 = 0;
        WriteConsoleOutputCharacterA(self.STD_HANDLE, message, coord, &mut dw_bytes_written)
    }
}
