use auxtools::*;
use detour::RawDetour;

// Adapted from https://github.com/Putnam3145/auxmaptick

#[cfg(windows)]
static SEND_MAPS_SIGNATURE: &[Option<u8>] =
	signature!("55 8B EC 6A FF 68 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 50 81 EC ?? ?? ?? ?? A1 ?? ?? ?? ?? 33 C5 89 45 F0 53 56 57 50 8D 45 F4 ?? ?? ?? ?? ?? ?? A0 ?? ?? ?? ?? 04 01 75 05 E8 ?? ?? ?? ?? E8");

#[cfg(unix)]
static SEND_MAPS_SIGNATURE: &[Option<u8>] =
    signature!("55 89 E5 57 56 53 81 EC ?? ?? ?? ?? 80 3D ?? ?? ?? ?? ?? 0F 84 ?? ?? ?? ??");

static mut SEND_MAPS_ORIGINAL: Option<unsafe extern "C" fn()> = None;

#[init(full)]
fn hook_send_maps(_ctx: &DMContext) -> Result<(), String> {
    let byondcore = match sigscan::Scanner::for_module(BYONDCORE) {
        Some(scanner) => scanner,
        None => {
            return Err(String::from(
                "MAPTICK: Failed to create scanner for the byondcore module!",
            ))
        }
    };

    let send_maps_byond = match byondcore.find(SEND_MAPS_SIGNATURE) {
        Some(ptr) => ptr,
        None => return Err(String::from("MAPTICK: Failed to locate SendMaps!")),
    };

    unsafe {
        let tick_hook =
            RawDetour::new(send_maps_byond as *const (), map_tick_hook as *const ()).unwrap();

        if let Err(e) = tick_hook.enable() {
            return Err(format!("MAPTICK: Failed to enable SendMaps hook: {}", e));
        }

        let ret = std::mem::transmute(tick_hook.trampoline());
        std::mem::forget(tick_hook);
        SEND_MAPS_ORIGINAL = Some(ret);
    }
    Ok(())
}

#[no_mangle]
unsafe extern "C" fn map_tick_hook() {
    let start = std::time::Instant::now();
    SEND_MAPS_ORIGINAL.unwrap()();
    Value::globals().set(
        "internal_tick_usage",
        start.elapsed().as_micros() as f32 / 100000.0,
    );
}
