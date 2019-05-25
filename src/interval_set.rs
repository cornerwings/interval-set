use interval::Interval;

#[derive(Clone, Debug)]
pub struct IntervalSet {
    intervals: Vec<Interval>
}

impl IntervalSet {

    pub fn empty() -> IntervalSet {
        return IntervalSet {
            intervals: vec![]
        }
    }

    pub fn one(a: isize) -> IntervalSet {
        IntervalSet::range(a, a)
    }

    pub fn range(a: isize, b: isize) -> IntervalSet {
        let intervals = vec![Interval::new(a, b)];
        return IntervalSet {
            intervals: intervals
        }
    }

    pub fn min(&self) -> isize {
        return self.intervals[0].a;
    }

    pub fn max(&self) -> isize {
        return self.intervals[self.intervals.len() - 1].b;
    }

    pub fn length(&self) -> usize {
        return self.intervals.len();
    }

    pub fn clear(&mut self) {
        self.intervals.clear();
    }

    fn pop(&mut self) {
        self.intervals.pop();
    }

    fn back(&self) -> &Interval {
        assert!(!self.intervals.is_empty(), "Cannot access the last interval of an empty set.");
        &self.intervals[self.intervals.len() - 1]
    }

    fn add_interval(&mut self, interval: Interval) {
        if self.intervals.is_empty() {
            self.intervals.push(interval);
        } else {
            if self.back().equals(&interval) {
                return;
            }

            let merged = 
                if self.back().adjacent(&interval) || !self.back().disjoint(&interval) {
                    self.intervals.pop().unwrap().union(&interval)
                } else {
                    interval
                };

            self.intervals.push(merged);
        }
    }

    pub fn add_one(&self, el: isize) -> IntervalSet {
        return self.or(&IntervalSet::one(el));
    }

    pub fn add_range(&self, a: isize, b: isize) -> IntervalSet {
        return self.or(&IntervalSet::range(a, b));
    }

    pub fn add(&self, i: Interval) -> IntervalSet {
        return self.or(&IntervalSet {
            intervals: vec![i]
        })
    }

    pub fn or(&self, other: &IntervalSet) -> IntervalSet {
        let mut res = IntervalSet { intervals: vec![] };

        let mut i = 0;
        let mut j = 0;

        while i < self.length() && j < other.length() {
            let mine = &self.intervals[i];
            let theirs = &other.intervals[j];

            if mine.starts_before(theirs) {
                res.add_interval(mine.clone());
                i = i + 1;
            } else {
                res.add_interval(theirs.clone());
                j = j + 1;
            }
        }

        while i < self.length() {
            res.add_interval(self.intervals[i].clone());
            i = i + 1;
        }

        while j < other.length() {
            res.add_interval(other.intervals[i].clone());
            j = j + 1;
        }

        return res;

    }

    pub fn subtract(&self, other: &IntervalSet) -> IntervalSet {
        let mut res = IntervalSet { intervals: vec![] };

        let mut i = 0;
        let mut j = 0;

        while i < self.length() && j < other.length() {

            let mine = &self.intervals[i];
            let theirs = &other.intervals[j];

            if theirs.b < mine.a {
                j = j + 1;
                continue;
            }

            if theirs.a > mine.b {
                res.add_interval(mine.clone());
                i = i + 1;
                continue;
            }
            
            let before_current = if theirs.a > mine.a { Some(Interval::new(mine.a, theirs.a - 1)) } else { None };
            let after_current = if theirs.b < mine.b { Some(Interval::new(theirs.b + 1, mine.b)) } else { None };

            if before_current.is_some() {
                if after_current.is_some() {
                    res.add_interval(before_current.unwrap());
                    res.add_interval(after_current.unwrap());
                    i = i + 1;
                    j = j + 1;
                    continue;
                }
                else {
                    res.add_interval(before_current.unwrap());
                    i = i + 1;
                    continue;
                }
            }
            else {
                if after_current.is_some() {
                    res.add_interval(after_current.unwrap());
                    j = j + 1;
                    continue;
                }
                else {
                    res.pop();
                    continue;
                }
            }
        }

        while i < self.length() {
            res.add_interval(self.intervals[i].clone());
            i = i + 1;
        }

        return res;
    }

    pub fn and(&self, other: &IntervalSet) -> IntervalSet {
        let mut res = IntervalSet { intervals: vec![] };

        let mut i = 0;
        let mut j = 0;

        while i < self.length() && j < other.length() {
            let mine = &self.intervals[i];
            let theirs = &other.intervals[j];

            if mine.starts_before_disjoint(theirs) {
                i = i + 1;
            } else if theirs.starts_before_disjoint(mine) {
                j = j + 1;
            } else if mine.properly_contains(theirs) {
                res.add_interval(mine.intersection(theirs));
                j = j + 1;
            } else if theirs.properly_contains(mine) {
                res.add_interval(mine.intersection(theirs));
                i = i + 1;
            } else if !mine.disjoint(theirs) {
                res.add_interval(mine.intersection(theirs));
                if mine.starts_after_non_disjoint(theirs) {
                    j = j + 1;
                } else if theirs.starts_after_non_disjoint(mine) {
                    i = i + 1;
                }
            }
        }

        return res;
    }

    pub fn contains(&self, el: isize) -> bool {

        let n = self.intervals.len();
        let mut l = 0;
        let mut r = n - 1;

        while l <= r {
            let m = (l + r) / 2;
            let interval = &self.intervals[m];

            if interval.b < el {
                l = m + 1;
            } else if interval.a > el {
                r = m - 1;
            } else {
                return true;
            }
        }
        return false;
    }

}