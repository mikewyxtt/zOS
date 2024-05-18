/*  fb.rs - UEFI framebuffer interface
 *
 *  zOS  --  Advanced *NIX System
 *  Copyright (C) 2024  Free Software Foundation, Inc.
 *
 *  zOS is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  zOS is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with zOS. If not, see <http://www.gnu.org/licenses/>.
 */

#![allow(dead_code)]

use core::{ptr, sync::atomic::Ordering};
use crate::lib::rwlock::RwLock;
use super::libuefi::bootservices::BootServices;
use super::libuefi::protocol::graphics_output::GraphicsOutputProtocol;

static FB: RwLock<Framebuffer> = RwLock::new(Framebuffer::null());

const DEFAULT_HORIZONTAL_RESOLUTION: u32 = 1024;
const DEFAULT_VERTICAL_RESOLUTION: u32 = 768;



pub struct Framebuffer {
    pub base_addr:  *const u8,
    pub pitch:      u32,
    pub width:      u32,
    pub height:     u32,
    pub size:       usize,
    pub depth:      u32,
    pub enabled:    bool,
}


impl Framebuffer {
    pub const fn null() -> Self {
        let mut fb: Self = unsafe { core::mem::zeroed::<Self>() };
        fb.base_addr = ptr::null();

        fb
    }

    pub fn as_array(&self) -> &'static [u8] {
        let size: usize = (self.width * self.height * self.depth).try_into().unwrap();
        unsafe { core::slice::from_raw_parts(self.base_addr, size) }
    }

    pub fn as_mut_array(&self) -> &'static mut [u8] {
        let size: usize = (self.width * self.height * self.depth).try_into().unwrap();
        unsafe { core::slice::from_raw_parts_mut(self.base_addr as *mut u8, size) }
    }

    pub fn plot_pixel(&self, x: usize, y: usize, _color: u32) {
        let buff: &mut [u8] = self.as_mut_array();

        match self.depth {
            4 => {
                let pos = (x + y * self.pitch as usize) * self.depth as usize;
                buff[pos] = 0xFF;
                buff[pos + 1] = 0xFF;
                buff[pos + 2] = 0xFF;
            }

            _ => {
                //
            }
        }
    }

    pub fn set_resolution(horizontal: u32, vertical: u32) -> Result<(), ()> {
        let efi_sys_table = unsafe { &*super::libuefi::SYSTEM_TABLE_PTR.load(Ordering::Relaxed) };
        let gfx_proto = BootServices::handle_protocol::<GraphicsOutputProtocol>(efi_sys_table.console_out_handle.cast());
    
        let max_mode = unsafe { (*gfx_proto.mode).max_mode };
        let mut mode_set = false;
    
        for mode_num in 0..max_mode {
            let info = gfx_proto.query_mode(mode_num).unwrap().1;
    
            if info.horizontal_resolution == horizontal && info.vertical_resolution == vertical {
                let _ = gfx_proto.set_mode(mode_num);
                mode_set = true;
            }
    
        }
    
        if mode_set {
            let mode =  unsafe { &*gfx_proto.mode };
            let mode_info =  unsafe { &*mode.mode_info };
            let mut fb = FB.write().unwrap();
    
            fb.enabled = true;
            fb.base_addr = mode.fb_base;
            fb.pitch = mode_info.pixels_per_scanline as u32;
            fb.width = mode_info.horizontal_resolution;
            fb.height = mode_info.vertical_resolution;
            fb.size = mode.fb_size;
            fb.depth = fb.size as u32 / fb.width / fb.height;

            return Ok(());
        }
        else {
            return Err(());
        }
    }
}

pub fn get_active_fb() -> Result<&'static RwLock<Framebuffer>, ()> {
    let fb = FB.read().unwrap();

    if fb.enabled {
        assert_ne!(fb.base_addr, ptr::null());
        Ok(&FB)
    }
    else {
        Err(())
    }
}


pub fn init() {
    Framebuffer::set_resolution(DEFAULT_HORIZONTAL_RESOLUTION, DEFAULT_VERTICAL_RESOLUTION).expect("Could not set resolution to 1024x768");
}