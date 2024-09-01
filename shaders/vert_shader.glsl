
layout(location=0)in vec3 aPos;

layout(std430, binding = 0) buffer color_buffer {
    vec3 colors[];
};

uniform mat4 viewProjection;

void main()
{
    gl_Position=viewProjection*vec4(aPos.x,aPos.y,aPos.z,1.);
}
