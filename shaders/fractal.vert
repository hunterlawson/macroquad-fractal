#version 100

attribute vec3 position; // vertex position in screen space
attribute vec2 texcoord; // the corner's UV (0,0) top left -> (1,1) bottom right

// draw_rectangle gives screen-space coordinates 
// x : (0, screen_width_in_pixels), y : (0, screen_height_in_pixels)
// The GPU's clip space is (-1, 1)
// Projection * Model converts from pixels -> clip space
uniform mat4 Model;
uniform mat4 Projection;

// Output to the fragment shader

varying highp vec2 uv;

void main() {
    // convert the position to a homogeneous 4D vector
    // w = 1.0 indicates a position, not a direction
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}