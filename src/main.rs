extern crate trash_can;

use trash_can::modules::garbage_collector::GarbageCollector;

fn main() {
    let mut g_collector = GarbageCollector::init();

    g_collector.work();
}
