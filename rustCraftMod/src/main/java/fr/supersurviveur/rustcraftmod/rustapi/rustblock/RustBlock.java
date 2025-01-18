package fr.supersurviveur.rustcraftmod.rustapi.rustblock;

import fr.supersurviveur.rustcraftmod.rustapi.RustAPI;
import net.minecraft.block.Block;
import net.minecraft.block.BlockState;
import net.minecraft.entity.Entity;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;

public class RustBlock extends Block  {
    long rust_object;
    RustAPI rustAPI;

    public RustBlock(Settings settings, RustAPI rustAPI, long block) {
        super(settings);
        this.rustAPI = rustAPI;
        this.rust_object = block;
    }


    @Override
    public native void onSteppedOn(World world, BlockPos pos, BlockState state, Entity entity);

}
