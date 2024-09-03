#version 450 core

layout(location=0)in vec3 in_color;

uniform ivec2 sim_dim;

out vec3 vs_color;

void main(void){
    int x=gl_VertexID%sim_dim.x;
    int y=gl_VertexID/sim_dim.x;
    float x_float=((float(x)/float(sim_dim.x))-.5)*2.;
    float y_float=((float(y)/float(sim_dim.y))-.5)*2.;
    
    gl_Position=vec4(x_float,y_float,-.5,1.);
    vs_color=in_color;
}
