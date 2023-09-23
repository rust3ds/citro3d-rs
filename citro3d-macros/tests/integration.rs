use citro3d_macros::include_shader;

#[test]
fn includes_shader_static() {
    static SHADER_BYTES: &[u8] = include_shader!("test.pica");

    assert_eq!(SHADER_BYTES.len() % 4, 0);
}

#[test]
fn includes_shader_const() {
    const SHADER_BYTES: &[u8] = include_shader!("test.pica");

    assert_eq!(SHADER_BYTES.len() % 4, 0);
}
