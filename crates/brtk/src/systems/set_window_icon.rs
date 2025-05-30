use std::io::Cursor;

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use winit::window::Icon;

use crate::resources::WindowIcon;

pub fn set_window_icon(
    windows: NonSend<WinitWindows>,
    window_icon: Res<WindowIcon>,
    primary_window: Single<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.into_inner();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };

    let icon_buf = Cursor::new(**window_icon);
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
