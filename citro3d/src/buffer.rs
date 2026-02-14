//! Configure vertex buffer objects to be sent to the GPU for rendering.
//!
//! See the [`attrib`] module for details on how to describe the shape and type
//! of the VBO data.

use std::any::type_name;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::rc::Rc;

use ctru::linear::LinearAllocator;

use crate::Error;
use crate::attrib;

/// A buffer allocated in Linear memory.
pub trait BufferData: 'static {
    /// A pointer to the underlying data
    fn buf_ptr(&self) -> *mut c_void;
    /// The size (in bytes) of each element in the buffer
    fn stride(&self) -> usize;
    /// How many elements are in the buffer
    fn buf_len(&self) -> usize;
}

impl<T: Sized + 'static> BufferData for Vec<T, LinearAllocator> {
    fn buf_ptr(&self) -> *mut c_void {
        self.as_ptr() as _
    }

    fn stride(&self) -> usize {
        std::mem::size_of::<T>()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }
}

impl<T: Sized + 'static> BufferData for Box<[T], LinearAllocator> {
    fn buf_ptr(&self) -> *mut c_void {
        self.as_ref() as *const _ as _
    }

    fn stride(&self) -> usize {
        std::mem::size_of::<T>()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }
}

impl<T: Sized + 'static> BufferData for Rc<[T], LinearAllocator> {
    fn buf_ptr(&self) -> *mut c_void {
        self.as_ref() as *const _ as _
    }

    fn stride(&self) -> usize {
        std::mem::size_of::<T>()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }
}

/// A handle to a VBO buffer in linear memory, to be used with [`Info`].
/// This handle is reference counted so can be cheaply cloned and used
/// with mutliple [`Info`] instances without duplicating memory.
#[derive(Clone)]
pub struct Buffer {
    _data: Rc<dyn BufferData>,

    // These fields could just be dynamically fetched since the buffer
    // is dyn BufferVec, or we can spend 12 extra bytes per buffer to
    // avoid some indirection each draw call.
    data_ptr: *const c_void,
    stride: isize,
    len: usize,
}

impl Buffer {
    /// Allocate a new `Buffer` in Linear memory and copy `data` into it.
    /// Each element of `data` should correspond to data for a single vertex.
    ///
    /// If you already have an owned [`Vec`] in Linear memory then
    /// [`Buffer::new_in_linear`] should be preferred to take ownership
    /// of that allocation instead of reallocating and copying.
    pub fn new<T: Sized + Copy + 'static>(data: &[T]) -> Buffer {
        let mut linear_data = Vec::with_capacity_in(data.len(), LinearAllocator);
        linear_data.extend_from_slice(data);

