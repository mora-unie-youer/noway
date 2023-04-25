use smithay::{
    backend::renderer::{
        element::surface::WaylandSurfaceRenderElement, ImportAll, ImportMem, Renderer,
    },
    render_elements,
};

use super::pointer::PointerRenderElement;

render_elements! {
    pub CustomRenderElements<R> where
        R: ImportAll + ImportMem;
    Pointer=PointerRenderElement<R>,
    Surface=WaylandSurfaceRenderElement<R>,
}

impl<R> std::fmt::Debug for CustomRenderElements<R>
where
    R: Renderer + std::fmt::Debug,
    <R as Renderer>::TextureId: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pointer(arg0) => f.debug_tuple("Pointer").field(arg0).finish(),
            Self::Surface(arg0) => f.debug_tuple("Surface").field(arg0).finish(),
            Self::_GenericCatcher(arg0) => f.debug_tuple("_GenericCatcher").field(arg0).finish(),
        }
    }
}
