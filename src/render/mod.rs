use smithay::{
    backend::renderer::{
        damage::{Error as OutputDamageTrackerError, OutputDamageTracker},
        element::{RenderElement, RenderElementStates, Wrap},
        ImportAll, ImportMem, Renderer,
    },
    desktop::space::{space_render_elements, Space, SpaceRenderElements},
    output::Output,
    render_elements,
    utils::{Physical, Rectangle},
};

use self::window::{WindowElement, WindowRenderElement};

pub mod window;

render_elements! {
    pub OutputRenderElements<R, E>
        where R: ImportAll + ImportMem;
    Space=SpaceRenderElements<R, E>,
    Window=Wrap<E>,
}

impl<R, E> std::fmt::Debug for OutputRenderElements<R, E>
where
    R: Renderer + ImportAll + ImportMem + std::fmt::Debug,
    E: RenderElement<R> + std::fmt::Debug,
    <R as Renderer>::TextureId: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Space(arg0) => f.debug_tuple("Space").field(arg0).finish(),
            Self::Window(arg0) => f.debug_tuple("Window").field(arg0).finish(),
            Self::_GenericCatcher(arg0) => f.debug_tuple("_GenericCatcher").field(arg0).finish(),
        }
    }
}

pub fn output_elements<R>(
    output: &Output,
    space: &Space<WindowElement>,
    renderer: &mut R,
) -> Vec<OutputRenderElements<R, WindowRenderElement<R>>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + 'static,
{
    let mut output_render_elements = Vec::new();

    let space_elements = space_render_elements(renderer, [space], output).unwrap();
    output_render_elements.extend(space_elements.into_iter().map(OutputRenderElements::Space));

    output_render_elements
}

type Damage = Vec<Rectangle<i32, Physical>>;
pub fn render_output<R>(
    output: &Output,
    space: &Space<WindowElement>,
    renderer: &mut R,
    damage_tracker: &mut OutputDamageTracker,
    age: usize,
) -> Result<(Option<Damage>, RenderElementStates), OutputDamageTrackerError<R>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + 'static,
{
    let elements = output_elements(output, space, renderer);
    damage_tracker.render_output(renderer, age, &elements, [0.1, 0.1, 0.1, 1.0])
}
