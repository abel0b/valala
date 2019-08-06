use crate::scene::NodeId;
use glium::Surface;

pub enum PickingEvent {
    MouseUp(NodeId),
    MouseDown(NodeId),
    HoverEnter(NodeId),
    HoverLeave(NodeId),
}

pub struct Picker {
    entity: Option<NodeId>,
    picking_attachments: Option<(
        glium::texture::UnsignedTexture2d,
        glium::framebuffer::DepthRenderBuffer,
    )>,
    picking_pbo: glium::texture::pixel_buffer::PixelBuffer<u32>,
}

impl Picker {
    pub fn new(display: &glium::Display) -> Picker {
        Picker {
            entity: None,
            picking_attachments: None,
            picking_pbo: glium::texture::pixel_buffer::PixelBuffer::new_empty(display, 1),
        }
    }

    pub fn initialize_picking_attachments(
        &mut self,
        display: &glium::Display,
        (width, height): (u32, u32),
    ) {
        self.picking_attachments = Some((
            glium::texture::UnsignedTexture2d::empty_with_format(
                display,
                glium::texture::UncompressedUintFormat::U32,
                glium::texture::MipmapsOption::NoMipmap,
                width,
                height,
            )
            .unwrap(),
            glium::framebuffer::DepthRenderBuffer::new(
                display,
                glium::texture::DepthFormat::F32,
                width,
                height,
            )
            .unwrap(),
        ))
    }

    pub fn commit(&mut self, cursor_position: Option<(i32, i32)>) {
        if let (Some(cursor), Some(&(ref picking_texture, _))) =
            (cursor_position, self.picking_attachments.as_ref())
        {
            let read_target = glium::Rect {
                left: (cursor.0 - 1) as u32,
                bottom: (picking_texture.get_height().unwrap() as i32
                    - std::cmp::max(cursor.1 + 1, 0)) as u32,
                width: 1,
                height: 1,
            };

            if read_target.left < picking_texture.get_width()
                && read_target.bottom < picking_texture.get_height().unwrap()
            {
                picking_texture
                    .main_level()
                    .first_layer()
                    .into_image(None)
                    .unwrap()
                    .raw_read_to_pixel_buffer(&read_target, &self.picking_pbo);
            } else {
                self.picking_pbo.write(&[0]);
            }
        } else {
            self.picking_pbo.write(&[0]);
        }
    }

    pub fn update(&mut self) -> Vec<PickingEvent> {
        let mut events = Vec::new();
        self.entity = match self.picking_pbo.read().map(|d| d[0]).unwrap_or(0) {
            0 => None,
            id => {
                let node_id = NodeId::Entity(id);
                if let Some(previous_id) = self.entity {
                    if previous_id != node_id {
                        events.push(PickingEvent::HoverLeave(previous_id));
                        events.push(PickingEvent::HoverEnter(node_id));
                    }
                }
                Some(node_id)
            }
        };
        events
    }

    pub fn target(
        &self,
        display: &glium::Display,
    ) -> Option<glium::framebuffer::SimpleFrameBuffer> {
        if let Some((ref picking_texture, ref depth_buffer)) = self.picking_attachments {
            picking_texture
                .main_level()
                .first_layer()
                .into_image(None)
                .unwrap()
                .raw_clear_buffer([0u32, 0, 0, 0]);
            let mut target = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                display,
                picking_texture,
                depth_buffer,
            )
            .unwrap();
            target.clear_depth(1.0);
            Some(target)
        } else {
            None
        }
    }
}
