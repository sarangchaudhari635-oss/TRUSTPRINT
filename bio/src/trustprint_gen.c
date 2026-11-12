#include <stdint.h>
#include "bio.h" // Must be first to place _start at 0x0

#define PROBE_PIN 21
#define PIN_MASK (1 << PROBE_PIN)

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

