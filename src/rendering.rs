use mvengine::rendering::post::OpenGLPostProcessShader;

pub struct WorldShaders {
    pub ssao: OpenGLPostProcessShader,
    pub clouds: OpenGLPostProcessShader
}

impl WorldShaders {
    pub fn new() -> Self {
        let mut this = Self {
            ssao: OpenGLPostProcessShader::new(include_str!("../res/shaders/ssao.frag")),
            clouds: OpenGLPostProcessShader::new(include_str!("../res/shaders/clouds.frag")),
        };
        this.ssao.make();
        this.ssao.bind();
        
        this.clouds.make();
        this.clouds.bind();
        
        this
    }
}