use egui_winit::winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

const KEY_BACKSPACE: u8 = 0x08;
const KEY_ENTER: u8 = 0x0d;
const KEY_RIGHT: u8 = 0x1c;
const KEY_LEFT: u8 = 0x1d;
const KEY_UP: u8 = 0x1e;
const KEY_DOWN: u8 = 0x1f;
const KEY_SPACE: u8 = 0x20;
const KEY_QUOTES: u8 = 0x22;
const KEY_LPAREN: u8 = 0x28;
const KEY_RPAREN: u8 = 0x29;
const KEY_COMMA: u8 = 0x2c;
const KEY_0: u8 = 0x30;
const KEY_1: u8 = 0x31;
const KEY_2: u8 = 0x32;
const KEY_3: u8 = 0x33;
const KEY_4: u8 = 0x34;
const KEY_5: u8 = 0x35;
const KEY_6: u8 = 0x36;
const KEY_7: u8 = 0x37;
const KEY_8: u8 = 0x38;
const KEY_9: u8 = 0x39;
const KEY_COLON: u8 = 0x3a;
const KEY_EQUALS: u8 = 0x3d;
const KEY_A: u8 = 0x41;
const KEY_B: u8 = 0x42;
const KEY_D: u8 = 0x44;
const KEY_E: u8 = 0x45;
const KEY_F: u8 = 0x46;
const KEY_H: u8 = 0x48;
const KEY_I: u8 = 0x49;
const KEY_K: u8 = 0x4b;
const KEY_L: u8 = 0x4c;
const KEY_M: u8 = 0x4d;
const KEY_N: u8 = 0x4e;
const KEY_O: u8 = 0x4f;
const KEY_P: u8 = 0x50;
const KEY_R: u8 = 0x52;
const KEY_S: u8 = 0x53;
const KEY_T: u8 = 0x54;
const KEY_U: u8 = 0x55;
const KEY_X: u8 = 0x58;
const KEY_Y: u8 = 0x59;

const KEYMOD_CTRL : u8 = 0x01;
const KEYMOD_SHIFT: u8 = 0x02;
const KEYMOD_KANA : u8 = 0x04;
const KEYMOD_CAPS : u8 = 0x08;
const KEYMOD_GRAPH: u8 = 0x10;

pub struct Keyboard {
    pub key_pressed: u8,
    keymods_active: u8,
    shift_held: bool,

