#version 330 core
in vec2 TexCoord;

uniform sampler2D ourTexture;

out vec4 FragColor;

void main()
{
  FragColor = texture(ourTexture, TexCoord) * 0.01 + vec4(1.0, 0.0, 0.0, 1.0);
}