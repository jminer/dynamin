
use super::super::backend::GenericWindowBackend;


type HANDLE = uint;

pub struct WindowBackend {
	tmp: int
}

impl GenericWindowBackend for WindowBackend {
	fn new() -> WindowBackend {
		WindowBackend { tmp: 0 }
	}
}

