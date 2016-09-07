extern crate rust_controllers;
use rust_controllers::graphics::shaders;
use rust_controllers::graphics::Vertex;

extern crate cgmath;
#[macro_use]
extern crate glium;
// extern crate glium_text;
// extern crate rand;

use std::{thread, time};
// use rand::Rng;

fn main() {

    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Hello world")
        .build_glium()
        .unwrap();
    let shader_prog = glium::Program::from_source(&display,
                                                  shaders::VERT_SHADER_SRC,
                                                  shaders::FRAG_SHADER_SRC,
                                                  None)
        .unwrap();
    let mut draw_params: glium::DrawParameters = Default::default();
    draw_params.point_size = Some(10f32);

    let ten_millis = time::Duration::from_millis(10);

    const MAX_NUM_VERTS: usize = 10;
    let mut points: Vec<Vertex> = Vec::with_capacity(MAX_NUM_VERTS);
    // fill with dummy points to get correct VBO size
    points.resize(MAX_NUM_VERTS, Vertex { position: [-1.0, -1.0] });
    let mut vert_buffer = glium::VertexBuffer::dynamic(&display, &points).unwrap();
    let mut next_vert_idx = 0;
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    // let txt_sys = glium_text::TextSystem::new(&display);
    // let font = glium_text::FontTexture::new(&display, &include_bytes!("FreeMono.ttf")[..], 70)
    //     .unwrap();
    // let text = glium_text::TextDisplay::new(&txt_sys, &font, "1.0");

    let mut time_from_start: f32 = 0.0;
    let mut counter = 0;

    loop {
        time_from_start += 0.01;
        counter += 1;
        if counter == 100 {
            counter = 0;
            let mut vert_map = vert_buffer.map_write();
            vert_map.set(next_vert_idx,
                         Vertex { position: [10.0 + time_from_start, 5.0] });
            next_vert_idx = (next_vert_idx + 1) % MAX_NUM_VERTS;  // ring buffer
            println!("writing to vert buffer");
        }

        // let (w, h) = display.get_framebuffer_dimensions();
        let sliding_projection: [[f32; 4]; 4] = cgmath::ortho(0f32 + time_from_start,
                                                              10. + time_from_start,
                                                              0.,
                                                              10.,
                                                              -1.,
                                                              1.)
            .into();
        // let projection_mat: [[f32; 4]; 4] = projection.into();

        let mut frame = display.draw();
        frame.clear_color(0.9, 0.9, 0.9, 1.0);
        frame.draw(&vert_buffer,
                  &indices,
                  &shader_prog,
                  &uniform!{Pmat: sliding_projection},
                  &draw_params)
            .unwrap();
        frame.finish().unwrap();


        // let text_projection = cgmath::ortho(0f32 + t, 10. + t, 0., 10., -1., 1.);
        // let text_width = text.get_width();
        // let em = 0.5f32;
        // let bottom_left = [5.0, 0.5];

        // // treated as column-major
        // let text_pos: [[f32; 4]; 4] = (text_projection *
        //                                cgmath::Matrix4::new(em / text_width,
        //                                                     0.0,
        //                                                     0.0,
        //                                                     0.0, //
        //                                                     0.0,
        //                                                     em * (w as f32) / (h as f32) /
        //                                                     text_width,
        //                                                     // em / text_width,
        //                                                     0.0,
        //                                                     0.0, //
        //                                                     0.0,
        //                                                     0.0,
        //                                                     1.0,
        //                                                     0.0, //
        //                                                     bottom_left[0],
        //                                                     bottom_left[1],
        //                                                     0.0,
        //                                                     1.0))
        //     .into();
        // glium_text::draw(&text, &txt_sys, &mut frame, text_pos, (0.0, 0.0, 0.0, 1.0));


        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::ReceivedCharacter('q') => return,
                _ => (),
            }
        }
        thread::sleep(ten_millis);
    }

}
