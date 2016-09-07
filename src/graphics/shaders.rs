pub static VERT_SHADER_SRC: &'static str = r#"
    #version 330

    in vec2 position;
    uniform mat4 Pmat;

    void main() {
        gl_Position = Pmat * vec4(position, 0.0, 1.0);
    }
    "#;

pub static FRAG_SHADER_SRC: &'static str = r#"
    #version 330

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "#;
