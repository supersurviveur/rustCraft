package fr.supersurviveur.rustcraftmod;

import fr.supersurviveur.rustcraftmod.rustapi.RustAPI;
import net.fabricmc.api.ModInitializer;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class Rustcraftmod implements ModInitializer {
    RustAPI rustAPIManager;
    public static final Logger LOGGER = LoggerFactory.getLogger("rustcraftmod");
    public static String MODID;

    @Override
    public void onInitialize() {
        LOGGER.info("Entry main");
        rustAPIManager = new RustAPI("rustcraftmod");

        rustAPIManager.onInitialize();

//        CommandRegistrationCallback.EVENT.register((dispatcher, registryAccess, environment) -> dispatcher.register(CommandManager.literal("reload").executes(context -> {
//            context.getSource().sendFeedback(() -> Text.literal("Called /reload"), false);
//            rustAPIManager.reload();
//            context.getSource().sendFeedback(() -> Text.literal("§2Reloaded"), false);
//            return 1;
//        })));
    }
}
