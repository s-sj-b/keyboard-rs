
mod scancode;
use scancode::*;

// for use in keyboard firmware
// DO NOT SEND THESE AS SCANCODES
pub const FN: u8 = 0xEE;
pub const RAISE: u8 = 0xEF;
pub const LOWER: u8 = 0xFE;

pub type Layer = [[u8; 12]; 5];

pub const LAYER_1: [[u8; 12]; 5] = [
    [KEYBOARD_CAPSLOCK, KEYBOARD_1, KEYBOARD_2, KEYBOARD_3, KEYBOARD_4, KEYBOARD_5, KEYBOARD_6, KEYBOARD_7, KEYBOARD_8, KEYBOARD_9, KEYBOARD_0, KEYBOARD_DELETE_FORWARD],
    [KEYBOARD_TAB, KEYBOARD_Q, KEYBOARD_W, KEYBOARD_E, KEYBOARD_R, KEYBOARD_T, KEYBOARD_Y, KEYBOARD_U, KEYBOARD_I, KEYBOARD_O, KEYBOARD_P, KEYBOARD_DELETE],
    [KEYBOARD_ESCAPE, KEYBOARD_A, KEYBOARD_S, KEYBOARD_D, KEYBOARD_F, KEYBOARD_G, KEYBOARD_H, KEYBOARD_J, KEYBOARD_K, KEYBOARD_L, KEYBOARD_SEMICOLON, KEYBOARD_APOSTROPHE],
    [KEYBOARD_LEFTSHIFT, KEYBOARD_Z, KEYBOARD_X, KEYBOARD_C, KEYBOARD_V, KEYBOARD_B, KEYBOARD_N, KEYBOARD_M, KEYBOARD_COMMA, KEYBOARD_PERIOD, KEYBOARD_FORWARD_SLASH, KEYBOARD_RETURN_ENTER],
    [KEYBOARD_LEFTCONTROL, FN, KEYBOARD_LEFTGUI, KEYBOARD_LEFTALT, LOWER, KEYBOARD_SPACEBAR, KEYBOARD_SPACEBAR, RAISE, KEYBOARD_LEFTARROW, KEYBOARD_UPARROW, KEYBOARD_DOWNARROW, KEYBOARD_RIGHTARROW],
];

pub const LAYER_RAISE: [[u8; 12]; 5] = [
    [0; 12],
    [0; 12],
    [0; 12],
    [0; 12],
    [0; 12],
];

// need
pub const LAYER_LOWER: Layer = [
    [0; 12],
    [0; 12],
    [0; 12],
    [0; 12],
    [0; 12],
];


pub const LAYERS: [Layer; 3] = [LAYER_LOWER, LAYER_1, LAYER_RAISE];

pub struct Keyboard {
    raise_pressed: bool,
    lower_pressed: bool,
    current_layer: usize,

    layers: [Layer; 3],
}

impl Keyboard {
    pub fn default() -> Self {
        Self {
            raise_pressed: false,
            lower_pressed: false,
            current_layer: 1,
            layers: LAYERS,
        }
    }
}