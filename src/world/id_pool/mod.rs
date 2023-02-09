use std::io::{ Error, ErrorKind };
use std::cmp;

pub struct IDPool {
	current_id: usize,
	used_ids: Vec<usize>,
	available_ids: Vec<usize>
}

impl IDPool {
	pub fn new() -> Self {
		IDPool {
			current_id: usize::MIN,
			used_ids: Vec::<usize>::new(),
			available_ids: Vec::<usize>::new()
		}
	}

	pub fn reserve_entity_id(&mut self) -> Result<usize, Box<dyn std::error::Error>> {
		if self.available_ids.len() > 0 {
			let id = self.available_ids.pop().unwrap();
			self.used_ids.push(id.clone());
			return Ok(id)
		}

		if self.current_id == usize::MAX {
			return Err(Box::new(Error::new(ErrorKind::Other, "Maximum entity limit reached")))
		}
		
		self.current_id = self.current_id + 1;
		self.used_ids.push(self.current_id.clone());
		Ok(self.current_id.clone())
	}

	pub fn dealloc_id(&mut self, id: usize) {
		if let Some(index) = self.used_ids.iter().position(|used_id| used_id == &id) {
			self.used_ids.remove(index);
			self.available_ids.push(id.clone());
		}
	}

	pub fn housekeeping(&mut self) {
		let max_available = match self.available_ids.iter().max() {
			Some(res) => res.clone(),
			None => usize::MIN
		};

		let max_used = match self.used_ids.iter().max() {
			Some(res) => res.clone(),
			None => usize::MIN
		};

		let max = cmp::max(max_available, max_used);

		if self.current_id > max {
			self.current_id = max;
		};

		self.available_ids = self.available_ids.clone().into_iter().filter(|a| a <= &self.current_id).collect::<Vec<usize>>();
	}
}