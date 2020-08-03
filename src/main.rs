use {core::{throws,Error}, ::xy::xy};
struct Pages(Vec<usvg::Tree>);
impl ui::widget::Widget for Pages {
	#[throws] fn paint(&mut self, target: &mut ui::widget::Target) {
		println!("{}", self.0.len());
		for (i, page) in self.0.iter().enumerate() {
			let page = resvg::render(page, usvg::FitTo::Width(target.size.x/2), None).unwrap();
			#[allow(non_camel_case_types)] #[derive(Clone, Copy, Debug)] pub struct rgba8 { pub r : u8, pub g : u8, pub b : u8, pub a: u8  }
			impl std::convert::From<&rgba8> for ui::bgra8 { fn from(&rgba8{r,g,b,a}: &rgba8) -> Self { Self{b,g,r,a} } }
			let page = ui::Image{stride:page.width(), size:xy{x:page.width(),y:page.height()}, data: unsafe{core::slice::cast::<rgba8>(page.data())}};
			let page = page.slice(xy{x:0,y:(page.size.y-target.size.y)/2},xy{x:target.size.x/2,y:target.size.y});
			let mut target = target.slice_mut(xy{x:i as u32*target.size.x/2,y:0},xy{x:target.size.x/2,y:target.size.y});
			target.set_map(&page, |_,p| p.into());
		}
	}
}
#[throws] fn main() {
	let pages = std::fs::read_dir("data")?.map(|e| e.unwrap().path()).filter(|p| p.extension().filter(|&e| e == "svg").is_some());
	let pages = pages.map(|path| usvg::Tree::from_file(path, &Default::default()).unwrap()).collect();
	ui::app::run(Pages(pages))?
}
