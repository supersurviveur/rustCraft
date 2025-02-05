package fr.supersurviveur.rustcraftmod.rustapi.rustblock;

import fr.supersurviveur.rustcraftmod.Rustcraftmod;
import fr.supersurviveur.rustcraftmod.rustapi.RustAPI;
import net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityTypeBuilder;
import net.minecraft.block.AbstractBlock;
import net.minecraft.block.Block;
import net.minecraft.block.BlockState;
import net.minecraft.block.entity.BlockEntityType;
import net.minecraft.entity.player.PlayerEntity;
import net.minecraft.item.BlockItem;
import net.minecraft.item.Item;
import net.minecraft.registry.Registries;
import net.minecraft.registry.Registry;
import net.minecraft.registry.RegistryKey;
import net.minecraft.registry.RegistryKeys;
import net.minecraft.util.Identifier;

import net.bytebuddy.ByteBuddy;
import net.bytebuddy.dynamic.DynamicType;
import net.minecraft.util.hit.BlockHitResult;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;

import java.lang.reflect.*;

public class BlockAPI {
    public static BlockEntityType<RustBlockEntity> DEMO_BLOCK;
    RustAPI rustApi;

    public BlockAPI(RustAPI rustApi) {
        this.rustApi = rustApi;
    }


    public void createBlock(long block, String block_name, Class<?> subclass) {
        try {
            Identifier id = Identifier.of(Rustcraftmod.MODID, block_name);
            RegistryKey<Block> key = RegistryKey.of(RegistryKeys.BLOCK, id);
            RegistryKey<Item> ikey = RegistryKey.of(RegistryKeys.ITEM, id);
            Object test = subclass.getDeclaredConstructor(AbstractBlock.Settings.class).newInstance(AbstractBlock.Settings.create().registryKey(key));
            test.getClass().getField("rust_object").set(test, block);
            Registry.register(Registries.BLOCK, Identifier.of(Rustcraftmod.MODID, block_name), (Block) test);
        } catch (InstantiationException | InvocationTargetException | IllegalAccessException | NoSuchMethodException |
                 NoSuchFieldException e) {
            throw new RuntimeException(e);
        }

        Identifier id = Identifier.of(Rustcraftmod.MODID, "test_rust");
        RegistryKey<Block> key = RegistryKey.of(RegistryKeys.BLOCK, id);
        RegistryKey<Item> ikey = RegistryKey.of(RegistryKeys.ITEM, id);
        RustBlock EXAMPLE_BLOCK = new RustBlock(AbstractBlock.Settings.create().registryKey(key), this.rustApi, block);
        Registry.register(Registries.BLOCK, Identifier.of(Rustcraftmod.MODID, "test_rust"), EXAMPLE_BLOCK);
        Registry.register(Registries.ITEM, Identifier.of(Rustcraftmod.MODID, "test_rust"), new BlockItem(EXAMPLE_BLOCK, new Item.Settings().registryKey(ikey)));

        DEMO_BLOCK = Registry.register(Registries.BLOCK_ENTITY_TYPE, Identifier.of("tutorial", "test_rust"), FabricBlockEntityTypeBuilder.create(RustBlockEntity::new, EXAMPLE_BLOCK).build());
    }
}
