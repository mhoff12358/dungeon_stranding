@tool

class_name RelativelyPlaced
extends Control

enum Mode { SIZE, MARGINS }

@export var mode:Mode = Mode.SIZE:
	set(value):
		if self.skip_setter:
			mode = value
			return
		if mode == value: return
		unify_properties()
		mode = value
		notify_property_list_changed()
		request_layout()

@export var left_margin:Vector2 = Vector2(0.0, 0.0):
	set(value):
		if self.skip_setter:
			left_margin = value
			return
		if left_margin == value: return
		left_margin = value
		request_layout()

@export var top_margin:Vector2 = Vector2(0.0, 0.0):
	set(value):
		if self.skip_setter:
			top_margin = value
			return
		if top_margin == value: return
		top_margin = value
		request_layout()

@export var right_margin:Vector2 = Vector2(0.0, 0.0):
	set(value):
		if self.skip_setter:
			right_margin = value
			return
		if right_margin == value: return
		right_margin = value
		request_layout()

@export var bottom_margin:Vector2 = Vector2(0.0, 0.0):
	set(value):
		if self.skip_setter:
			bottom_margin = value
			return
		if bottom_margin == value: return
		bottom_margin = value
		request_layout()

@export var width:Vector2 = Vector2(1.0, 0.0):
	set(value):
		if self.skip_setter:
			width = value
			return
		if width == value: return
		width = value
		request_layout()

@export var height:Vector2 = Vector2(0.0, 1.0):
	set(value):
		if self.skip_setter:
			height = value
			return
		if height == value: return
		height = value
		request_layout()

@export var reference_node: Control:
	set(value):
		if self.skip_setter:
			reference_node = value
			return
		if reference_node == value: return
		reference_node = value
		request_layout()

func only_show_properties_in_mode(property: Dictionary, visible_mode: Mode, names: Array):
	if property.name in names:
		if self.mode == visible_mode:
			property.usage = PROPERTY_USAGE_EDITOR
		else:
			property.usage = PROPERTY_USAGE_NO_EDITOR

func _validate_property(property: Dictionary):
	self.only_show_properties_in_mode(property, Mode.SIZE, ["width", "height"])
	self.only_show_properties_in_mode(property, Mode.MARGINS, ["right_margin", "bottom_margin"])
	 

func _ready():
	print("ready")
	self.skip_setter = false
	self.request_layout.call_deferred()

#func ready2():
#	print("ready2")
#	self.request_layout()
		
#func _enter_tree():
#	print("enter ", width)
#	self.request_layout()

var skip_setter = false

func unify_properties():
	self.skip_setter = true
	if mode == Mode.MARGINS:
		self.width = Vector2.RIGHT - self.right_margin - self.left_margin
		self.height = Vector2.DOWN - self.bottom_margin - self.top_margin
	else:
		self.right_margin = Vector2.RIGHT - self.width - self.left_margin
		self.bottom_margin = Vector2.DOWN - self.height - self.top_margin
	self.skip_setter = false

func calc_values(size_vec:Vector2, proportions:Vector2):
	return size_vec.dot(proportions)

func request_layout():
	self.unify_properties()

	if reference_node:
		var left = int(calc_values(reference_node.size, left_margin))
		var right = int(calc_values(reference_node.size, Vector2.RIGHT - right_margin))
		var top = int(calc_values(reference_node.size, top_margin))
		var bottom = int(calc_values(reference_node.size, Vector2.DOWN - bottom_margin))
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
		var x2 = floor(calc_values(parent_size, Vector2.RIGHT - right_margin))
		var y2 = floor(calc_values(parent_size, Vector2.DOWN - bottom_margin))
		var w = x2 - x
		var h = y2 - y
		if w < 0: w = 0
		if h < 0: h = 0
		self.position = Vector2(x, y)
		self.size = Vector2(w, h)
		print("set pos & size ", self.position, ", ", self.size)
