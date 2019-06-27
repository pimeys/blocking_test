use super::Res;
use futures03::Future;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct DBIO<T>(pub Pin<Box<dyn Future<Output = Res<T>> + Send + 'static>>);

impl<T> Future for DBIO<T> {
    type Output = Res<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}
