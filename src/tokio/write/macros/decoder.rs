macro_rules! decoder {
    ($(#[$attr:meta])* $name:ident) => {
        pin_project_lite::pin_project! {
            $(#[$attr])*
            #[derive(Debug)]
            ///
            /// This structure implements an [`AsyncWrite`](tokio::io::AsyncWrite) interface and will
            /// take in compressed data and write it uncompressed to an underlying stream.
            pub struct $name<W> {
                #[pin]
                inner: async_buf_write::tokio::Compat<
                    crate::generic::write::Decoder<
                        async_buf_write::tokio::Compat<W>,
                        crate::codec::$name,
                    >,
                >,
            }
        }

        impl<W: tokio::io::AsyncWrite> $name<W> {
            /// Creates a new decoder which will take in compressed data and write it uncompressed
            /// to the given stream.
            pub fn new(writer: W) -> $name<W> {
                $name {
                    inner: async_buf_write::tokio::Compat::new(
                        crate::generic::write::Decoder::new(
                            async_buf_write::tokio::Compat::new(writer),
                            crate::codec::$name::new(),
                        ),
                    ),
                }
            }

            /// Acquires a reference to the underlying reader that this decoder is wrapping.
            pub fn get_ref(&self) -> &W {
                self.inner.get_ref().get_ref().get_ref()
            }

            /// Acquires a mutable reference to the underlying reader that this decoder is
            /// wrapping.
            ///
            /// Note that care must be taken to avoid tampering with the state of the reader which
            /// may otherwise confuse this decoder.
            pub fn get_mut(&mut self) -> &mut W {
                self.inner.get_mut().get_mut().get_mut()
            }

            /// Acquires a pinned mutable reference to the underlying reader that this decoder is
            /// wrapping.
            ///
            /// Note that care must be taken to avoid tampering with the state of the reader which
            /// may otherwise confuse this decoder.
            pub fn get_pin_mut(self: std::pin::Pin<&mut Self>) -> std::pin::Pin<&mut W> {
                self.project().inner.get_pin_mut().get_pin_mut().get_pin_mut()
            }

            /// Consumes this decoder returning the underlying reader.
            ///
            /// Note that this may discard internal state of this decoder, so care should be taken
            /// to avoid losing resources when this is called.
            pub fn into_inner(self) -> W {
                self.inner.into_inner().into_inner().into_inner()
            }
        }

        impl<W: tokio::io::AsyncWrite> tokio::io::AsyncWrite for $name<W> {
            fn poll_write(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &[u8],
            ) -> std::task::Poll<std::io::Result<usize>> {
                self.project().inner.poll_write(cx, buf)
            }

            fn poll_flush(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                self.project().inner.poll_flush(cx)
            }

            fn poll_shutdown(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                self.project().inner.poll_shutdown(cx)
            }
        }

        const _: () = {
            fn _assert() {
                use crate::util::{_assert_send, _assert_sync};
                use core::pin::Pin;
                use tokio::io::AsyncWrite;

                _assert_send::<$name<Pin<Box<dyn AsyncWrite + Send>>>>();
                _assert_sync::<$name<Pin<Box<dyn AsyncWrite + Sync>>>>();
            }
        };
    }
}
