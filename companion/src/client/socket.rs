#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
	Connecting,
	Connected,
	Disconnected,
	Error,
}
