@tool
extends Control

signal folded_changed(value: bool)

@export var folded: bool :
	set(value):
		folded = value
		self.handle_folded()
		
@export var folded_version: Control
@export var unfolded_version: Control

# Called when the node enters the scene tree for the first time.
func _ready():
	self.handle_folded()

func set_is_folded():
	self.folded = true

func set_is_unfolded():
	self.folded = false

func set_size_from_children():
	if self.folded_version == null or self.unfolded_version == null:
		return
		
	self.custom_minimum_size = self.folded_version.get_rect().size
	if not self.folded:
		var unfolded_size = self.unfolded_version.get_rect().size
		self.custom_minimum_size.y += unfolded_size.y
		self.custom_minimum_size.x = max(self.custom_minimum_size.x, unfolded_size.x)

func handle_folded():
	if self.unfolded_version != null:
		self.unfolded_version.visible = !self.folded
	self.set_size_from_children()
	self.folded_changed.emit(self.folded)
