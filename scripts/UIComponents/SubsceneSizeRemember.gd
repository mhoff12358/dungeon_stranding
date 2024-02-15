@tool

class_name SubsceneSizeRememberer
extends Control

var in_subscene: bool
@export var scene_path:String = ""

func _ready():
	if not Engine.is_editor_hint():
		return
		
	if self.owner != null and self.owner.owner != null:
		self.ready_as_subscene()
	else:
		self.ready_as_scene()

func ready_as_subscene():
	in_subscene = true
	self.get_parent().resized.connect(self.parent_resized)

func ready_as_scene():
	in_subscene = false

func parent_resized():
	if self.scene_path.is_empty():
		return

	var input_scene: PackedScene = load(self.scene_path)
	var root = input_scene.instantiate()
	assert(root is Control)
	root.size = self.get_parent().size

	var output_scene = PackedScene.new()
	var pack_result = output_scene.pack(root)
	output_scene.take_over_path(self.scene_path)
	if pack_result == OK:
		var error = ResourceSaver.save(output_scene, self.scene_path)
		if error != OK:
			push_error("Failed to update subscene size for \"" + self.scene_path + "\".")
