package fr.supersurviveur.rustcraftmod.rustapi;

import fr.supersurviveur.rustcraftmod.rustapi.rustblock.BlockAPI;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;


public class RustAPI {
    public BlockAPI blockAPI;

    String buildPath = "/home/julien/code/rustCraft/build/out/librust_lib_test.so"; // TODO use a proper config for this

    public native void onInitialize();

    public RustAPI() {
        System.load(buildPath);
        blockAPI = new BlockAPI(this);
    }

    public BlockAPI getBlockAPI() {
        return this.blockAPI;
    }

    public void info(String message) {
        Logger LOGGER = LoggerFactory.getLogger("rustcraftmod");
        LOGGER.info(message);
    }
}
