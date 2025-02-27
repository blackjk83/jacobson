use systeminfo::{
        consts::SystemHardware,
            from_system_hardware
};

fn get_hw_info() -> SystemHardware {
        from_system_hardware()
}

fn main() {
        println!("{:#?}", get_hw_info())
}
