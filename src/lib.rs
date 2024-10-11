#![no_std]
mod module_defs;
mod defines;
// Some things we need
use embedded_hal::delay::{DelayNs};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::spi::MODE_2;
use embedded_hal::spi::Operation::Transfer;

use rp235x_hal as hal;
use rp235x_hal::timer::CopyableTimer0;

//Create traits for the SPI interface
pub trait SpiDevice<E: core::fmt::Debug> { //Out of CS, SCLK, MOSI, MISO, this trait is for the SPI device as a whole
    fn transfer<'w>(&mut self, read: &'w mut [u8], write: &'w [u8]) -> Result<(), E>; //transfer data, perform read/write. -> returns result of the operation as a Result.
}

pub trait ChipSelectPin<E> { //trait for the chip select pin
    fn set_high(&mut self) -> Result<(), E>; //set the chip select pin high
    fn set_low(&mut self) -> Result<(), E>; //set the chip select pin low
}
pub trait AckPin: InputPin {}
pub trait EnablePin: OutputPin {}

// Global timer variable (initialized to None)
static mut TIMER: Option<hal::Timer<CopyableTimer0>> = None; 

pub struct P1AM<SPI, CS, ACK, EN, E> { //define the P1AM struct
    spi: SPI, //SPI device
    cs: CS, //Chip select pin
    ack: ACK, //Acknowledge pin
    en: EN, //Enable pin
    base_slot: [BaseSlot; 8], //adjust as needed
    _error: core::marker::PhantomData<E>, //PhantomData is a marker type used to inform the compiler that a type is used in a generic context but is not actually used in the runtime.
}

//define structure for BaseSlot
#[derive(Clone, Copy)]
struct BaseSlot {
    db_loc: usize, //location of the slot in the database
}

