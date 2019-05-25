pub mod interval;
pub mod interval_set;

#[cfg(test)]
mod tests {
    use crate::interval_set::IntervalSet;

    #[test]
    fn it_works() {    
    	let set1 = IntervalSet::empty();
    	set1.add_range(10, 50);
    	set1.add_range(1, 8);

    	let set2 = IntervalSet::empty();
    	set2.add_range(3, 7);

    	let set3 = set1.subtract(&set2);
        assert_eq!(3, set3.length());
    }
}
