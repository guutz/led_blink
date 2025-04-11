
extern "C" {
    fn SVCall();
    fn PendSV();
    fn SysTick();
}

pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[doc(hidden)]
#[link_section = ".vector_table.exceptions2"]
#[no_mangle]
pub static __EXCEPTIONS2: [Vector; 14] = [
    // Exception 2: Non Maskable Interrupt.
    Vector { reserved: 0 },

    // Exception 3: Hard Fault Interrupt.
    Vector { reserved: 0 },

    // Exception 4: Memory Management Interrupt [not on Cortex-M0 variants].
    Vector { reserved: 0 },

    // Exception 5: Bus Fault Interrupt [not on Cortex-M0 variants].
    Vector { reserved: 0 },

    // Exception 6: Usage Fault Interrupt [not on Cortex-M0 variants].
    Vector { reserved: 0 },

    // Exception 7: APPLICATION VECTOR CHECKSUM
    Vector { reserved: usize::wrapping_neg(0x10002000 + 0xc1 ) },

    // 8-10: Reserved
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },

    // Exception 11: SV Call Interrupt.
    Vector { handler: SVCall },

    // Exception 12: Debug Monitor Interrupt [not on Cortex-M0 variants].
    Vector { reserved: 0 },

    // 13: Reserved
    Vector { reserved: 0 },

    // Exception 14: Pend SV Interrupt [not on Cortex-M0 variants].
    Vector { handler: PendSV },
    
    // Exception 15: System Tick Interrupt.
    Vector { handler: SysTick },
];
