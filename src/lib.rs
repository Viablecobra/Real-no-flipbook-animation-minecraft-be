use std::ffi::{CStr, CString};
use std::fs;
use std::path::PathBuf;
use libc::c_char;

fn get_android_minecraft_paths() -> Vec<PathBuf> {
    vec![
        PathBuf::from("/storage/emulated/0/games/com.mojang"),
        PathBuf::from("/sdcard/games/com.mojang"),
        PathBuf::from("/storage/sdcard0/games/com.mojang"),
        PathBuf::from("/data/data/com.mojang.minecraftpe/files/games/com.mojang"),
    ]
}

fn find_minecraft_config() -> Option<PathBuf> {
    for path in get_android_minecraft_paths() {
        let options_file = path.join("options.txt");
        if options_file.exists() {
            return Some(path);
        }
    }
    None
}

fn disable_flipbook_animation(base_path: &PathBuf) -> bool {
    let options_path = base_path.join("options.txt");
    
    if let Ok(content) = fs::read_to_string(&options_path) {
        let mut lines: Vec<String> = content.lines().map(String::from).collect();
        let mut found = false;
        let mut modified = false;
        
        for line in lines.iter_mut() {
            if line.starts_with("flipbook_animation:") {
                *line = "flipbook_animation:false".to_string();
                found = true;
                modified = true;
                break;
            }
        }
        
        if !found {
            lines.push("flipbook_animation:false".to_string());
            modified = true;
        }
        
        if modified {
            fs::write(&options_path, lines.join("\n")).is_ok()
        } else {
            true // Already disabled
        }
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn disable_minecraft_flipbook() -> i32 {
    if let Some(minecraft_path) = find_minecraft_config() {
        if disable_flipbook_animation(&minecraft_path) {
            0 // Success
        } else {
            -1 // Failed to modify
        }
    } else {
        -2 // Minecraft not found
    }
}

#[no_mangle]
pub extern "C" fn disable_minecraft_flipbook_detailed() -> *mut c_char {
    match find_minecraft_config() {
        Some(minecraft_path) => {
            if disable_flipbook_animation(&minecraft_path) {
                CString::new("SUCCESS: Flipbook animations disabled").unwrap().into_raw()
            } else {
                CString::new("ERROR: Failed to modify configuration").unwrap().into_raw()
            }
        }
        None => CString::new("ERROR: Minecraft installation not found").unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn is_minecraft_installed() -> i32 {
    if find_minecraft_config().is_some() {
        1
    } else {
        0
    }
}