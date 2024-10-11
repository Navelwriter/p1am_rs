//Now we can define the constants in Rust

pub const NUMBER_OF_MODULES: u8 = 15;
pub const MOD_HDR: u8 = 			0x02;
pub const VERSION_HDR: u8 = 		0x03;
pub const ACTIVE_HDR: u8 =			0x04;
pub const DROPOUT_HDR: u8 =			0x05;
pub const CFG_HDR: u8 =				0x10;
pub const READ_CFG_HDR: u8 = 		0x11;
pub const PETWD_HDR: u8 =			0x30;
pub const STARTWD_HDR: u8 =			0x31;
pub const STOPWD_HDR: u8 =			0x32;
pub const CONFIGWD_HDR: u8 =		0x33;
pub const READ_STATUS_HDR: u8 = 	0x40;
pub const READ_DISCRETE_HDR: u8 = 	0x50;
pub const READ_ANALOG_HDR: u8 =		0x51;
pub const READ_BLOCK_HDR: u8 =		0x52;
pub const WRITE_DISCRETE_HDR: u8 = 	0x60;
pub const WRITE_ANALOG_HDR: u8 =	0x61;
pub const WRITE_BLOCK_HDR: u8 =		0x62;
pub const FW_UPDATE_HDR: u8 =		0xAA;

pub const DUMMY: u8 =		0xFF;
pub const EMPTY_SLOT_ID: u32 = 0xFFFFFFFE;
pub const MAX_TIMEOUT: u32 =	  0xFFFFFFFF;

pub const DISCRETE_IN_BLOCK: u8 =	0;
pub const ANALOG_IN_BLOCK: u8 =		1;
pub const DISCRETE_OUT_BLOCK: u8 =	2;
pub const ANALOG_OUT_BLOCK: u8 =	3;
pub const STATUS_IN_BLOCK: u8 =		4;

pub const MISSING24V_STATUS: u8 = 	3;
pub const BURNOUT_STATUS: u8 =		5;
pub const UNDER_RANGE_STATUS: u8 =	7;
pub const OVER_RANGE_STATUS: u8 =	11;

pub const TOGGLE: u8 =				0x01;
pub const HOLD: u8 =				0x00;