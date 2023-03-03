void main() {
 float d = 1.0;
 for (float i = 0.0; i < 100.0; i++){
 	vec4 circle = texture2D(Texture, vec2(0, i)/TexSize);
 	d = min(d, ceil(distance(circle.xy, uv)-circle.z));
 }
 gl_FragColor = vec4(vec3(d), 1);
}