        Buffer {
            data_ptr: linear_data.as_ptr() as _,
            len: linear_data.len(),
            stride: linear_data
                .stride()
                .try_into()
                .map_err(|_| format!("{} is too large to be used in a buffer.", type_name::<T>()))
                .unwrap(),
            _data: Rc::new(linear_data),
        }
    }

    /// Allocate a new `Buffer` in Linear memory and copy `data` into it.
    /// The `stride` should correspond to the number of bytes in data
    /// per single vertex.
    ///
    /// If you already have an owned [`Vec`] in Linear memory then
    /// [`Buffer::new_in_linear_with_stride`] should be preferred to take ownership
    /// of that allocation instead of reallocating and copying.
    ///
    /// # Errors
    /// * If the length of `data` is not a mutliple of `stride`
    pub fn new_with_stride(data: &[u8], stride: usize) -> Result<Buffer, ()> {
        if data.len() % stride != 0 {
            return Err(());
        }

        let mut linear_data = Vec::with_capacity_in(data.len(), LinearAllocator);
        linear_data.extend_from_slice(data);

        Ok(Buffer {
            data_ptr: linear_data.as_ptr() as _,
            len: linear_data.len() / stride,
            stride: stride as isize,
            _data: Rc::new(linear_data),
        })
    }

    /// Convert an existing buffer allocated in Linear memory to a `Buffer` to be used
    /// with [`buffer::Info`]. Each element in `data` should correspond with data for
    /// a single vertex.
    pub fn new_in_linear<B: BufferData>(data: B) -> Buffer {
        Buffer {
            data_ptr: data.buf_ptr() as _,
            stride: data
                .stride()
                .try_into()
                .map_err(|_| format!("{}'s buffer elements are too large.", type_name::<B>()))
                .unwrap(),
            len: data.buf_len(),
            _data: Rc::new(data),
        }
    }

    /// Convert an existing buffer of unstructured data allocated in Linear memory
    /// e.g. `Vec<u8, LinearAllocator>` to a `Buffer` to be used with [`buffer::Info`].
    /// Each element in `data` should correspond with data for a single vertex.
    ///
    /// # Errors
    /// * If the length of `data` is not a mutliple of `stride`
    pub fn new_in_linear_with_stride(data: impl BufferData, stride: usize) -> Result<Buffer, ()> {
        if data.buf_len() % stride != 0 {
            return Err(());
        }

        Ok(Buffer {
            data_ptr: data.buf_ptr() as _,
            stride: stride
                .try_into()
                .map_err(|_| format!("{stride} is too large for a buffer element."))
                .unwrap(),
            len: data.buf_len() / stride,
            _data: Rc::new(data),
        })
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.data_ptr
    }

    pub fn stride(&self) -> isize {
        self.stride
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

/// Vertex buffer info. This struct is used to describe the shape of the buffer
/// data to be sent to the GPU for rendering.
#[doc(alias = "C3D_BufInfo")]
#[derive(Clone)]
pub struct Info {
    info: citro3d_sys::C3D_BufInfo,
    buffers: Vec<Buffer>,
}

/// A type that can be used as an index for indexed drawing.
pub trait Index {
    /// The data type of the index, as used by [`citro3d_sys::C3D_DrawElements`]'s `type_` parameter.
    const TYPE: libc::c_int;
}

impl Index for u8 {
    const TYPE: libc::c_int = citro3d_sys::C3D_UNSIGNED_BYTE as _;
}

impl Index for u16 {
    const TYPE: libc::c_int = citro3d_sys::C3D_UNSIGNED_SHORT as _;
}

/// The geometric primitive to draw (i.e. what shapes the buffer data describes).
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
#[doc(alias = "GPU_Primitive_t")]
pub enum Primitive {
    /// Draw triangles (3 vertices per triangle).
    Triangles = ctru_sys::GPU_TRIANGLES,
    /// Draw a triangle strip (each vertex shared by 1-3 triangles).
    TriangleStrip = ctru_sys::GPU_TRIANGLE_STRIP,
    /// Draw a triangle fan (first vertex shared by all triangles).
    TriangleFan = ctru_sys::GPU_TRIANGLE_FAN,
    /// Geometry primitive. Can be used for more complex use cases like geometry
    /// shaders that output custom primitives.
    GeometryPrim = ctru_sys::GPU_GEOMETRY_PRIM,
}

impl Default for Info {
    #[doc(alias = "BufInfo_Init")]
    fn default() -> Self {
        let mut info = MaybeUninit::zeroed();
        let info = unsafe {
            citro3d_sys::BufInfo_Init(info.as_mut_ptr());
            info.assume_init()
        };
        Self {
            info,
            buffers: Vec::new(),
        }
    }
}

impl Info {
    pub fn as_raw(&self) -> *mut citro3d_sys::C3D_BufInfo {
        &self.info as *const _ as _
    }

    /// Construct buffer info without any registered data.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> u16 {
        self.buffers.first().map(|b| b.len() as _).unwrap_or(0)
    }

    /// Register vertex buffer object data. The resulting [`Slice`] will have its
    /// lifetime tied to both this [`Info`] and the passed-in VBO. `vbo_data` is
    /// assumed to use one `T` per drawn primitive, and its layout is assumed to
    /// match the given `attrib_info`
    ///
    /// # Errors
    ///
    /// Registering VBO data may fail:
    ///
    /// * if `vbo_data` is not allocated with the [`ctru::linear`] allocator
    /// * if the maximum number (12) of VBOs are already registered
    #[doc(alias = "BufInfo_Add")]
    pub fn add<'this, 'idx>(
        &'this mut self,
        vbo_buffer: Buffer,
        permutation: attrib::Permutation,
    ) -> Result<(), Error>
    where
        'this: 'idx,
    {
        // SAFETY:
        // * The lifetime of the VBO data is extended by the `Buffer` copy that is
        // stored in `self.buffers` which reference counts the buffer allocation
        // * The pointer to &mut self.0 is used to access values
        // in the BufInfo, not copied to be used later.
        let res = unsafe {
            citro3d_sys::BufInfo_Add(
                &mut self.info,
                vbo_buffer.as_ptr().cast(),
                vbo_buffer.stride(),
                permutation.attrib_count as _,
                permutation.permutation,
            )
        };

        // Error codes from <https://github.com/devkitPro/citro3d/blob/master/source/buffers.c#L11>
        match res {
            ..=-3 => Err(crate::Error::System(res)),
            -2 => Err(crate::Error::InvalidMemoryLocation),
            -1 => Err(crate::Error::TooManyBuffers),
            _ => {
                self.buffers.push(vbo_buffer);
                Ok(())
            }
        }
    }
}
