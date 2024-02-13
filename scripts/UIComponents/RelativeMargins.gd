@tool

class_name RelativeMargins
extends GGComponent

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

@export var right_margin:Vector2 = Vector2(0.0, 0.0):
	set(value):
		if right_margin == value: return
		right_margin = value
		request_layout()

@export var bottom_margin:Vector2 = Vector2(0.0, 0.0):
	set(value):
		if bottom_margin == value: return
		bottom_margin = value
		request_layout()

func calc_values(size_vec:Vector2, proportions:Vector2):
	return size_vec.dot(proportions)

func _resolve_shrink_to_fit_height(available_size:Vector2):
	super(available_size)

	var relative_size = available_size
	if reference_node:
		relative_size = reference_node.size
	
	size.y += int(calc_values(relative_size, top_margin))
	size.y += int(calc_values(relative_size, Vector2.DOWN - bottom_margin))

func _resolve_shrink_to_fit_width(available_size:Vector2):
	super(available_size)

	var relative_size = available_size
	if reference_node:
		relative_size = reference_node.size
	
	size.x += int(calc_values(relative_size, left_margin))
	size.x += int(calc_values(relative_size, Vector2.RIGHT - right_margin))

func _with_margins(rect:Rect2)->Rect2:
	if reference_node:
		var left = int(calc_values(reference_node.size, left_margin))
		var right = int(calc_values(reference_node.size, Vector2.RIGHT - right_margin))
		var top = int(calc_values(reference_node.size, top_margin))
		var bottom = int(calc_values(reference_node.size, Vector2.DOWN - bottom_margin))
		var x = rect.position.x + left
		var y = rect.position.y + top
		var x2 = rect.position.x + (rect.size.x - right)
		var y2 = rect.position.y + (rect.size.y - bottom)
		var w = x2 - x
		var h = y2 - y
		if w < 0: w = 0
		if h < 0: h = 0
		return Rect2(x, y, w, h)
	else:
		var x = rect.position.x + floor(calc_values(rect.size, left_margin))
		var y = rect.position.y +floor(calc_values(rect.size, top_margin))
		var x2 = rect.position.x +floor(calc_values(rect.size, Vector2.RIGHT - right_margin))
		var y2 = rect.position.y + floor(calc_values(rect.size, Vector2.DOWN - bottom_margin))
		var w = x2 - x
		var h = y2 - y
		if w < 0: w = 0
		if h < 0: h = 0
		print(Rect2(x, y, w, h))
		return Rect2(x, y, w, h)
