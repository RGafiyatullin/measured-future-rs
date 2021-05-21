use super::*;

use std::cell::RefCell;

use ::polymap::TypeMap;

thread_local! {
    static CURRENT: RefCell<TypeMap> = Default::default();
}

pub fn with<R, F, Out>(f: F) -> Out
where
    R: Send + Sync + 'static,
    F: FnOnce(Option<&mut dyn ReportSink<R>>) -> Out,
{
    CURRENT.with(|current| {
        if let Some(mut typemap) = current.try_borrow_mut().ok() {
            f(typemap.get_mut::<BoxedReportSink<R>>().map(AsMut::as_mut))
        } else {
            f(None)
        }
    })
}

pub fn install<R, S>(sink: S) -> Option<BoxedReportSink<R>>
where
    R: Send + Sync + 'static,
    S: ReportSink<R>,
{
    let boxed = BoxedReportSink::new(sink);
    replace(Some(boxed))
}

pub fn replace<R>(sink_opt: Option<BoxedReportSink<R>>) -> Option<BoxedReportSink<R>>
where
    R: Send + Sync + 'static,
{
    CURRENT.with(|current| {
        // _trying_ to borrow here because this method may be called from `Drop::drop`
        current.try_borrow_mut().ok().and_then(|mut current| {
            if let Some(sink) = sink_opt {
                current.insert(sink)
            } else {
                current.remove()
            }
        })
    })
}
