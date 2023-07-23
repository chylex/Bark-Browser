use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use parser::KeySequenceParser;

use crate::input::keymap::parser::ParseError;

pub use self::binding::KeyBinding;

mod binding;
mod parser;

pub struct KeyMap<V> {
	keybinds: HashMap<KeyBinding, KeyMapTrieNode<V>>,
}

impl<V> KeyMap<V> {
	pub fn new() -> Self {
		Self { keybinds: HashMap::new() }
	}
	
	fn insert_sequence(&mut self, key_sequence: &[KeyBinding], value: V) -> Result<(), KeyMapInsertErrorType> {
		let mut map = self;
		let mut iter = key_sequence.iter().peekable();
		
		while let Some(key) = iter.next() {
			if iter.peek().is_none() {
				map.keybinds.insert(*key, KeyMapTrieNode::Leaf(value));
				return Ok(());
			} else if let KeyMapTrieNode::SubTree(ref mut nested) = map.keybinds.entry(*key).or_insert_with(|| KeyMapTrieNode::SubTree(Self::new())) {
				map = nested;
			} else {
				return Err(KeyMapInsertErrorType::ConflictingKeySequence);
			}
		}
		
		Err(KeyMapInsertErrorType::EmptyKeySequence)
	}
	
	pub fn insert(&mut self, key_sequence_str: &str, value: V) -> Result<(), KeyMapInsertError> {
		let mut parser = KeySequenceParser::new(key_sequence_str);
		let mut sequence = Vec::new();
		
		while let Some(key) = parser.next().map_err(|err| KeyMapInsertError::new(key_sequence_str.to_owned(), KeyMapInsertErrorType::ParseError(err)))? {
			sequence.push(key);
		}
		
		self.insert_sequence(&sequence, value).map_err(|err| KeyMapInsertError::new(key_sequence_str.to_owned(), err))
	}
	
	pub fn lookup(&self, key_sequence: &[KeyBinding]) -> KeyMapLookupResult<&V> {
		let mut map = self;
		let mut iter = key_sequence.iter().peekable();
		
		while let Some(node) = iter.next().and_then(|key| map.keybinds.get(key)) {
			if iter.peek().is_none() {
				return if let KeyMapTrieNode::Leaf(value) = node {
					KeyMapLookupResult::Found(value)
				} else {
					KeyMapLookupResult::Prefix
				}
			} else if let KeyMapTrieNode::SubTree(nested) = node {
				map = nested;
			}
		}
		
		KeyMapLookupResult::None
	}
}

enum KeyMapTrieNode<V> {
	SubTree(KeyMap<V>),
	Leaf(V),
}

pub enum KeyMapLookupResult<V> {
	None,
	Prefix,
	Found(V),
}

#[derive(Debug, Clone)]
pub struct KeyMapInsertError {
	sequence: String,
	error: KeyMapInsertErrorType,
}

impl KeyMapInsertError {
	const fn new(sequence: String, error: KeyMapInsertErrorType) -> Self {
		Self { sequence, error }
	}
	
	pub fn sequence(&self) -> &str {
		&self.sequence
	}
	
	pub const fn error(&self) -> &KeyMapInsertErrorType {
		&self.error
	}
}

#[derive(Debug, Clone)]
pub enum KeyMapInsertErrorType {
	EmptyKeySequence,
	ConflictingKeySequence,
	ParseError(ParseError),
}

impl Display for KeyMapInsertErrorType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::EmptyKeySequence => write!(f, "Empty key sequence."),
			Self::ConflictingKeySequence => write!(f, "Conflicting key sequence."),
			Self::ParseError(err) => write!(f, "Parse error: {err}"),
		}
	}
}
