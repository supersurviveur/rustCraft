package fr.supersurviveur.mappingsmod;

import net.fabricmc.api.ModInitializer;

public class Mappingsmod implements ModInitializer {
    static {
        System.load("/home/julien/code/rustCraft/rustcraft/rustcraft_mappings_gen/target/debug/librustcraft_mappings_gen.so");
    }

    private native void run();

    @Override
    public void onInitialize() {
        run();
    }
}
