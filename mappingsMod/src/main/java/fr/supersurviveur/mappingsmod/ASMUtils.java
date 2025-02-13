package fr.supersurviveur.mappingsmod;

import org.objectweb.asm.ClassReader;
import org.objectweb.asm.tree.AnnotationNode;
import org.objectweb.asm.tree.ClassNode;
import org.objectweb.asm.tree.FieldNode;
import org.objectweb.asm.tree.MethodNode;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.lang.constant.MethodTypeDesc;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Executable;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.Enumeration;
import java.util.jar.JarEntry;
import java.util.jar.JarFile;


public class ASMUtils {
    public static boolean isNullable(MethodNode methodNode) {
        if (methodNode.visibleAnnotations != null) {
            for (AnnotationNode annotation : methodNode.visibleAnnotations) {
                if (annotation.desc.equals("Lorg/jetbrains/annotations/Nullable;")) {
                    return true;
                }
            }
        }
        return false;
    }

    public static boolean isNullable(FieldNode fieldNode) {
        if (fieldNode.visibleAnnotations != null) {
            for (AnnotationNode annotation : fieldNode.visibleAnnotations) {
                if (annotation.desc.equals("Lorg/jetbrains/annotations/Nullable;")) {
                    return true;
                }
            }
        }
        return false;
    }

    public static boolean isNullable(MethodNode methodNode, int argPos) {
        if (methodNode.visibleParameterAnnotations != null && methodNode.visibleParameterAnnotations[argPos] != null) {
            for (AnnotationNode annotation : methodNode.visibleParameterAnnotations[argPos]) {
                if (annotation.desc.equals("Lorg/jetbrains/annotations/Nullable;")) {
                    return true;
                }
            }
        }
        return false;
    }

    public static byte[] getBytes(String className, String jar) throws IOException {
        String javaFileName = className.replace('.', '/') + ".class";

        try (JarFile jarFile = new JarFile(jar)) {
            Enumeration<JarEntry> entries = jarFile.entries();
            while (entries.hasMoreElements()) {
                JarEntry entry = entries.nextElement();

                if (entry.getName().endsWith(".class") && entry.getName().equals(javaFileName)) {
                    InputStream inputStream = jarFile.getInputStream(entry);
                    return getBytes(inputStream);
                }
            }
        }
        throw new IOException("Class " + className + " not found");
    }

    public static byte[] getBytes(InputStream is) throws IOException {
        try (ByteArrayOutputStream os = new ByteArrayOutputStream();) {
            byte[] buffer = new byte[0xFFFF];
            for (int len; (len = is.read(buffer)) != -1; )
                os.write(buffer, 0, len);
            os.flush();
            return os.toByteArray();
        }
    }

