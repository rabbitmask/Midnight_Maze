use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use winreg::RegKey;

pub(crate) fn running_in_vm() -> bool {

    false
}
