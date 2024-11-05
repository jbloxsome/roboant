/*!
 * Blink the builtin LED - the "Hello World" of embedded programming.
 */
#![no_std]
#![no_main]

use panic_halt as _;
use embedded_hal::digital::OutputPin;

// Calibration constants - we'll adjust these
const SERVO0_MIN_PULSE: u32 = 1100;
const SERVO0_MAX_PULSE: u32 = 2100;

const SERVO1_MIN_PULSE: u32 = 1300;
const SERVO1_MAX_PULSE: u32 = 1800;

const SERVO2_MIN_PULSE: u32 = 1100;
const SERVO2_MAX_PULSE: u32 = 2100;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let mut servo0 = pins.d2.into_output();
    let mut servo1 = pins.d3.into_output();
    let mut servo2 = pins.d4.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    fn set_servo_position<PIN: OutputPin>(
        servo: &mut PIN,
        pulse_width: u32
    ) {
        servo.set_high();
        arduino_hal::delay_us(pulse_width);
        servo.set_low();
        arduino_hal::delay_us(20000 - pulse_width);
    }

    fn run_forward<PIN0: OutputPin, PIN1: OutputPin, PIN2: OutputPin>(servo0: &mut PIN0, servo1: &mut PIN1, servo2: &mut PIN2, steps: u32) {
        for _ in 0..steps {
            // Move servo1 to the rightmost position (this is the servo which controls the ant's middle leg).
            // Moving to the rightmost position moves the middle leg down, which lifts the ant's body up
            // and allows the other legs to push it forward.
            for _ in 0..10 {
                set_servo_position(servo1, SERVO1_MAX_PULSE);
            }

            // Move servo0 and servo2 to min (these are the front and back servos which control the ant's front and back legs). Moving to min moves the
            // left legs back and right legs forward.
            for _ in 0..10 {
                set_servo_position(servo0, SERVO0_MIN_PULSE);
                set_servo_position(servo2, SERVO2_MIN_PULSE);
            }

            // Move servo1 to the leftmost position (this is the servo which controls the ant's middle leg). Moving to the leftmost position moves the
            // middle leg up, which drops the ant's body down and allows the legs to push it forward.
            for _ in 0..10 {
                set_servo_position(servo1, SERVO1_MIN_PULSE);
            }

            // Move servo0 and servo2 to max (these are the front and back servos which control the ant's front and back legs). Moving to max moves the
            // left legs forward and right legs back.
            for _ in 0..10 {
                set_servo_position(servo0, SERVO0_MAX_PULSE);
                set_servo_position(servo2, SERVO2_MAX_PULSE);
            }
        }
    }

    fn run_backward<PIN0: OutputPin, PIN1: OutputPin, PIN2: OutputPin>(servo0: &mut PIN0, servo1: &mut PIN1, servo2: &mut PIN2, steps: u32) {
        for _ in 0..steps {
            // Move servo1 to the rightmost position (this is the servo which controls the ant's middle leg).
            // Moving to the rightmost position moves the middle leg down, which lifts the ant's body up
            // and allows the other legs to push it forward.
            for _ in 0..10 {
                set_servo_position(servo1, SERVO1_MAX_PULSE);
            }

            // Move servo0 and servo2 to max (these are the front and back servos which control the ant's front and back legs). Moving to max moves the
            // left legs forward and right legs back.
            for _ in 0..10 {
                set_servo_position(servo0, SERVO0_MAX_PULSE);
                set_servo_position(servo2, SERVO2_MAX_PULSE);
            }

            // Move servo1 to the leftmost position (this is the servo which controls the ant's middle leg). Moving to the leftmost position moves the
            // middle leg up, which drops the ant's body down and allows the legs to push it forward.
            for _ in 0..10 {
                set_servo_position(servo1, SERVO1_MIN_PULSE);
            }

            // Move servo0 and servo2 to min (these are the front and back servos which control the ant's front and back legs). Moving to min moves the
            // left legs back and right legs forward.
            for _ in 0..10 {
                set_servo_position(servo0, SERVO0_MIN_PULSE);
                set_servo_position(servo2, SERVO2_MIN_PULSE);
            }
        }
    }

    fn turn_right<PIN0: OutputPin, PIN1: OutputPin, PIN2: OutputPin>(servo0: &mut PIN0, servo1: &mut PIN1, servo2: &mut PIN2, steps: u32) {
        for _ in 0..steps {
            // Lift body by moving middle leg down
            for _ in 0..10 {
                set_servo_position(servo1, SERVO1_MAX_PULSE);
            }

            // Move front and back legs to create turning motion
            for _ in 0..10 {
                set_servo_position(servo0, SERVO0_MIN_PULSE);
                set_servo_position(servo2, SERVO2_MIN_PULSE);
            }

            // Lower body to grip ground
            for _ in 0..10 {
                set_servo_position(servo1, (SERVO1_MIN_PULSE + SERVO1_MAX_PULSE) / 2);
            }

            // Push against ground to turn
            for _ in 0..10 {
                set_servo_position(servo0, SERVO0_MAX_PULSE);
                set_servo_position(servo2, SERVO2_MAX_PULSE);
            }
        }
    }

    fn turn_left<PIN0: OutputPin, PIN1: OutputPin, PIN2: OutputPin>(servo0: &mut PIN0, servo1: &mut PIN1, servo2: &mut PIN2, steps: u32) {
        for _ in 0..steps {
            // Lift body by moving middle leg down
            for _ in 0..10 {
                set_servo_position(servo1, SERVO1_MIN_PULSE);
            }

            // Move front and back legs to create turning motion
            for _ in 0..10 {
                set_servo_position(servo0, SERVO0_MAX_PULSE);
                set_servo_position(servo2, SERVO2_MAX_PULSE);
            }

            // Lower body to grip ground
            for _ in 0..10 {
                set_servo_position(servo1, (SERVO1_MIN_PULSE + SERVO1_MAX_PULSE) / 2);
            }

            // Push against ground to turn
            for _ in 0..10 {
                set_servo_position(servo0, SERVO0_MIN_PULSE);
                set_servo_position(servo2, SERVO2_MIN_PULSE);
            }
        }
    }

    loop {
        run_forward(&mut servo0, &mut servo1, &mut servo2, 5);
        arduino_hal::delay_ms(3000);
        turn_left(&mut servo0, &mut servo1, &mut servo2, 5);
        arduino_hal::delay_ms(3000);
        turn_right(&mut servo0, &mut servo1, &mut servo2, 5);
        arduino_hal::delay_ms(3000);
        run_backward(&mut servo0, &mut servo1, &mut servo2, 5);
        arduino_hal::delay_ms(3000);
    }
}