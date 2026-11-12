// ============================================================================
// TRUSTPRINT BIO Multi-Core Sampler Firmware (Core 1)
//
// Target: Baochip-1x PicoRV32 BIO Coprocessor Core 1
// Role: Deterministic loopback state sampler and sequence verifier
// Input Pin: GPIO Pin 20
//
// HARDWARE LOOPBACK WIRING SPECIFICATION:
// Core 0 drives Output on GPIO Pin 21.
// Core 1 samples Input on GPIO Pin 20.
// On physical Dabao evaluation silicon, bridge Pin 21 to Pin 20 using
// a standard 0.1" jumper on the expansion header.
// ============================================================================

#include <stdint.h>
#include "bio.h" // Must be first to place _start at 0x0

#define SAMPLER_INPUT_PIN 20
#define PIN_MASK (1 << SAMPLER_INPUT_PIN)

void main(void) {
    // Configure Pin 20 as Input for Core 1 sampling
    set_gpio_mask(PIN_MASK);
    set_input_pins(PIN_MASK);

    while (1) {
        // Block until Core 0 posts transition epoch to FIFO 0
        uint32_t epoch = pop_fifo0();
        
        // Sample physical pin state
        uint32_t pins = read_gpio_pins();
        uint32_t sampled_state = (pins >> SAMPLER_INPUT_PIN) & 1;

        // Block on next epoch from Core 0
        uint32_t epoch2 = pop_fifo0();
        uint32_t pins2 = read_gpio_pins();
        uint32_t sampled_state2 = (pins2 >> SAMPLER_INPUT_PIN) & 1;

        // Return verified state mask back to Core 0 over FIFO 1 (unblocking it)
        uint32_t ack_payload = (epoch & 0xFFFF) | (sampled_state << 16) | (sampled_state2 << 17);
        (void)epoch2;
        push_fifo1(ack_payload);
    }
}

