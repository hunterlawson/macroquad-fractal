#version 100
precision highp float;

// Size of the display in pixels
uniform vec2 display_dimensions;
// Size of the viewport in the complex plane
uniform vec2 viewport_dimensions;
// Reference orbit
uniform vec2 reference_orbit[$size$];
// Reference orbit length
uniform int reference_orbit_length;
uniform int max_iter;

const int MAX_ITER = 1000;

// the output from the vertex shader
varying highp vec2 uv;

vec2 mul_complex(vec2 a, vec2 b) {
    return vec2(a.x * b.x - a.y * b.y, a.y * b.x + a.x * b.y);
}

void main() {
    vec2 ref_p = display_dimensions / 2.;
    vec2 p = uv * display_dimensions;
    vec2 dc = (p - ref_p) * viewport_dimensions;

    // dz(n+1) = 2 * Z(n) * dzn + dzn^2 + dc
    vec2 dzn = vec2(0., 0.);
    int iter = 0;
    for(int i = 0; i < MAX_ITER; i++) {
        if(iter > reference_orbit_length) {
            break;
        }

        vec2 ref_zn = reference_orbit[iter];
        dzn = 2. * mul_complex(ref_zn, dzn) + mul_complex(dzn, dzn) + dc;

        vec2 check = dzn + ref_zn;
        if(check.x * check.x + check.y * check.y > 4.) {
            gl_FragColor = vec4(1., 0., 0., 1.);
            return;
        }

        iter += 1;
    }


    gl_FragColor = vec4(0., 0., 0., 1.);
}
