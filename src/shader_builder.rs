use std::fs::read_to_string;

#[derive(Default)]
pub struct ShaderBuilder {
	shader: String,
	path: String,
}

impl ShaderBuilder {
	pub fn init(path: &str) -> ShaderBuilder {
		// Set the minimally required fields of Foo.
		ShaderBuilder {
			shader: "".to_string(),
			path: path.to_string(),
		}
	}

	pub fn add(mut self, file_name: &str) -> ShaderBuilder {
		self.shader = [
			&self.shader,
			read_to_string(&[&self.path, file_name, ".glsl"].join(""))
				.unwrap()
				.as_str(),
		]
		.join("");
		self
	}

	pub fn build(self) -> String {
		self.shader
	}
}
