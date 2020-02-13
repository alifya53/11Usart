// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// #[allow(unused_imports)]
// use aux11::{entry, iprint, iprintln};

// #[entry]
// fn main() -> ! {
//     let (usart1, mono_timer, itm) = aux11::init();

//     // Send a string
//     for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
//         usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
//     }

//     loop {}
// }
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();
    // rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());
    // gpioe.moder.modify(|_, w| w.moder8().output());

    loop {
        // Wait until there's data available
        while usart1.isr.read().rxne().bit_is_clear() {}

        // Retrieve the data
        let _byte = usart1.rdr.read().rdr().bits() as u8;
        const GPIOE_BSRR: u32 = 0x48001018;
        unsafe{
        if _byte == 65 {
            *(GPIOE_BSRR as *mut u32) = 1 << 8;
        }
        else if _byte == 66 {
            *(GPIOE_BSRR as *mut u32) = 1 << 9;
        }
        else if _byte == 67 {
            *(GPIOE_BSRR as *mut u32) = 1 << 10;
        }
        else if _byte == 68 {
            *(GPIOE_BSRR as *mut u32) = 1 << 11;
        }
        else if _byte == 69 {
            *(GPIOE_BSRR as *mut u32) = 1 << 12;
        }else if _byte == 70 {
            *(GPIOE_BSRR as *mut u32) = 1 << 13;
        }
        else if _byte == 71 {
            *(GPIOE_BSRR as *mut u32) = 1 << 14;
        }
        else if _byte == 72 {
            *(GPIOE_BSRR as *mut u32) = 1 << 15;
        }
        else if _byte == 64 {
            for i in 8..16{
                *(GPIOE_BSRR as *mut u32) = 1 << i;
            };
        }else if _byte == 35 {
            for i in 8..16{
                *(GPIOE_BSRR as *mut u32) = 1 << (i + 16);
            };
        }else if _byte == 97 {
            *(GPIOE_BSRR as *mut u32) = 1 << (8+16);
        }
        else if _byte == 98 {
            *(GPIOE_BSRR as *mut u32) = 1 << (9+16);
        }
        else if _byte == 99 {
            *(GPIOE_BSRR as *mut u32) = 1 << (10+16);
        }
        else if _byte == 100 {
            *(GPIOE_BSRR as *mut u32) = 1 << (11+16);
        }
        else if _byte == 101 {
            *(GPIOE_BSRR as *mut u32) = 1 << (12+16);
        }else if _byte == 102 {
            *(GPIOE_BSRR as *mut u32) = 1 << (13+16);
        }
        else if _byte == 103 {
            *(GPIOE_BSRR as *mut u32) = 1 << (14+16);
        }
        else if _byte == 104 {
            *(GPIOE_BSRR as *mut u32) = 1 << (15+16);
        }   

    };
        iprintln!(&mut itm.stim[0], "`for` loop took {}", _byte);

        // aux11::bkpt();
    }
}