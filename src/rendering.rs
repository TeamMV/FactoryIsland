use mvengine::rendering::post::{OpenGLPostProcessShader, OpenGlBlendShader};

pub struct WorldShaders {
    pub ssao: OpenGLPostProcessShader,
    pub trns: OpenGLPostProcessShader,
    pub clouds: OpenGLPostProcessShader,
    pub overlay: OpenGLPostProcessShader,
    pub overlay_blend: OpenGlBlendShader,
}

impl WorldShaders {
    pub fn new() -> Self {
        let mut this = Self {
            ssao: OpenGLPostProcessShader::new(include_str!("../res/shaders/ssao.frag")),
            trns: OpenGLPostProcessShader::new(include_str!("../res/shaders/terrain_transition.frag")),
            clouds: OpenGLPostProcessShader::new(include_str!("../res/shaders/clouds.frag")),
            overlay: OpenGLPostProcessShader::new(include_str!("../res/shaders/overlay.frag")),
            overlay_blend: OpenGlBlendShader::new(include_str!(
                "../res/shaders/overlay_blend.frag"
            )),
        };
        this.ssao.make();
        this.ssao.bind();

        this.trns.make();
        this.trns.bind();

        this.clouds.make();
        this.clouds.bind();

        this.overlay.make();
        this.overlay.bind();

        this.overlay_blend.make();
        this.overlay_blend.bind();

        this
    }
}
