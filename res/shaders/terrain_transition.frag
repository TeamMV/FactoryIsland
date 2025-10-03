#version 450

layout (location = 0) in vec2 fUv;
layout (location = 0) out vec4 outColor;

uniform sampler2D COLOR;
uniform vec2 RES;

uniform float TILE_SIZE;
uniform vec2 CAM;
uniform sampler2D OFFSETS;
uniform float FRAME;

void main() {
    vec2 cam = CAM;
    //okay so basically the same offset is applied on the cpu side lmao nah bro this is so stupid
    cam -= 0.25 * 0.5 * TILE_SIZE;
    vec2 worldPos = cam + fUv * RES;

    // Base UV for offset map, tile-aligned and repeatable
    vec2 offsetUV = fract(worldPos / TILE_SIZE);

    // the colors of OFFSETS specify the uv shift at a certain pixel positions.
    // the following table shows how colors map to offsets in pixels:
    // 0.0 -> -TILE_SIZE
    // 0.5 -> 0
    // 1.0 -> TILE_SIZE
    vec2 offset = texture(OFFSETS, offsetUV).rg * 2.0 - 1.0; // [-1..1]
    offset = (offset * TILE_SIZE) / RES;

    vec2 uvOff = offset + fUv;
    uvOff = clamp(uvOff, vec2(0.0), vec2(1.0));

    outColor = texture(COLOR, uvOff);
}
