use {core::{throws,Error}, ::xy::xy};
struct PagesView{pages:Vec<usvg::Tree>, skip: usize}
impl ui::widget::Widget for PagesView {
	#[throws] fn paint(&mut self, target: &mut ui::widget::Target) {
		let mut x = 0;
		for page in self.pages.iter().skip(self.skip) {
			let page = resvg::render(page, usvg::FitTo::Height(target.size.y), Some(usvg::Color::white())).unwrap();
			#[allow(non_camel_case_types)] #[derive(Clone, Copy, Debug)] pub struct rgba8 { pub r : u8, pub g : u8, pub b : u8, pub a: u8  }
			impl std::convert::From<&rgba8> for image::bgra8 { fn from(&rgba8{r,g,b,a}: &rgba8) -> Self { Self{b,g,r,a} } }
			let page = image::Image{stride:page.width(), size:xy{x:page.width(),y:page.height()}, data: unsafe{core::slice::cast::<rgba8>(page.data())}};
			if x+page.size.x > target.size.x { break; }
			target.slice_mut(xy{x,y:0},page.size).set_map(&page, |_,p| p.into());
			x += page.size.x;
		}
	}
	fn event(&mut self, &ui::widget::Event{key, ..}: &ui::widget::Event) -> bool {
		if key == '⎋' { false }
		else if ['←','⌫'].contains(&key) && self.skip >= 2 { self.skip -= 2; true }
		else if self.skip + 2 < self.pages.len() { self.skip += 2; true }
		else { false }
	}
}
#[throws] fn main() {
	let pages = {let mut v=std::fs::read_dir(".")?.map(|e| e.unwrap().path()).filter(|p| p.extension().filter(|&e| e == "svg").is_some()).collect::<Vec<_>>(); v.sort(); v.into_iter()};
	let pages = pages.map(|path| usvg::Tree::from_file(path, &Default::default()).unwrap()).collect();
	ui::app::run(PagesView{pages, skip:0})?
}
