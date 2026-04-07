#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::write_volatile;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Adresses spécifiques au RP2040 (Raspberry Pi Pico)
const GPIO_OUT_SET: *mut u32 = 0xd0000014 as *mut u32;
const GPIO_OUT_CLR: *mut u32 = 0xd0000018 as *mut u32;
const LED_PIN: u32 = 1 << 25; // La LED est sur le GPIO 25

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Note : Normalement, il faut configurer les registres de fonction (IO_BANK)
    // Mais sur Wokwi, certains modèles acceptent l'écriture directe pour simplifier.
    
    loop {
        unsafe {
            // Allumer la LED
            write_volatile(GPIO_OUT_SET, LED_PIN);
            delay(100_000);
            
            // Éteindre la LED
            write_volatile(GPIO_OUT_CLR, LED_PIN);
            delay(100_000);
        }
    }
}

fn delay(n: u32) {
    for _ in 0..n {
        core::hint::black_box(()); // Empêche le compilateur d'ignorer la boucle
    }
}