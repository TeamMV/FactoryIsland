#version 450

layout (location = 0) in vec2 fUv;
layout (location = 0) out vec4 outColor;

uniform sampler2D COLOR;
uniform vec2 RES;

uniform float TILE_SIZE;
uniform vec2 CAM;
uniform sampler2D OFFSETS;

void main() {
    vec2 worldPos = CAM + fUv * RES;

    // Base UV for offset map, tile-aligned and repeatable
    vec2 offsetUV = worldPos / TILE_SIZE;
    vec2 offsetSample = texture(OFFSETS, offsetUV).rg * 2.0 - 1.0; // [-1..1]

    vec2 uvOff = offsetSample / (RES) + fUv;

    outColor = texture(COLOR, uvOff);
}
