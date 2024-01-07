static mut STATUS_INFO: String = String::new();
static mut SPLIT_NUM: usize = 16;

pub fn set_status_info(info: String) {
	unsafe {
		STATUS_INFO = info;
	}
}

pub fn get_status_info() -> String {
	unsafe {
		STATUS_INFO.clone()
	}
}

pub fn set_split_num(split_num: String) {
	unsafe {
		SPLIT_NUM = match split_num.parse() {
			Ok(n) => n,
			Err(_) => SPLIT_NUM,
		};

		set_status_info(format!("Set Split Num to {}", SPLIT_NUM));
	}
}

pub fn get_split_num() -> i32 {
	unsafe {
		SPLIT_NUM as i32
	}
}