    public static ClassNode getClassNode(String c, String jar) {
        try {
            byte[] bytes = getBytes(c, jar);
            ClassReader classReader = new ClassReader(bytes);
            ClassNode classNode = new ClassNode();
            classReader.accept(classNode, 0);
            return classNode;
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public static ClassNode getClassNode(Class<?> c) {
        return getClassNode(c.getName(), c.getProtectionDomain().getCodeSource().getLocation().getPath());
    }

    public static boolean isStaticMethod(Class<?> c, String name, Class<?>... parameterTypes) {
        try {
            Method method = c.getMethod(name, parameterTypes);
            return isStatic(method);
        } catch (NoSuchMethodException e) {
            throw new RuntimeException(e);
        }
    }

    public static boolean isStatic(Method method) {
        return java.lang.reflect.Modifier.isStatic(method.getModifiers());
    }

    public static boolean isStatic(Executable method) {
        return java.lang.reflect.Modifier.isStatic(method.getModifiers());
    }

    public static boolean isStaticField(Class<?> c, String name) {
        try {
            Field field = c.getField(name);
            return isStatic(field);
        } catch (NoSuchFieldException e) {
            throw new RuntimeException(e);
        }
    }

    public static boolean isStatic(Field field) {
        return java.lang.reflect.Modifier.isStatic(field.getModifiers());
    }

    public static char getModifiers(ModifiersLambda f, String className, String method, String methodSig) throws ReflectiveOperationException {
        // Parse signature to get the method signature
        MethodType sig;
        try {
            sig = MethodTypeDesc.ofDescriptor(methodSig).resolveConstantDesc(MethodHandles.lookup());
        } catch (TypeNotPresentException e) {
            System.out.println("Type not present: " + e.typeName());
            return 0;
        }
        Class<?> c;
        try {
            c = Class.forName(className.replace("/", "."));
        } catch (ExceptionInInitializerError | ClassNotFoundException | NoClassDefFoundError e) {
            System.out.println("Class not found: " + className + " error: " + e);
            return 0;
        }
        Executable m;
        try {
            if (method.equals("<init>")) {
                m = c.getDeclaredConstructor(sig.parameterArray());
            } else {
                m = c.getDeclaredMethod(method, sig.parameterArray());
            }
        } catch (NoSuchMethodException e) {
            System.out.println("Method not found in class " + className + ": " + method);
            return 0;
        } catch (NoClassDefFoundError e) {
            System.out.println("Type error in method in class " + className + ": " + method);
            return 0;
        }
        ClassNode classNode = ASMUtils.getClassNode(c);
        return f.op(classNode, m);
    }

    public static char getModifiers(String className, String method, String methodSig) throws ReflectiveOperationException {
        return getModifiers((classNode, m) -> {
            char modifiers = Modifiers.None;
            boolean found = false;
            for (MethodNode methodNode : classNode.methods) {
                if (methodNode.name.equals(method) && methodNode.desc.equals(methodSig)) {
                    if (isNullable(methodNode)) {
                        modifiers |= Modifiers.Nullable;
                    }
                    found = true;
                    break;
                }
            }
            if (!found) {
                System.out.println("Method not found in class " + className + " in bytecode: " + method);
            }

            // Check if the method is static
            if (isStatic(m)) {
                modifiers |= Modifiers.Static;
            }

            return modifiers;
        }, className, method, methodSig);
    }

    public static char getModifiers(String className, String method, String methodSig, int argPos) throws ReflectiveOperationException {
        return getModifiers((classNode, m) -> {
            char modifiers = Modifiers.None;
            boolean found = false;
            for (MethodNode methodNode : classNode.methods) {
                if (methodNode.name.equals(method) && methodNode.desc.equals(methodSig)) {
                    if (isNullable(methodNode, argPos)) {
                        modifiers |= Modifiers.Nullable;
                    }
                    found = true;
                    break;
                }
            }
            if (!found) {
                System.out.println("Method not found in class " + className + " in bytecode: " + method);
            }

            return modifiers;
        }, className, method, methodSig);
    }

    public static char getModifiers(String className, String field) {
        Class<?> c;
        try {
            c = Class.forName(className.replace("/", "."));
        } catch (ExceptionInInitializerError | ClassNotFoundException | NoClassDefFoundError e) {
            System.out.println("Class not found: " + className + " error: " + e);
            return 0;
        }
        Field f;
        try {
            f = c.getDeclaredField(field);
        } catch (NoSuchFieldException e) {
            System.out.println("Field not found in class " + className + ": " + field);
            return 0;
        }
        ClassNode classNode = ASMUtils.getClassNode(c);

        char modifiers = Modifiers.None;
        boolean found = false;
        for (FieldNode fieldNode : classNode.fields) {
            if (fieldNode.name.equals(field)) {
                if (isNullable(fieldNode)) {
                    modifiers |= Modifiers.Nullable;
                }
                found = true;
                break;
            }
        }
        if (!found) {
            System.out.println("Field not found in class " + className + " in bytecode: " + field);
        }

        // Check if the field is static
        if (isStatic(f)) {
            modifiers |= Modifiers.Static;
        }

        return modifiers;
    }

    interface ModifiersLambda {
        char op(ClassNode classNode, Executable m);
    }
}
