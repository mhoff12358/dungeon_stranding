@tool

class_name RelativelyPlaced
extends Control

@export var left_margin:Vector2 = Vector2(0.0, 0.0):
	set(value):
		if left_margin == value: return
		left_margin = value
		request_layout()

@export var top_margin:Vector2 = Vector2(0.0, 0.0):
	set(value):
		if top_margin == value: return
		top_margin = value
		request_layout()

@export var width:Vector2 = Vector2(1.0, 0.0):
	set(value):
		if width == value: return
		width = value
		request_layout()

@export var height:Vector2 = Vector2(0.0, 1.0):
	set(value):
		if height == value: return
		height = value
		request_layout()

@export var reference_node: Control:
	set(value):
		if reference_node == value: return
		reference_node = value
		request_layout()

@export var position_at_relative: bool:
	set(value):
		if position_at_relative == value: return
		position_at_relative = value
		request_layout()

var skip_layout = true
func _ready():
	skip_layout = false
	self.request_layout.call_deferred()

	if reference_node:
		reference_node.resized.connect(self.request_layout)
	else:
		self.get_parent().resized.connect(self.request_layout)

func calc_values(size_vec:Vector2, proportions:Vector2):
	return size_vec.dot(proportions)

func request_layout():
	if skip_layout:
		return

	var relative_size = self.get_parent().size
	if reference_node:
		relative_size = reference_node.size

	var x = floor(calc_values(relative_size, left_margin))
	var y = floor(calc_values(relative_size, top_margin))
	var x2 = floor(calc_values(relative_size, left_margin + width))
	var y2 = floor(calc_values(relative_size, top_margin + height))
	var w = x2 - x
	var h = y2 - y
	if w < 0: w = 0
	if h < 0: h = 0
	self.size = Vector2(w, h)
	if reference_node != null && self.position_at_relative:
		self.global_position = reference_node.global_position
	else:
		self.position = Vector2(x, y)
