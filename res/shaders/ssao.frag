#version 450

layout (location = 0) in vec2 fUv;
layout (location = 0) out vec4 outColor;

uniform sampler2D COLOR;
uniform sampler2D DEPTH;
uniform vec2 RES;

const float OFFSET = 10.0;
const float FACTOR = 20.0;

void main() {
    vec2 texelSize = 1.0 / RES;
    float depth = texture(DEPTH, fUv).r;

    float offDepths = 0.0;
    int sampleCount = 0;

    for (float x = -OFFSET; x <= OFFSET; x += 1.0) {
        for (float y = -OFFSET; y <= OFFSET; y += 1.0) {
            vec2 newUv = fUv + vec2(x, y) * texelSize;
            offDepths += texture(DEPTH, newUv).r;
            sampleCount++;
        }
    }
    
    offDepths /= float(sampleCount);
    float occlusion = 1.0 - clamp((depth - offDepths), 0.0, 1.0) * FACTOR;

    vec4 color = texture(COLOR, fUv);
    outColor = vec4(color.rgb * occlusion, 1.0);
}
