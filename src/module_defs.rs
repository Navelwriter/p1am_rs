#[derive(Clone, Copy)]
pub struct ModuleDescriptor {
    pub module_id: u32,
    pub di_bytes: u8,
    pub do_bytes: u8,
    pub ai_bytes: u8,
    pub ao_bytes: u8,
    pub status_bytes: u8,
    pub config_bytes: u8,
    pub data_size: u8,
    pub module_name: &'static str,
}

pub const MDB: [ModuleDescriptor; 40] = [
    ModuleDescriptor {
        module_id: 0x00000000,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 0,
        module_name: "Empty",
    },
    ModuleDescriptor {
        module_id: 0x04A00042,
        di_bytes: 1,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-08ND-TTL",
    },
    //Generate the rest of the module descriptors using the following info...
    // {0x04A00081, 1, 0, 0, 0, 0, 0, 1, "P1-08ND3"},	//P1-08ND3
    ModuleDescriptor {
        module_id: 0x04A00081,
        di_bytes: 1,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-08ND3",
    },

	// {0x04A00085, 1, 0, 0, 0, 0, 0, 1, "P1-08NA"},	//P1-08NA
    ModuleDescriptor {
        module_id: 0x04A00085,
        di_bytes: 1,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-08NA",
    },

	// {0x04A00087, 1, 0, 0, 0, 0, 0, 1, "P1-08SIM"},	//P1-08SIM
    ModuleDescriptor {
        module_id: 0x04A00087,
        di_bytes: 1,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-08SIM",
    },

	// {0x04A00088, 1, 0, 0, 0, 0, 0, 1, "P1-08NE3"},	//P1-08NE3
    ModuleDescriptor {
        module_id: 0x04A00088,
        di_bytes: 1,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-08NE3",
    },

	// {0x05200082, 2, 0, 0, 0, 0, 0, 1, "P1-16ND3"},	//P1-16ND3
    ModuleDescriptor {
        module_id: 0x05200082,
        di_bytes: 2,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-16ND3",
    },

	// {0x05200089, 2, 0, 0, 0, 0, 0, 1, "P1-16NE3"},	//P1-16NE3
    ModuleDescriptor {
        module_id: 0x05200089,
        di_bytes: 2,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-16NE3",
    },

	// {0x14030050, 0, 1, 0, 0, 0, 0, 1, "P1-04TRS"},	//P1-04TRS
    ModuleDescriptor {
        module_id: 0x14030050,
        di_bytes: 0,
        do_bytes: 1,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-04TRS",
    },
    
    // {0x1403F481, 0, 0, 0, 32, 4, 4, 0xA0, "P1-04PWM"},	//P1-04PWM
    ModuleDescriptor {
        module_id: 0x1403F481,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 32,
        status_bytes: 4,
        config_bytes: 4,
        data_size: 0xA0,
        module_name: "P1-04PWM",
    },

	// {0x1404008D, 0, 1, 0, 0, 0, 0, 1, "P1-08TA"},	//P1-08TA
    ModuleDescriptor {
        module_id: 0x1404008D,
        di_bytes: 0,
        do_bytes: 1,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-08TA",
    },

	// {0x1404008F, 0, 1, 0, 0, 0, 0, 1, "P1-08TRS"},	//P1-08TRS
    ModuleDescriptor {
        module_id: 0x1404008F,
        di_bytes: 0,
        do_bytes: 1,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-08TRS",
    },

	// {0x14040091, 0, 2, 0, 0, 0, 0, 1, "P1-16TR"},	//P1-16TR
    ModuleDescriptor {
        module_id: 0x14040091,
        di_bytes: 0,
        do_bytes: 2,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-16TR",
    },

	// {0x14050046, 0, 1, 0, 0, 0, 0, 1, "P1-08TD-TTL"}, //P1-08TD-TTL
    ModuleDescriptor {
        module_id: 0x14050046,
        di_bytes: 0,
        do_bytes: 1,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-08TD-TTL",
    },

	// {0x14050081, 0, 1, 0, 0, 0, 0, 1, "P1-08TD1"},	//P1-08TD1
    ModuleDescriptor {
        module_id: 0x14050081,
        di_bytes: 0,
        do_bytes: 1,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-08TD1",
    },

	// {0x14050082, 0, 1, 0, 0, 0, 0, 1, "P1-08TD2"},	//P1-08TD2
    ModuleDescriptor {
        module_id: 0x14050082,
        di_bytes: 0,
        do_bytes: 1,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-08TD2",
    },

	// {0x14080085, 0, 2, 0, 0, 0, 0, 1, "P1-15TD1"},	//P1-15TD1
    ModuleDescriptor {
        module_id: 0x14080085,
        di_bytes: 0,
        do_bytes: 2,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-15TD1",
    },

	// {0x14080086, 0, 2, 0, 0, 0, 0, 1, "P1-15TD2"},	//P1-15TD2
    ModuleDescriptor {
        module_id: 0x14080086,
        di_bytes: 0,
        do_bytes: 2,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-15TD2",
    },

	// {0x24A50081, 1, 1, 0, 0, 0, 0, 1, "P1-16CDR"},	//P1-16CDR
    ModuleDescriptor {
        module_id: 0x24A50081,
        di_bytes: 1,
        do_bytes: 1,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-16CDR",
    },

	// {0x24A50082, 1, 1, 0, 0, 0, 0, 1, "P1-15CDD1"},	//P1-15CDD1
    ModuleDescriptor {
        module_id: 0x24A50082,
        di_bytes: 1,
        do_bytes: 1,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-15CDD1",
    },

	// {0x24A50083, 1, 1, 0, 0, 0, 0, 1, "P1-15CDD2"},	//P1-15CDD2
    ModuleDescriptor {
        module_id: 0x24A50083,
        di_bytes: 1,
        do_bytes: 1,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 1,
        module_name: "P1-15CDD2",
    },

	// {0x34605581, 0, 0, 16, 0, 12, 18, 16, "P1-04AD"},	//P1-04AD
    ModuleDescriptor {
        module_id: 0x34605581,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 16,
        ao_bytes: 0,
        status_bytes: 12,
        config_bytes: 18,
        data_size: 16,
        module_name: "P1-04AD",
    },

    // {0x34605582, 0, 0, 16, 0, 12, 2, 16, "P1-04AD-1"},	//P1-04AD-1
    ModuleDescriptor {
        module_id: 0x34605582,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 16,
        ao_bytes: 0,
        status_bytes: 12,
        config_bytes: 2,
        data_size: 16,
        module_name: "P1-04AD-1",
    },

	// {0x34605583, 0, 0, 16, 0, 12, 2, 16, "P1-04AD-2"},	//P1-04AD-2
    ModuleDescriptor {
        module_id: 0x34605583,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 16,
        ao_bytes: 0,
        status_bytes: 12,
        config_bytes: 2,
        data_size: 16,
        module_name: "P1-04AD-2",
    },

	// {0x34605588, 0, 0, 16, 0, 12, 8, 16, "P1-04RTD"},	//P1-04RTD
    ModuleDescriptor {
        module_id: 0x34605588,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 16,
        ao_bytes: 0,
        status_bytes: 12,
        config_bytes: 8,
        data_size: 16,
        module_name: "P1-04RTD",
    },

	// {0x3460558F, 0, 0, 16, 0, 12, 2, 12, "P1-04ADL-1"}, //P1-04ADL-1
    ModuleDescriptor {
        module_id: 0x3460558F,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 16,
        ao_bytes: 0,
        status_bytes: 12,
        config_bytes: 2,
        data_size: 12,
        module_name: "P1-04ADL-1",
    },

	// {0x34605590, 0, 0, 16, 0, 12, 2, 12, "P1-04ADL-2"}, //P1-04ADL-2
    ModuleDescriptor {
        module_id: 0x34605590,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 16,
        ao_bytes: 0,
        status_bytes: 12,
        config_bytes: 2,
        data_size: 12,
        module_name: "P1-04ADL-2",
    },

	// {0x34608C81, 0, 0, 16, 0, 12, 20, 32, "P1-04THM"},	//P1-04THM
    ModuleDescriptor {
        module_id: 0x34608C81,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 16,
        ao_bytes: 0,
        status_bytes: 12,
        config_bytes: 20,
        data_size: 32,
        module_name: "P1-04THM",
    },

	// {0x34608C8E, 0, 0, 16, 0, 12, 8, 32, "P1-04NTC"}, 	//P1-04NTC
    ModuleDescriptor {
        module_id: 0x34608C8E,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 16,
        ao_bytes: 0,
        status_bytes: 12,
        config_bytes: 8,
        data_size: 32,
        module_name: "P1-04NTC",
    },

	// {0x34A0558A, 0, 0, 32, 0, 12, 2, 12, "P1-08ADL-1"}, //P1-08ADL-1
    ModuleDescriptor {
        module_id: 0x34A0558A,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 32,
        ao_bytes: 0,
        status_bytes: 12,
        config_bytes: 2,
        data_size: 12,
        module_name: "P1-08ADL-1",
    },

	// {0x34A0558B, 0, 0, 32, 0, 12, 2, 12, "P1-08ADL-2"}, //P1-08ADL-2
    ModuleDescriptor {
        module_id: 0x34A0558B,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 32,
        ao_bytes: 0,
        status_bytes: 12,
        config_bytes: 2,
        data_size: 12,
        module_name: "P1-08ADL-2",
    },

	// {0x34A5A481, 2, 0, 36, 36, 4, 12, 0xC0, "P1-02HSC"}, //P1-02HSC
    ModuleDescriptor {
        module_id: 0x34A5A481,
        di_bytes: 2,
        do_bytes: 0,
        ai_bytes: 36,
        ao_bytes: 36,
        status_bytes: 4,
        config_bytes: 12,
        data_size: 0xC0,
        module_name: "P1-02HSC",
    },

	// {0x44035583, 0, 0, 0, 16, 4, 0, 12, "P1-04DAL-1"},	//P1-04DAL-1
    ModuleDescriptor {
        module_id: 0x44035583,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 16,
        status_bytes: 4,
        config_bytes: 0,
        data_size: 12,
        module_name: "P1-04DAL-1",
    },

	// {0x44035584, 0, 0, 0, 16, 4, 0, 12, "P1-04DAL-2"}, //P1-04DAL-2
    ModuleDescriptor {
        module_id: 0x44035584,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 16,
        status_bytes: 4,
        config_bytes: 0,
        data_size: 12,
        module_name: "P1-04DAL-2",
    },

	// {0x44055588, 0, 0, 0, 32, 4, 0, 12, "P1-08DAL-1"}, //P1-08DAL-1
    ModuleDescriptor {
        module_id: 0x44055588,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 32,
        status_bytes: 4,
        config_bytes: 0,
        data_size: 12,
        module_name: "P1-08DAL-1",
    },

	// {0x44055589, 0, 0, 0, 32, 4, 0, 12, "P1-08DAL-2"}, //P1-08DAL-2
    ModuleDescriptor {
        module_id: 0x44055589,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 32,
        status_bytes: 4,
        config_bytes: 0,
        data_size: 12,
        module_name: "P1-08DAL-2",
    },

	// {0x5461A783, 0, 0, 16, 8, 12, 2, 12, "P1-4ADL2DAL-1"}, //P1-4ADL2DAL-1
    ModuleDescriptor {
        module_id: 0x5461A783,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 16,
        ao_bytes: 8,
        status_bytes: 12,
        config_bytes: 2,
        data_size: 12,
        module_name: "P1-4ADL2DAL-1",
    },

	// {0x5461A784, 0, 0, 16, 8, 12, 2, 12, "P1-4ADL2DAL-2"}, //P1-4ADL2DAL-2
    ModuleDescriptor {
        module_id: 0x5461A784,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 16,
        ao_bytes: 8,
        status_bytes: 12,
        config_bytes: 2,
        data_size: 12,
        module_name: "P1-4ADL2DAL-2",
    },
    
    ModuleDescriptor {
        module_id: 0xFFFFFFFF,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 0,
        module_name: "BAD SLOT",
    },

    ModuleDescriptor {
        module_id: 0x00000000,
        di_bytes: 0,
        do_bytes: 0,
        ai_bytes: 0,
        ao_bytes: 0,
        status_bytes: 0,
        config_bytes: 0,
        data_size: 0,
        module_name: "BAD SLOT",
    },
];