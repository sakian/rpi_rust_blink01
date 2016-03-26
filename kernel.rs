// kernel.rs
// compile using: rustc --target arm-unknown-linux-gnueabihf -O --emit=obj kernel.rs
// link using: arm-none-eabi-gcc -O0 -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostartfiles --specs=nosys.specs kernel.o -o kernel.elf
// isolate binary using: arm-none-eabi-objcopy kernel.elf -O binary kernel.img

#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]

const GPIO_BASE: u32 = 0x20200000;

fn sleep(value: u32) {  
    for _ in 1..value {
        unsafe { asm!(""); }
    }
}

#[no_mangle]
pub extern fn main() {  
    let gpio = GPIO_BASE as *const u32;
    let led_on = unsafe { gpio.offset(8) as *mut u32 };
    let led_off = unsafe { gpio.offset(11) as *mut u32 };

    loop {
        unsafe { *(led_on) = 1 << 15; }
        sleep(2000000);
        unsafe { *(led_off) = 1 << 15; }
        sleep(2000000);
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() {}

