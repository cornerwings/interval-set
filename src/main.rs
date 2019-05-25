extern crate interval_set;

use interval_set::interval_set::IntervalSet;

fn main() {
	let mut set1 = IntervalSet::empty();
	set1 = set1.add_range(10, 50);
	set1 = set1.add_range(3, 8);
	set1 = set1.add_range(1, 5);
	print!("set1 {:?}\n", set1);

	let mut set2 = IntervalSet::empty();
	set2 = set2.add_range(3, 7);
	//set2 = set2.add_range(13, 15);
	print!("set2 {:?}\n", set2);

	let mut set3 = set1.subtract(&set2);
	// set3 = set3.add_range(3, 8);
	print!("set3 {:?}\n", set3);
}