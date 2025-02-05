package fr.supersurviveur.rustcraftmod.rustapi;

import fr.supersurviveur.rustcraftmod.Rustcraftmod;
import fr.supersurviveur.rustcraftmod.rustapi.rustblock.BlockAPI;
import net.bytebuddy.ByteBuddy;
import net.bytebuddy.dynamic.DynamicType;
import net.minecraft.block.Block;
import org.slf4j.LoggerFactory;

import java.lang.constant.MethodTypeDesc;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Modifier;
import java.lang.reflect.Type;


public class RustAPI {
    public static BlockAPI blockAPI;
    public static String modName;

    static String buildPath = "/media/julien/SSD1/code/rustCraft/build/out/librustcraft_test.so"; // TODO use a proper config for this

    public RustAPI(String modName) {
        System.load(buildPath);
        blockAPI = new BlockAPI(this);
        RustAPI.modName = modName;
        Rustcraftmod.MODID = modName;
    }

    public static void info(String message) {
        LoggerFactory.getLogger(modName).info(message);
    }

    public native void onInitialize();

    public void reload() {
        System.load(buildPath);
    }

    public BlockAPI getBlockAPI() {
        return blockAPI;
    }

    private Class<?> classForName(String c) throws ClassNotFoundException {
        if (c.startsWith("L")) {
            c = c.substring(1, c.length() - 1);
        }
        return Class.forName(c.replace("/", "."));
    }

    public Class<?> makeClass(String className, String[] methodsNames, String[] methodsSig, String superClass) {
        DynamicType.Builder<?> builder;
        if (superClass != null) {
            try {
                builder = new ByteBuddy().subclass(classForName(superClass));
            } catch (ClassNotFoundException e) {
                throw new RuntimeException(e);
            }
        } else {
            builder = new ByteBuddy().subclass(Object.class);
        }
        builder = builder
                .name("fr.supersurviveur.rustcraftmod."+className)
                .defineField("rust_object", long.class, Modifier.PUBLIC);
        try {
            for (int i = 0; i < methodsNames.length; i++) {
                MethodType sig = MethodTypeDesc.ofDescriptor(methodsSig[i]).resolveConstantDesc(MethodHandles.lookup());
                builder = builder.defineMethod(methodsNames[i], sig.returnType(), Modifier.PUBLIC | Modifier.NATIVE).withParameters(sig.parameterArray()).withoutCode();
            }
        } catch (ReflectiveOperationException e) {
            throw new RuntimeException(e);
        }

        return builder.make().load(this.getClass().getClassLoader()).getLoaded();
    }
}
