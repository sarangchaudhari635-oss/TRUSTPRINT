// ============================================================================
// TRUSTPRINT BIO Multi-Core Generator Firmware (Core 0)
//
// Target: Baochip-1x PicoRV32 BIO Coprocessor Core 0
// Role: Deterministic square-wave transition generator with quantum lockstep
// Output Pin: GPIO Pin 21
//
// HARDWARE LOOPBACK WIRING SPECIFICATION:
// Core 0 drives Output on GPIO Pin 21.
// Core 1 samples Input on GPIO Pin 20.
// On physical Dabao evaluation silicon, bridge Pin 21 to Pin 20 using
// a standard 0.1" jumper on the expansion header.
// ============================================================================

#include <stdint.h>
#include "bio.h" // Must be first to place _start at 0x0

#define GEN_OUTPUT_PIN 21
#define PIN_MASK (1 << GEN_OUTPUT_PIN)
#define ACK_TIMEOUT_CYCLES 50000

void main(void) {
    uint32_t epoch = 0;
    
    // Configure Pin 21 as Output
    set_gpio_mask(PIN_MASK);
    set_output_pins(PIN_MASK);
    
    // Initial drive LOW
    clear_gpio_pins_n(~PIN_MASK);

    while (1) {
        // --- High Phase ---
        // Snap to next quantum tick for cycle-accurate alignment
        wait_quantum();
        set_gpio_pins(PIN_MASK);
        epoch++;
        // Push transition epoch into FIFO 0 for Core 1 to sample
        push_fifo0(epoch);

        // --- Low Phase ---
        wait_quantum();
        clear_gpio_pins_n(~PIN_MASK);
        epoch++;
        // Push transition epoch into FIFO 0 for Core 1 to sample
        push_fifo0(epoch);

        // Wait for Core 1 acknowledgment over FIFO 1 (blocking read)
        uint32_t ack = pop_fifo1();
        (void)ack;
    }
}

