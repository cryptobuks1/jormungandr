use crate::testing::node::explorer::Explorer;
use jormungandr_lib::interfaces::BlockDate;
use std::convert::{TryFrom, TryInto};

pub fn wait_for_epoch(epoch_id: u64, mut explorer: Explorer) {
    explorer.enable_logs();
    while u64::try_from(
        explorer
            .last_block()
            .unwrap()
            .data
            .unwrap()
            .main_tip
            .block
            .date
            .epoch
            .id,
    )
    .unwrap()
        < epoch_id
    {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

pub fn wait_for_date(target_block_date: BlockDate, mut explorer: Explorer) {
    explorer.enable_logs();

    loop {
        let current_block_date = explorer
            .last_block()
            .unwrap()
            .data
            .unwrap()
            .main_tip
            .block
            .date;

        let epoch = current_block_date.epoch.id.try_into().unwrap();
        let slot_id = current_block_date.slot.parse::<u32>().unwrap();

        let current_block_date = BlockDate::new(epoch, slot_id);

        if target_block_date <= current_block_date {
            return;
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
