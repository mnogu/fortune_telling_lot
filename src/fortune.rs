// SPDX-License-Identifier: GPL-2.0

//! fortune-telling lot

use kernel::bindings;
use kernel::error;
use kernel::prelude::*;

module! {
    type: Fortune,
    name: "fortune",
    author: "Muneyuki Noguchi",
    description: "fortune-telling lot",
    license: "GPL",
}

// https://rust-for-linux.github.io/docs/src/kernel/random.rs.html
fn getrandom(dest: &mut [u8]) -> Result {
    let res = unsafe { bindings::wait_for_random_bytes() };
    if res != 0 {
        return error::to_result(res);
    }
    unsafe {
        bindings::get_random_bytes(dest.as_mut_ptr() as *mut core::ffi::c_void, dest.len());
    }
    Ok(())
}

struct Fortune {}

impl kernel::Module for Fortune {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let candidates = [
            "大吉 (Great blessing)",
            "吉 (Blessing)",
            "中吉 (Middle blessing)",
            "小吉 (Small blessing)",
            "末吉 (Future blessing)",
            "凶 (Misfortune)",
            "大凶 (Great misfortune)",
        ];
        let mut dest = [0; 1];
        getrandom(&mut dest)?;
        pr_info!("{}\n", candidates[dest[0] as usize % candidates.len()]);
        Ok(Fortune {})
    }
}
