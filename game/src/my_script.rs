use fyrox::{
    core::{reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*},
    event::Event,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "afad6894-70ba-433a-b1fa-a548c886aa01")]
#[visit(optional)]
pub struct MyScript {
    // Add fields here.
}

impl ScriptTrait for MyScript {
    fn on_init(&mut self, _context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, _context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Put object logic here.
        let transform = context.scene.graph[context.handle].local_transform_mut();
        // TODO: through animation nodes ?
        let mut pos = transform.position().clone_owned();
        pos.x = f32::cos(context.elapsed_time % 6.3);
        transform.set_position(pos);
    }
}
