use gtk::{gdk, graphene, gsk, prelude::*};

mod macros;

// TODO: Is the size really useful?
pub fn get_circle_paintable_from_color(color: &gdk::RGBA, size: f32) -> gdk::Paintable {
    let snapshot = gtk::Snapshot::new();

    let radius = size / 2.0;

    let graphene_rect = graphene::Rect::new(0.0, 0.0, size, size);
    let rounded_rect = gsk::RoundedRect::from_rect(graphene_rect, radius);

    snapshot.push_rounded_clip(&rounded_rect);

    snapshot.append_color(color, &graphene_rect);

    snapshot.pop();

    let graphene_size = graphene::Size::new(size, size);
    snapshot.to_paintable(Some(&graphene_size)).unwrap()
}
