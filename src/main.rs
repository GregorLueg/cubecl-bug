use cubecl::prelude::*;
use cubecl::server::Handle;
use cubecl::wgpu::{WgpuDevice, WgpuRuntime};
use cubecl::zspace::striding::row_major_contiguous_strides;
use cubecl::zspace::{Shape, Strides};
use std::marker::PhantomData;

pub struct GpuTensor<R: Runtime, F: CubeElement + Numeric> {
    data: Handle,
    shape: Vec<usize>,
    strides: Vec<usize>,
    _r: PhantomData<R>,
    _f: PhantomData<F>,
}

impl<R: Runtime, F: CubeElement + Numeric> Clone for GpuTensor<R, F> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            shape: self.shape.clone(),
            strides: self.strides.clone(),
            _r: PhantomData,
            _f: PhantomData,
        }
    }
}

impl<R: Runtime, F: Numeric + CubeElement> GpuTensor<R, F> {
    pub fn from_slice(data: &[F], shape: Vec<usize>, client: &ComputeClient<R>) -> Self {
        let handle = client.create_from_slice(F::as_bytes(data));
        let strides = row_major_contiguous_strides(&shape).to_vec();
        Self {
            data: handle,
            shape,
            strides,
            _r: PhantomData,
            _f: PhantomData,
        }
    }

    pub fn empty(shape: Vec<usize>, client: &ComputeClient<R>) -> Self {
        let size = shape.iter().product::<usize>() * core::mem::size_of::<F>();
        let handle = client.empty(size);
        let strides = row_major_contiguous_strides(&shape).to_vec();
        Self {
            data: handle,
            shape,
            strides,
            _r: PhantomData,
            _f: PhantomData,
        }
    }

    pub fn into_tensor_arg(&self) -> TensorArg<R> {
        unsafe {
            TensorArg::from_raw_parts(
                self.data.clone(),
                Strides::new(&self.strides),
                Shape::from(self.shape.clone()),
            )
        }
    }

    pub fn read(self, client: &ComputeClient<R>) -> Vec<F> {
        let bytes = client.read_one(self.data).unwrap();
        F::from_bytes(&bytes).to_vec()
    }

    pub fn vram_bytes(&self) -> usize {
        self.shape.iter().product::<usize>() * std::mem::size_of::<F>()
    }

    pub fn handle(&self) -> &Handle {
        &self.data
    }
}

#[cube(launch_unchecked)]
fn repro(input: &Tensor<u32>, out: &mut Tensor<u32>) {
    if UNIT_POS_X != 0u32 {
        terminate!();
    }
    let a = input[0usize]; // 130
    let b = input[1usize]; // 494

    // 1. expression-if (ternary) with a runtime condition -> WRONG
    //    a != b is true, so this must be 1. Observed: 0 (the else branch).
    out[0usize] = if a != b { 1u32 } else { 0u32 };

    // 2. the other arm of the same comparison, also via expression-if -> WRONG
    //    a == b is false, so this must be 0. Observed: 0 (the else branch).
    out[1usize] = if a == b { 1u32 } else { 0u32 };
    // (1) and (2) are both 0 -> impossible for two integers; the construct,
    //     not the comparison, is returning its else value unconditionally.

    // 3. statement-if with the same runtime condition -> CORRECT
    out[2usize] = 7u32;
    if a != b {
        out[2usize] = 1u32; // runs as expected -> 1
    }

    // 4. expression-if with a comptime/literal condition -> CORRECT
    //    (folded at compile time, no runtime select emitted)
    out[3usize] = if 1u32 != 0u32 { 1u32 } else { 0u32 }; // -> 1

    // 5. the same comparison inside a while condition -> CORRECT
    let mut c = 0u32;
    let mut i = 0u32;
    while i < b {
        c += 1u32;
        i += 1u32;
    }
    out[4usize] = c; // -> 494

    // raw values, to prove the inputs are read correctly
    out[5usize] = a; // -> 130
    out[6usize] = b; // -> 494
}

fn main() {
    let device = WgpuDevice::DefaultDevice;
    let client = WgpuRuntime::client(&device);

    let input = GpuTensor::<WgpuRuntime, u32>::from_slice(&[130u32, 494u32], vec![2], &client);
    let out = GpuTensor::<WgpuRuntime, u32>::from_slice(&[0u32; 7], vec![7], &client);

    unsafe {
        repro::launch_unchecked::<WgpuRuntime>(
            &client,
            CubeCount::Static(1, 1, 1),
            CubeDim::new_2d(32, 1),
            input.into_tensor_arg(),
            out.clone().into_tensor_arg(),
        );
    }

    let o = out.read(&client);

    println!("out[0] = {}  // a != b via expr-if   (expect 1)", o[0]);
    println!("out[1] = {}  // a == b via expr-if   (expect 0)", o[1]);
    println!("out[2] = {}  // a != b via stmt-if   (expect 1)", o[2]);
    println!("out[3] = {}  // 1 != 0 via expr-if   (expect 1)", o[3]);
    println!("out[4] = {}  // while i < b counter  (expect 494)", o[4]);
    println!("out[5] = {}  // a                     (expect 130)", o[5]);
    println!("out[6] = {}  // b                     (expect 494)", o[6]);

    assert_eq!(o[5], 130);
    assert_eq!(o[6], 494);
    assert_eq!(o[2], 1, "statement-if regressed too");
    assert_eq!(o[3], 1, "comptime expr-if regressed too");
    assert_eq!(
        o[0], 1,
        "BUG: runtime expression-if returned the else branch"
    );
    assert!(
        !(o[0] == 0 && o[1] == 0),
        "BUG: a != b and a == b both false -- expression-if ignores its condition"
    );
}
