#![allow(unused_imports)]
use builtin::*;
use builtin_macros::*;
use vstd::prelude::*;

pub mod multilog;
pub mod pmem;

use crate::multilog::multilogimpl_t::*;
use crate::multilog::multilogimpl_v::*;
use crate::pmem::device_t::*;
use crate::pmem::pmemmock_t::*;
use crate::pmem::pmemspec_t::*;
use crate::pmem::pmemutil_v::*;

verus! {
    fn main() {}

    fn test_multilog_with_timestamps() -> bool {
        proof { lemma_auto_if_no_outstanding_writes_then_flush_is_idempotent(); }

        let device_capacity = 1024;
        let log_capacity = 256;
        let mut device_regions = Vec::new();
        device_regions.push(log_capacity); device_regions.push(log_capacity);
        device_regions.push(log_capacity); device_regions.push(log_capacity);
        let ghost old_device_regions = device_regions@;

        let device = VolatileMemoryMockingPersistentMemoryDevice::new(device_capacity);

        // Required to pass the precondition for get_regions -- we need to unroll the
        // recursive spec fn `fold_left` enough times to calculate the sum of
        // all of the PM regions.
        proof { reveal_with_fuel(Seq::fold_left, 5); }
        let result = device.get_regions(device_regions);

        let mut regions = match result {
            Ok(regions) => regions,
            Err(()) => return false
        };

        let mut multilog1_regions = Vec::new();
        let mut multilog2_regions = Vec::new();
        multilog1_regions.push(regions.pop().unwrap());
        multilog1_regions.push(regions.pop().unwrap());
        multilog2_regions.push(regions.pop().unwrap());
        multilog2_regions.push(regions.pop().unwrap());

        let mut multilog1_regions = VolatileMemoryMockingPersistentMemoryRegions::combine_regions(multilog1_regions);
        let mut multilog2_regions = VolatileMemoryMockingPersistentMemoryRegions::combine_regions(multilog2_regions);

        let result = MultiLogImpl::setup(&mut multilog1_regions);
        let (log1_capacities, multilog_id1) = match result {
            Ok((log1_capacities, multilog_id)) => (log1_capacities, multilog_id),
            Err(_) => return false
        };

        assert(UntrustedMultiLogImpl::recover(multilog1_regions@.committed(), multilog_id1).is_Some());

        let result = MultiLogImpl::setup(&mut multilog2_regions);
        let (log2_capacities, multilog_id2) = match result {
            Ok((log2_capacities, multilog_id)) => (log2_capacities, multilog_id),
            Err(_) => return false
        };

        let mut device2_regions = Vec::new();
        device2_regions.push(log_capacity); device2_regions.push(log_capacity);
        device2_regions.push(log_capacity); device2_regions.push(log_capacity);

        let device2 = VolatileMemoryMockingPersistentMemoryDevice::new(device_capacity);
        let result = device2.get_regions(device2_regions);

        let mut regions = match result {
            Ok(regions) => regions,
            Err(()) => return false
        };

        let mut multilog3_regions = Vec::new();
        let mut multilog4_regions = Vec::new();
        multilog3_regions.push(regions.pop().unwrap());
        multilog3_regions.push(regions.pop().unwrap());
        multilog4_regions.push(regions.pop().unwrap());
        multilog4_regions.push(regions.pop().unwrap());

        let mut multilog3_regions = VolatileMemoryMockingPersistentMemoryRegions::combine_regions(multilog3_regions);
        let mut multilog4_regions = VolatileMemoryMockingPersistentMemoryRegions::combine_regions(multilog4_regions);


        let result = MultiLogImpl::setup(&mut multilog3_regions);
        let (log3_capacities, multilog_id3) = match result {
            Ok((log3_capacities, multilog_id)) => (log3_capacities, multilog_id),
            Err(_) => return false
        };


        let result = MultiLogImpl::setup(&mut multilog4_regions);
        let (log4_capacities, multilog_id4) = match result {
            Ok((log4_capacities, multilog_id)) => (log4_capacities, multilog_id),
            Err(_) => return false
        };

        // proof {
        //     let (flushed_regions, new_timestamp) = region1@.flush(timestamp@);
        //     assert(flushed_regions.committed() =~= region1@.committed());

        //     let (flushed_regions, new_timestamp) = region2@.flush(timestamp@);
        //     assert(flushed_regions.committed() =~= region2@.committed());
        // }

        let result = MultiLogImpl::start(multilog1_regions, multilog_id1);
        let multilog1 = match result {
            Ok(multilog) => multilog,
            Err(_) => return false
        };

        // // // This should not verify, because `timestamp2` does not correspond to `region2`,
        // // // even though `timestamp` and `timestamp2` have the same numerical value right now.
        // // let result2 = MultiLogImpl::start(region2, multilog_id1, timestamp2);

        true
    }
}
