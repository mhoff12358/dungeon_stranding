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

var skip_layout = true
func _ready():
	skip_layout = false
	self.request_layout.call_deferred()

func calc_values(size_vec:Vector2, proportions:Vector2):
	return size_vec.dot(proportions)

func request_layout():
	if skip_layout:
		return

	if reference_node:
		var left = int(calc_values(reference_node.size, left_margin))
		var right = int(calc_values(reference_node.size, left_margin + width))
		var top = int(calc_values(reference_node.size, top_margin))
		var bottom = int(calc_values(reference_node.size, top_margin + height))
		var x = left
		var y = top
		var x2 = reference_node.size.x - right
		var y2 = reference_node.size.y - bottom
		var w = x2 - x
		var h = y2 - y
		if w < 0: w = 0
		if h < 0: h = 0
		self.position = Vector2(x, y)
		self.size = Vector2(w, h)
	else:
		var parent_size = self.get_parent().size
		var x = floor(calc_values(parent_size, left_margin))
		var y = floor(calc_values(parent_size, top_margin))
		var x2 = floor(calc_values(parent_size, left_margin + width))
		var y2 = floor(calc_values(parent_size, top_margin + height))
		var w = x2 - x
		var h = y2 - y
		if w < 0: w = 0
		if h < 0: h = 0
		self.position = Vector2(x, y)
		self.size = Vector2(w, h)
