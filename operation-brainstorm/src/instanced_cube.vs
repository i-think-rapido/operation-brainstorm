#version 330

layout (location = 0) in vec3 vertexPosition;

uniform int dimX;
uniform int dimY;
uniform int dimZ;
uniform vec3 spacing;
uniform vec4 cubeColors[256];

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec4 vColor;

void main()
{
    int instanceID = gl_InstanceID;

    int x = instanceID % dimX;
    int y = (instanceID / dimX) % dimY;
    int z = (instanceID / (dimX * dimY)) % dimZ;

    vec3 instanceOffset = vec3(x, y, z) * spacing;

    vec4 worldPosition = model * vec4(vertexPosition + instanceOffset, 1.0);
    gl_Position = projection * view * worldPosition;

    vColor = cubeColors[instanceID];
}
