@tool

class_name SubsceneSizeRememberer
extends Control

var in_subscene: bool
@export var cached_size: Vector2 = -Vector2.ONE:
	set(value):
		cached_size = value
		if Engine.is_editor_hint():
			if not in_subscene:
				self.apply_cache()

func _ready():
	if not Engine.is_editor_hint():
		return
		
	if self.owner != null and self.owner.owner != null:
		self.ready_as_subscene()
	else:
		self.ready_as_scene()

func ready_as_subscene():
	in_subscene = true

func ready_as_scene():
	in_subscene = false
	if cached_size != -Vector2.ONE:
		self.apply_cache.call_deferred()

func apply_cache():
	var parent = self.get_parent()
	assert(parent is Control)
	parent.size = self.cached_size