impl<SPI, CS, ACK, EN, E: core::fmt::Debug> P1AM<SPI, CS, ACK, EN, E> //implement the P1AM struct
//Impl is a keyword that is used to define a trait for a type. 
//trait is a collection of methods defined for an unknown type: Self.
//Self is a keyword that is used to refer to the current type.
//In this case, the trait is P1AM, and the type is SPI, CS, ACK, EN, and E.
//type is a keyword that is used to define an alias for a type, in this case, the type is P1AM.
where
    SPI: SpiDevice<E>, //SPI device
    CS: ChipSelectPin<E>, //Chip select pin
    ACK: AckPin, //Acknowledge pin
    EN: EnablePin, //Enable pin
    {
    pub fn new(spi: SPI, cs: CS, ack: ACK, en: EN) -> Self { //define the new function for the P1AM struct
        let mut p1am = Self{ //return the P1AM struct
            spi, //SPI device
            cs, //Chip select pin
            ack, //Acknowledge pin
            en, //Enable pin
            base_slot: [BaseSlot { db_loc: 0 }; 8], //adjust as needed
            _error: core::marker::PhantomData, 
        };
        p1am.init_pins();
        p1am
    }

    fn init_pins(&mut self) { //initialize the pins
        self.cs.set_high().ok(); //set the chip select pin high
        self.en.set_high().ok(); //set the enable pin high
    }

    // The -> usize syntax indicates that the function will return a value of type usize.
    pub fn init(&mut self, timer: &mut hal::Timer<CopyableTimer0>) -> usize {

        let mut slots = 0;
        let mut db_loc = 0;
        let mut retry = 0;
        //set the global timer variable to the timer
        unsafe { TIMER = Some(*timer);}
        self.enable_base_controller(true);
        self.delay_ms(100);

        if self.spi_timeout(1000*5000, 0, 0) == false {
            // Serial.println("No Base Controller Activity");
		    // Serial.println("Check External Supply Connection");
            self.delay_ms(1000);
            return 0;
        }

        while (slots == 0 || slots > 15) && retry < 5 {
            if self.handle_hdr(0x00) {
                self.delay_ms(5);
                slots = self.spi_send_recv_byte(0xFF);
                if slots == 0 || slots > 15 {
                    if retry > 2 {
                        self.enable_base_controller(false);
                        self.delay_ms(10);
                        self.enable_base_controller(true);
                        self.delay_ms(10);
                    }
                    retry += 1;
                }
            }
        }

        if retry >= 5 {
            // Serial.println("Zero modules in the base");
            self.delay_ms(500);
            return 0;
        }

        if slots > defines::NUMBER_OF_MODULES {
            // Serial.println("\nToo many modules in Base");
            // Serial.print("Device only supports ");
            // Serial.println(NUMBER_OF_MODULES);
            return 0;
        }

        self.spi_timeout(200, 0x00, 0);

        // Receive module IDs
        let mut module_ids = [0u32; 8];
        let mut buf = [0u8; 8 * 4];
        self.spi_send_recv_buf(&mut buf, slots as usize * 4, true);

        // Convert byte array to u32 array
        for i in 0..slots as usize {
            module_ids[i] = u32::from_be_bytes([buf[i * 4], buf[i * 4 + 1], buf[i * 4 + 2], buf[i * 4 + 3]]);
        }
        let mut base_controller_constants = [0u8; 8 * 7];

        for i in 0..slots as usize {
            db_loc = 0;
            while module_ids[i] != module_defs::MDB[db_loc].module_id {
                if module_defs::MDB[db_loc].module_id == 0xFFFFFFFF {
                    // debugPrintln("Module is not in Module List");
                    break;
                }
                db_loc += 1;
            }
            self.base_slot[i].db_loc = db_loc;

            base_controller_constants[i * 7] = module_defs::MDB[db_loc].di_bytes;
            base_controller_constants[i * 7 + 1] = module_defs::MDB[db_loc].do_bytes;
            base_controller_constants[i * 7 + 2] = module_defs::MDB[db_loc].ai_bytes;
            base_controller_constants[i * 7 + 3] = module_defs::MDB[db_loc].ao_bytes;
            base_controller_constants[i * 7 + 4] = module_defs::MDB[db_loc].status_bytes;
            base_controller_constants[i * 7 + 5] = module_defs::MDB[db_loc].config_bytes;
            base_controller_constants[i * 7 + 6] = module_defs::MDB[db_loc].data_size;
        }

        self.spi_timeout(200, 0x00, 0);
        self.delay_ms(1);
        self.spi_send_recv_buf(&mut base_controller_constants, slots as usize * 7, false); // Changed to false
        self.delay_ms(10);
        // for i in 0..slots as usize {
        //     db_loc = self.base_slot[i].db_loc;
        //     if module_defs::MDB[db_loc].config_bytes > 0 {
        //         let cfg_array = self.load_config_buf(module_defs::MDB[db_loc].module_id);
        //         while !self.configure_module(cfg_array, i + 1) {
        //             // debugPrintln("Working");
        //         }
        //     }
        // }
        slots as usize

    }

    // Helper function to access the global timer and provide a safe interface
    fn delay_ms(&mut self, ms: u32) {
        unsafe {
            //only run if the timer is not None and matches Timer<CopyableTimer0> type
            if let Some(timer) = &mut TIMER {
                timer.delay_ms(ms);
            } 
        }
    }
    
    fn delay_us(&mut self, us: u32) {
        unsafe {
            //only run if the timer is not None and matches Timer<CopyableTimer0> type
            if let Some(timer) = &mut TIMER {
                timer.delay_us(us);
            } 
        }
    }

    fn enable_base_controller(&mut self, state: bool) {
        if state {
            self.en.set_high().ok();
        } else {
            self.en.set_low().ok();
        }
    }

    fn spi_timeout(&mut self, mut us: u32, resend_msg: u8, retry_period: u16,) -> bool 
        {
            let mut retry_time = 0;

            while !self.ack.is_high().unwrap_or(false) && us != 0 {
                self.delay_us(1);
                us -= 1;

                retry_time += 1;
                if retry_period != 0 && retry_time == retry_period {
                    self.spi_send_recv_byte(resend_msg);
                    retry_time = 0;
                }
            }
            self.delay_us(50);

            if us > 0 {
                true
            } else {
                // debugPrintln("Timeout");
                false
            }
        }

    fn spi_send_recv_byte(&mut self, data: u8) -> u8{
        let mut read_buf = [0u8; 1];
        self.spi.transfer(&mut read_buf, &[data]).unwrap();
        read_buf[0]
    }

    fn spi_send_recv_buf(&mut self, buf : &mut [u8], len: usize, return_data: bool) {
        self.cs.set_low().ok(); // Assert CS
        
        if return_data {

            for i in 0..buf.len() {
                let mut read_buf = [0u8; 1];
                self.spi.transfer(&mut read_buf, &[buf[i]]).unwrap();
                buf[i] = read_buf[0];
            }
        } else {
            for i in 0..buf.len() {
                self.spi.transfer(&mut [], &[buf[i]]).unwrap();
            }
        }
    }



    fn handle_hdr(&mut self, HDR: u8) -> bool {
        //Convert to rust
        // while(!digitalRead(slaveAckPin));		//Wait for Base Controller to be out of base scanning
	    // spiSendRecvByte(HDR);					//Send intital Header to ping DMA
	    // return spiTimeout(MAX_TIMEOUT,HDR,2000);
        while !self.ack.is_high().unwrap_or(false) {}
        self.spi_send_recv_byte(HDR);
        self.spi_timeout(defines::MAX_TIMEOUT, HDR, 2000)
    }

}
    
    


