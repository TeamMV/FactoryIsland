#version 450

layout (location = 0) in vec2 fUv;

layout(location = 0) out vec4 outColor;

uniform sampler2D COLOR;
uniform sampler2D DEPTH;
uniform vec2 RES; // Screen resolution

void main() {
    // Slight wavy distortion based on resolution
    float waveStrength = 0.01 * (RES.x / 1920.0);  // Adjust wave strength based on screen width
    float waveFrequency = 10.0 * (RES.y / 1080.0);  // Adjust wave frequency based on screen height

    // Apply a sine wave distortion to the UVs
    vec2 wavyUv = fUv + vec2(
    sin(fUv.y * waveFrequency) * waveStrength,  // Horizontal distortion based on vertical UV
    cos(fUv.x * waveFrequency) * waveStrength   // Vertical distortion based on horizontal UV
    );

    // Fetch the texture color with the distorted UVs
    vec4 color = texture(COLOR, wavyUv);

    // Apply inversion
    color = vec4(1.0 - color.r, 1.0 - color.g, 1.0 - color.b, 1.0);

    // Output the final color
    outColor = color;
}
