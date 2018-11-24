extern crate trash_can;

use trash_can::modules::garbage_info_collector::GarbageInfoCollector;

fn main() {
    let mut collector = GarbageInfoCollector::init();

    collector.work();
}
