use gtk::{gdk, graphene, gsk, prelude::*};

mod macros;

pub fn get_circle_paintable_from_color(color: &gdk::RGBA, size: i32) -> gdk::Paintable {
    let snapshot = gtk::Snapshot::new();

    let rect_size_f32 = size as f32;
    let radius = rect_size_f32 / 2.0;

    let graphene_rect = graphene::Rect::new(0.0, 0.0, rect_size_f32, rect_size_f32);
    let rounded_rect = gsk::RoundedRect::from_rect(graphene_rect, radius);

    snapshot.push_rounded_clip(&rounded_rect);

    snapshot.append_color(color, &graphene_rect);

    snapshot.pop();

    let graphene_size = graphene::Size::new(rect_size_f32, rect_size_f32);
    snapshot.to_paintable(Some(&graphene_size)).unwrap() // .unwrap() for simplicity; handle errors in real code
}
