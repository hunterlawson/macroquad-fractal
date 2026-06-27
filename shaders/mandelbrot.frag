#version 100
precision highp float;

uniform float base_re;
uniform float scale_re;
uniform float base_im;
uniform float scale_im;
uniform vec2 pixel_dimensions;

uniform int max_iter;

const int MAX_ITER = 1000;

// the output from the vertex shader
varying highp vec2 uv;

float fractal_value(vec2 c) {
    vec2 z = c;
    float z_x2 = z.x * z.x;
    float z_y2 = z.y * z.y;
    float mag_2 = z_x2 + z_y2;

    for(int i = 0; i < MAX_ITER; i++) {
        if (i >= max_iter ) break;
        if (mag_2 > 256.) {
            float lz = log2(mag_2) * 0.5;
            float smooth_i = float(i) + 1. - log2(lz);
            float t = 0.5 + 0.5 * cos(smooth_i * 0.5);
            return t;
        }
        z = vec2(z_x2 - z_y2, 2. * z.x * z.y) + c;
        z_x2 = z.x * z.x;
        z_y2 = z.y * z.y;
        mag_2 = z_x2 + z_y2;
    }

    return 0.;
}

vec2 c_for(vec2 offset_px) {
    vec2 sub_uv = offset_px / pixel_dimensions;
    return vec2(base_re, base_im) + (uv + sub_uv) * vec2(scale_re, scale_im);
}

void main() {
    float val = 0.;
    // 4x supersampling
    vec2 offsets[4];
    offsets[0] = vec2(-0.25, -0.25);
    offsets[1] = vec2(-0.25,  0.25);
    offsets[2] = vec2( 0.25,  0.25);
    offsets[3] = vec2( 0.25, -0.25);

    for(int s = 0; s < 4; s++) {
        vec2 c = c_for(offsets[s]);
        val += fractal_value(c);
    }

    val /= 4.;

    gl_FragColor = vec4(val, 0., 0., 1.);
}
