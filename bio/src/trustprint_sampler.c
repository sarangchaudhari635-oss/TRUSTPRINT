#include <stdint.h>
#include "bio.h" // Must be first to place _start at 0x0

#define PROBE_PIN 21
#define PIN_MASK (1 << PROBE_PIN)

void main(void) {
    // Configure Pin 21 as Input for Core 1 sampling
    set_gpio_mask(PIN_MASK);
    set_input_pins(PIN_MASK);

    while (1) {
        // Block until Core 0 posts transition epoch to FIFO 0
        uint32_t epoch = pop_fifo0();
        
        // Sample physical pin state
        uint32_t pins = read_gpio_pins();
        uint32_t sampled_state = (pins >> PROBE_PIN) & 1;

        // Block on next epoch from Core 0
        uint32_t epoch2 = pop_fifo0();
        uint32_t pins2 = read_gpio_pins();
        uint32_t sampled_state2 = (pins2 >> PROBE_PIN) & 1;

        // Return verified state mask back to Core 0 over FIFO 1 (unblocking it)
        uint32_t ack_payload = (epoch & 0xFFFF) | (sampled_state << 16) | (sampled_state2 << 17);
        (void)epoch2;
        push_fifo1(ack_payload);
    }
}

