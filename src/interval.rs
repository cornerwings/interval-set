use std::cmp;

#[derive(Clone, Debug)]
pub struct Interval {
	pub (crate) a: isize,
	pub (crate) b: isize,
}

impl Interval {

	pub fn new(a: isize, b: isize) -> Interval {
		assert!(a <= b);
		Interval {
			a, b
		}
	}

	pub fn length(&self) -> isize {
		if self.b < self.a {
			return 0;
		}
		return self.b - self.a + 1;
	}

	pub fn starts_before(&self, other: &Interval) -> bool { 
		return self.a < other.a; 
	}

	pub fn starts_before_disjoint(&self, other: &Interval) -> bool {
		return self.a < other.a && self.b < other.a;
	}

	pub fn starts_before_non_disjoint(&self, other: &Interval) -> bool {
		return self.a <= other.a && self.b >= other.a;
	}

	pub fn starts_after(&self, other: &Interval) -> bool { 
		return self.a > other.a; 
	}

	pub fn starts_after_disjoint(&self, other: &Interval) -> bool { 
		return self.a > other.b; 
	}

	pub fn starts_after_non_disjoint(&self, other: &Interval) -> bool {
		return self.a > other.a && self.a <= other.b; // this.b>=other.b implied
	}

	pub fn disjoint(&self, other: &Interval) -> bool {
		return self.starts_before_disjoint(other) || self.starts_after_disjoint(other);
	}

	pub fn adjacent(&self, other: &Interval) -> bool {
		return self.a == other.b+1 || self.b == other.a-1;
	}

	pub fn properly_contains(&self, other: &Interval) -> bool {
		return other.a >= self.a && other.b <= self.b;
	}

	pub fn equals(&self, other: &Interval) -> bool {
		return self.a == other.a && self.b == other.b;
	}

	pub fn union(&self, other: &Interval) -> Interval {
		return Interval {
			a: cmp::min(self.a, other.a),
			b: cmp::max(self.b, other.b)
		}
	}

	pub fn intersection(&self, other: &Interval) -> Interval {
		return Interval {
			a: cmp::max(self.a, other.a),
			b: cmp::min(self.b, other.b)
		}
	}

	pub fn difference_not_properly_contained(&self, other: Interval) -> Option<Interval> {
		if other.starts_before_non_disjoint(self) {
			return Some(Interval {
				a: cmp::max(self.a, other.b + 1),
				b: self.b
			})
		} else if other.starts_after_non_disjoint(self) {
			return Some(Interval {
				a: self.a,
				b: other.a - 1
			})
		} else {
			return None
		}
	}

}

