#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in vec2 aTexCoord;

out VS_OUTPUT {
    vec4 Color;
} OUT;

out vec2 TexCoord;

void main()
{
    gl_Position = vec4(vec3(Position, 0.0f),1.0f);
    OUT.Color = Color;
    TexCoord = aTexCoord;
}
