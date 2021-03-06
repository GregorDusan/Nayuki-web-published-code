/* 
 * Binary array set (Rust)
 * 
 * Copyright (c) 2020 Project Nayuki. (MIT License)
 * https://www.nayuki.io/page/binary-array-set
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 * - The above copyright notice and this permission notice shall be included in
 *   all copies or substantial portions of the Software.
 * - The Software is provided "as is", without warranty of any kind, express or
 *   implied, including but not limited to the warranties of merchantability,
 *   fitness for a particular purpose and noninfringement. In no event shall the
 *   authors or copyright holders be liable for any claim, damages or other
 *   liability, whether in an action of contract, tort or otherwise, arising from,
 *   out of or in connection with the Software or the use or other dealings in the
 *   Software.
 */

use std;


#[derive(Clone,Default)]
pub struct BinaryArraySet<E> {
	
	// Each values[i]'s length is either 0 or 2^i, with elements in ascending order
	values: Vec<Vec<E>>,
	
	size: usize,
	
}


impl<E: std::cmp::Ord> BinaryArraySet<E> {
	
	pub fn new() -> Self {
		Self {
			values: Vec::new(),
			size: 0,
		}
	}
	
	
	// Runs in O(1) time
	pub fn is_empty(&self) -> bool {
		self.size == 0
	}
	
	
	// Runs in O(1) time
	pub fn len(&self) -> usize {
		self.size
	}
	
	
	pub fn clear(&mut self) {
		self.values.clear();
		self.size = 0;
	}
	
	
	// Runs in O((log n)^2) time
	pub fn contains(&self, val: &E) -> bool {
		self.values.iter().any(
			|vals| vals.binary_search(val).is_ok())
	}
	
	
	// Runs in average-case O((log n)^2) time, worst-case O(n) time
	pub fn insert(&mut self, val: E) -> bool {
		let result = !self.contains(&val);  // Checking for duplicates is expensive
		if result {
			self.insert_unique(val);
		}
		result
	}
	
	
	// Runs in amortized O(1) time, worst-case O(n) time
	pub fn insert_unique(&mut self, val: E) {
		assert!(self.size < std::usize::MAX, "Maximum size reached");
		self.size += 1;
		let mut toput: Vec<E> = vec![val];
		for vals in &mut self.values {
			if vals.is_empty() {
				*vals = toput;
				return;
			}
			
			// Merge two sorted arrays
			assert_eq!(vals.len(), toput.len());
			assert!(vals.len() <= std::usize::MAX / 2);
			toput = BinaryArraySet::merge_vecs(vals, toput);
		}
		self.values.push(toput);
	}
	
	
	pub fn check_structure(&self) {
		let mut sum: usize = 0;
		for (i, vals) in self.values.iter().enumerate() {
			let len = vals.len();
			assert!(len == 0 || len == 1 << i, "Invalid sub-vector length");
			for j in 1 .. len {
				assert!(vals[j - 1] < vals[j], "Invalid ordering of elements in vector");
			}
			sum += len;
		}
		assert_eq!(sum, self.size, "Size mismatch between counter and sub-vectors");
	}
	
	
	// (Private) Assuming that xs and ys are both in ascending order, this
	// moves all their elements into a new sorted vector zs and returns it.
	fn merge_vecs(xs: &mut Vec<E>, ys: Vec<E>) -> Vec<E> {
		let mut result = Vec::<E>::with_capacity(xs.len() + ys.len());
		let mut xiter = xs.drain(..);
		let mut yiter = ys.into_iter();
		let mut xnext = xiter.next();
		let mut ynext = yiter.next();
		loop {
			let takex: bool = match (xnext.as_ref(), ynext.as_ref()) {
				(None, None) => break,
				(_, None) => true,
				(None, _) => false,
				(Some(x), Some(y)) => *x <= *y,
			};
			if takex {
				result.push(xnext.unwrap());
				xnext = xiter.next();
			} else {
				result.push(ynext.unwrap());
				ynext = yiter.next();
			}
		}
		result
	}
	
}



/*---- Helper struct ----*/

impl<'a, E> IntoIterator for &'a BinaryArraySet<E> {
	type Item = &'a E;
	type IntoIter = Iter<'a, E>;
	
	fn into_iter(self) -> Self::IntoIter {
		Iter::<E>::new(&self)
	}
}


#[derive(Clone)]
pub struct Iter<'a, E:'a> {
	values: &'a Vec<Vec<E>>,
	index: usize,
	subindex: usize,
	count: usize,
}


impl<'a, E> Iter<'a, E> {
	// Runs in O(log n) time
	fn new(set: &'a BinaryArraySet<E>) -> Self {
		Self {
			values: &set.values,
			index: set.values.iter().position(|x| !x.is_empty())
				.unwrap_or(set.values.len()),
			subindex: 0,
			count: set.size,
		}
	}
}


impl<'a, E> Iterator for Iter<'a, E> {
	type Item = &'a E;
	
	// Runs in amortized O(1) time, worst-case O(log n) time
	fn next(&mut self) -> Option<Self::Item> {
		if self.index >= self.values.len() {
			return None;
		}
		let result: &E = &self.values[self.index][self.subindex];
		self.subindex += 1;
		while self.index < self.values.len() && self.subindex >= self.values[self.index].len() {
			self.subindex = 0;
			self.index += 1;
		}
		self.count -= 1;
		Some(result)
	}
	
	
	fn size_hint(&self) -> (usize,Option<usize>) {
		(self.count, Some(self.count))
	}
	
	fn count(self) -> usize {
		self.count
	}
	
}
