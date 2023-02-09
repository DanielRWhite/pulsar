pub trait Component {
	fn get_type(&self) -> String {
		std::any::type_name::<Self>().to_string().to_lowercase()
	}
}