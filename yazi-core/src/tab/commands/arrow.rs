use yazi_shared::{event::Exec, render};

use crate::{manager::Manager, tab::Tab, Step};

pub struct Opt {
	step: Step,
}

impl From<&Exec> for Opt {
	fn from(e: &Exec) -> Self {
		Self { step: e.args.first().and_then(|s| s.parse().ok()).unwrap_or_default() }
	}
}

impl<T> From<T> for Opt
where
	T: Into<Step>,
{
	fn from(t: T) -> Self { Self { step: t.into() } }
}

impl Tab {
	pub fn arrow(&mut self, opt: impl Into<Opt>) {
		let opt = opt.into() as Opt;
		if !self.current.arrow(opt.step) {
			return;
		}

		// Visual selection
		if let Some((start, items)) = self.mode.visual_mut() {
			let after = self.current.cursor;

			items.clear();
			for i in start.min(after)..=after.max(start) {
				items.insert(i);
			}
		}

		Manager::_hover(None);
		render!();
	}
}
