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
	var child = null
	if self.folded:
		child = self.folded_version
	else:
		child = self.unfolded_version
	if child:
		self.custom_minimum_size = child.get_rect().size

func handle_folded():
	print("Handling folded")
	if self.folded_version != null:
		print("Updating viz")
		self.folded_version.visible = self.folded
	if self.unfolded_version != null:
		self.unfolded_version.visible = !self.folded
	self.folded_changed.emit(self.folded)
	self.set_size_from_children()
