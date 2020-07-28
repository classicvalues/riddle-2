use crate::{math::*, *};

use std::rc::Rc;

pub struct SpriteAtlasBuilder<'a> {
    images: Vec<(image::Image, &'a mut Option<Sprite>)>,

    max_height: u32,
    total_width: u32,
}

impl<'a> SpriteAtlasBuilder<'a> {
    pub fn new() -> Self {
        Self {
            images: vec![],
            max_height: 0,
            total_width: 0,
        }
    }

    pub fn with_image(mut self, img: image::Image, sprite: &'a mut Option<Sprite>) -> Self {
        self.total_width += img.width();
        self.max_height = self.max_height.max(img.height());
        self.images.push((img, sprite));
        self
    }

    pub fn build(self, renderer: &Rc<Renderer>) -> Result<(), RendererError> {
        let mut atlas = image::Image::new(self.total_width, self.max_height)?;
        let mut sprite_bounds = vec![];
        let mut x = 0;
        for (img, sprite) in self.images {
            sprite_bounds.push((
                Rect {
                    location: Vector2 { x: x, y: 0 },
                    dimensions: img.dimensions(),
                },
                sprite,
            ));
            atlas.blit(&img, Vector2 { x, y: 0 }.convert());
            x += img.width();
        }

        let texture = Rc::new(Texture::from_image(
            &renderer.device,
            &renderer.queue,
            atlas,
        )?);

        for (bounds, sprite) in sprite_bounds {
            *sprite = Some(Sprite::from_texture_with_bounds(
                renderer,
                texture.clone(),
                bounds.convert(),
            ));
        }

        Ok(())
    }
}
