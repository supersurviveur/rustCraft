package fr.supersurviveur.rustcraftmod.rustapi.rustblock;

import fr.supersurviveur.rustcraftmod.rustapi.RustAPI;
import net.minecraft.block.AbstractBlock;
import net.minecraft.block.Block;
import net.minecraft.item.BlockItem;
import net.minecraft.item.Item;
import net.minecraft.registry.Registries;
import net.minecraft.registry.Registry;
import net.minecraft.registry.RegistryKey;
import net.minecraft.registry.RegistryKeys;
import net.minecraft.util.Identifier;

public class BlockAPI {
    RustAPI rustApi;

    public BlockAPI(RustAPI rustApi) {
        this.rustApi = rustApi;
    }

    public void createBlock(long block) {
        Identifier id = Identifier.of("tutorial", "test_rust");
        RegistryKey<Block> key = RegistryKey.of(RegistryKeys.BLOCK, id);
        RegistryKey<Item> ikey = RegistryKey.of(RegistryKeys.ITEM, id);
        RustBlock EXAMPLE_BLOCK = new RustBlock(AbstractBlock.Settings.create().registryKey(key), this.rustApi, block);
        Registry.register(Registries.BLOCK, Identifier.of("tutorial", "test_rust"), EXAMPLE_BLOCK);
        Registry.register(Registries.ITEM, Identifier.of("tutorial", "test_rust"), new BlockItem(EXAMPLE_BLOCK, new Item.Settings().registryKey(ikey)));
    }

}
