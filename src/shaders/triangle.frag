#version 330 core

in VS_OUTPUT {
    vec4 Color;
} IN;

out vec4 Color;

in vec2 TexCoord;

uniform sampler2D ourTexture;

void main()
{
    vec4 FragColor = texture(ourTexture, TexCoord);
    if(FragColor.a + Color[3] < 0.001)
        discard;
    Color = IN.Color;
    Color[0] += FragColor[0];
    Color[1] += FragColor[1];
    Color[2] += FragColor[2];
}
