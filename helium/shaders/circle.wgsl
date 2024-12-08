// Window attributes
@group(0) @binding(0) var<uniform> window_size: vec2<f32>;

// Widget attributes
@group(1) @binding(0) var<uniform> size: vec2<f32>;
@group(1) @binding(1) var<uniform> position: vec2<f32>;

struct VertexOutput{
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec4<f32>,
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

fn sd_circle(p:vec2<f32>,r:f32 ) -> f32 {
    return length(p)-r;
}


@vertex
fn vs_main(in:VertexInput) -> VertexOutput {
	var out: VertexOutput;
	
	// Normalize the coordinates, translate by 1 to the left and scale by 2 to cover the whole screen
	var position = (in.position/window_size) * 0.5 + 1;
	out.position = vec4<f32>(screen_to_ndc(in.position),1.0,1.0);
	out.color = in.color;
	return out;
}

@fragment
fn fs_main(in:VertexOutput) -> @location(0) vec4<f32> {
	var aspect_ratio = window_size.x/window_size.y;
	var center = (position + size/2);
	var p = (in.position.xy - center)/size;
	// Widget coordinates start at the top left, so center them
	var d = sd_circle(p,0.5);
    return vec4(d,d,d,1.0);
    //return in.color;
}