#![no_std]
#![no_main]

mod vga_buffer;
mod print;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    vga_buffer::WRITER.lock().set_color(vga_buffer::Color::Red, vga_buffer::Color::Black);
    println!("{}", info);

    loop {  }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("          Hello World");
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();

    loop { }
}
