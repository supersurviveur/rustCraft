use jni::objects::JObject;
use rustcraft_codegen::mappings_with_sig;

use crate::api::ModApi;

#[derive(Debug)]
pub struct BlockPos<'a> {
    api: ModApi<'a>,
    block_pos: JObject<'a>,
}

impl<'a> BlockPos<'a> {
    pub fn new(api: ModApi<'a>, block_pos: JObject<'a>) -> Self {
        BlockPos { api, block_pos }
    }
    pub fn get_x(&self) -> i32 {
        self.api
            .call_method(
                Some(&self.block_pos),
                mappings_with_sig!("net/minecraft/util/math/Vec3i", "getX"),
                &[],
            )
            .i()
            .unwrap()
    }
    pub fn get_y(&self) -> i32 {
        self.api
            .call_method(
                Some(&self.block_pos),
                mappings_with_sig!("net/minecraft/util/math/Vec3i", "getY"),
                &[],
            )
            .i()
            .unwrap()
    }
    pub fn get_z(&self) -> i32 {
        self.api
            .call_method(
                Some(&self.block_pos),
                mappings_with_sig!("net/minecraft/util/math/Vec3i", "getZ"),
                &[],
            )
            .i()
            .unwrap()
    }
}