    last_press: u16,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            key_pressed: 0x00,
            keymods_active: 0xff,
            shift_held: false,
            last_press: 0, // for check_shift
        }
    }

    pub fn check_shift(&self) -> u8 {
        // Call this after check_keyboard_press
        /*
        all of those are active low
        x--- ---- TEN: Numpad, Function key, special input key
        -x-- ---- KIN: Valid key
        --x- ---- REP: Key repeat
        ---x ---- GRAPH key ON
        ---- x--- CAPS lock ON
        ---- -x-- KANA lock ON
        ---- --x- SHIFT ON
        ---- ---x CTRL ON
        */
        let mut ret = 0xff;
        if self.shift_held {
            ret &= 0xff-KEYMOD_SHIFT;
        }
        if self.last_press != 0 {
            ret &= 0xff-0x40;
        }
        if (self.last_press & 0x100) != 0 {
            ret &= 0xff-0x80;
        }
        ret
    }

    pub fn check_press(&mut self) -> u16 {
        let mut ret = 0x00;
        ret |= self.key_pressed as u16;
        self.last_press = ret;
        ret
    }

    fn set_key_pressed(&mut self, input: &WinitInputHelper, keycode: VirtualKeyCode, val: u8) {
        if input.key_pressed(keycode) {
            self.key_pressed = val;
        }
    }

    pub fn set_btns_pressed(&mut self, input: &WinitInputHelper) {
        self.key_pressed = 0x00;
        self.last_press = 0x00;
        self.shift_held = false;
        self.set_key_pressed(input, VirtualKeyCode::Back, KEY_BACKSPACE);
        self.set_key_pressed(input, VirtualKeyCode::Return, KEY_ENTER);
        self.set_key_pressed(input, VirtualKeyCode::Right, KEY_RIGHT);
        self.set_key_pressed(input, VirtualKeyCode::Left, KEY_LEFT);
        self.set_key_pressed(input, VirtualKeyCode::Up, KEY_UP);
        self.set_key_pressed(input, VirtualKeyCode::Down, KEY_DOWN);
        self.set_key_pressed(input, VirtualKeyCode::Space, KEY_SPACE);

        if input.key_pressed(VirtualKeyCode::Key2) {
            self.shift_held = true;
            self.key_pressed = KEY_QUOTES;
        }
        if input.key_pressed(VirtualKeyCode::Key9) {
            self.shift_held = true;
            self.key_pressed = KEY_LPAREN;
        }
        if input.key_pressed(VirtualKeyCode::Key0) {
            self.shift_held = true;
            self.key_pressed = KEY_RPAREN;
        }

        self.set_key_pressed(input, VirtualKeyCode::Comma, KEY_COMMA);
        self.set_key_pressed(input, VirtualKeyCode::Numpad0, KEY_0);
        self.set_key_pressed(input, VirtualKeyCode::Numpad1, KEY_1);
        self.set_key_pressed(input, VirtualKeyCode::Numpad2, KEY_2);
        self.set_key_pressed(input, VirtualKeyCode::Numpad3, KEY_3);
        self.set_key_pressed(input, VirtualKeyCode::Numpad4, KEY_4);
        self.set_key_pressed(input, VirtualKeyCode::Numpad5, KEY_5);
        self.set_key_pressed(input, VirtualKeyCode::Numpad6, KEY_6);
        self.set_key_pressed(input, VirtualKeyCode::Numpad7, KEY_7);
        self.set_key_pressed(input, VirtualKeyCode::Numpad8, KEY_8);
        self.set_key_pressed(input, VirtualKeyCode::Numpad9, KEY_9);
        self.set_key_pressed(input, VirtualKeyCode::Colon, KEY_COLON);
        self.set_key_pressed(input, VirtualKeyCode::Equals, KEY_EQUALS);

        if input.held_shift() {
            self.shift_held = true;
            self.set_key_pressed(input, VirtualKeyCode::A, KEY_A);
            self.set_key_pressed(input, VirtualKeyCode::B, KEY_B);
            self.set_key_pressed(input, VirtualKeyCode::D, KEY_D);
            self.set_key_pressed(input, VirtualKeyCode::E, KEY_E);
            self.set_key_pressed(input, VirtualKeyCode::F, KEY_F);
            self.set_key_pressed(input, VirtualKeyCode::H, KEY_H);
            self.set_key_pressed(input, VirtualKeyCode::I, KEY_I);
            self.set_key_pressed(input, VirtualKeyCode::K, KEY_K);
            self.set_key_pressed(input, VirtualKeyCode::L, KEY_L);
            self.set_key_pressed(input, VirtualKeyCode::M, KEY_M);
            self.set_key_pressed(input, VirtualKeyCode::N, KEY_N);
            self.set_key_pressed(input, VirtualKeyCode::O, KEY_O);
            self.set_key_pressed(input, VirtualKeyCode::P, KEY_P);
            self.set_key_pressed(input, VirtualKeyCode::R, KEY_R);
            self.set_key_pressed(input, VirtualKeyCode::S, KEY_S);
            self.set_key_pressed(input, VirtualKeyCode::T, KEY_T);
            self.set_key_pressed(input, VirtualKeyCode::U, KEY_U);
            self.set_key_pressed(input, VirtualKeyCode::X, KEY_X);
            self.set_key_pressed(input, VirtualKeyCode::Y, KEY_Y);
        }
    }
}
