#![allow(unused_imports)]
use builtin::*;
use builtin_macros::*;
use vstd::prelude::*;
use vstd::slice::*;

pub mod infinitelog_t;
pub mod logimpl_v;
pub mod main_t;
pub mod math;
pub mod pmemspec_t;
pub mod sccf;

use crate::main_t::*;
use crate::pmemspec_t::*;

verus! {

    fn main() {
        if let Ok(mut pm) = PersistentMemory::create(4096) {
            if let Ok(_) = InfiniteLogImpl::setup(&mut pm) {
                if let Ok(mut log) = InfiniteLogImpl::start(pm) {
                    let mut v: Vec<u8> = Vec::<u8>::new();
                    v.push(30); v.push(42); v.push(100);
                    if let Ok(pos) = log.append(&v) {
                        if let Ok((head, tail, capacity)) = log.get_head_and_tail() {
                            assert (head == 0);
                            assert (tail == 3);
                            // TODO: add an assertion using maybe_corrupted here
                            // if let Ok(bytes) = log.read(1, 2) {
                            //     assert (bytes@[0] == 42);
                            // }
                        }
                    }
                }
            }
        }
    }

}
