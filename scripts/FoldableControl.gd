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

func set_is_folded(value: bool):
	self.folded = value

func get_children_size():
	if self.folded:
		return self.folded_version.get_rect().size
	else:
		return self.unfolded_version.get_rect().size

func set_size_from_children():
	self.size = self.get_children_size()

func handle_folded():
	print("Handling folded")
	if self.folded_version != null:
		print("Updating viz")
		self.folded_version.visible = self.folded
	if self.unfolded_version != null:
		self.unfolded_version.visible = !self.folded
	self.folded_changed.emit(self.folded)
