@group(0) @binding(0)
var<uniform> window_size: vec2<f32>;

@group(1) @binding(0)
var image_texture: texture_2d<f32>;
@group(1) @binding(1)
var image_sampler: sampler;

struct VertexOutput{
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec4<f32>,
	@location(1) uv: vec2<f32>
}

struct VertexInput{
	@location(0) position: vec2<f32>,
	@location(1) color:vec4<f32>,
	@location(2) uv:vec2<f32>
}

// Convert screen space coordinates to normalised device coordinates
fn screen_to_ndc(in:vec2<f32>) -> vec2<f32>{
	return vec2<f32>(
		(in.x / window_size.x) * 2.0 - 1.0, // Scale by 2 and translate by -1
		-((in.y / window_size.y) * 2.0 - 1.0),
	);
}

@vertex
fn vs_main(in:VertexInput) -> VertexOutput {
	var out: VertexOutput;
	
	var pos = screen_to_ndc(in.position);
	
	out.color = in.color;
	out.uv = in.uv;
	out.position = vec4<f32>(pos,1.0,1.0);
	return out;
}

@fragment
fn fs_main(in:VertexOutput) -> @location(0) vec4<f32> {
	var texture_color:vec4<f32> = textureSample(image_texture,image_sampler,in.uv); 
	
	// Mix between the icon color from the svg and the color of the vertex, works best with
	// black icons 
	// FIXME this means the input color alpha won't work properly
	var icon_color:vec3<f32> = mix(
		texture_color.xyz,
		in.color.xyz,
		1.0
	);

	return vec4(icon_color,texture_color.w);
}