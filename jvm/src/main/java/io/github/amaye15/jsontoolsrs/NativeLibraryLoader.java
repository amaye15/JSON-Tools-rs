package io.github.amaye15.jsontoolsrs;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Locale;

/**
 * Extracts and loads the platform-specific native library embedded as a JAR resource
 * under {@code /native/<os>-<arch>/lib<name>.<ext>}, following the same
 * extract-to-temp-file-then-{@code System.load} pattern used by zstd-jni/lz4-java.
 *
 * <p>The packaged jar officially bundles {@code linux-x86_64} and {@code
 * linux-aarch64} native libraries (standard Databricks compute and Graviton
 * instances respectively). {@code darwin-*} resources may also be present in
 * locally-built/dev jars for fast native test iteration, but are not part of the
 * distributed platform matrix.
 */
final class NativeLibraryLoader {

    private NativeLibraryLoader() {}

    static void load(String libraryBaseName) {
        String platform = detectPlatform();
        String extension = detectExtension();
        String resourcePath = "/native/" + platform + "/lib" + libraryBaseName + "." + extension;

        try (InputStream in = NativeLibraryLoader.class.getResourceAsStream(resourcePath)) {
            if (in == null) {
                throw new UnsatisfiedLinkError(
                        "No native library bundled for platform '" + platform + "' at "
                                + resourcePath + ". json-tools-rs-spark officially ships "
                                + "linux-x86_64 and linux-aarch64 native libraries.");
            }
            Path tempFile = Files.createTempFile("libjson_tools_rs_jni", "." + extension);
            tempFile.toFile().deleteOnExit();
            try (OutputStream out = Files.newOutputStream(tempFile)) {
                in.transferTo(out);
            }
            System.load(tempFile.toAbsolutePath().toString());
        } catch (IOException e) {
            throw new UnsatisfiedLinkError(
                    "Failed to extract native library from " + resourcePath + ": " + e.getMessage());
        }
    }

    private static String detectPlatform() {
        return detectOsFamily() + "-" + detectArch();
    }

    private static String detectOsFamily() {
        String osName = System.getProperty("os.name", "").toLowerCase(Locale.ROOT);
        if (osName.contains("linux")) {
            return "linux";
        } else if (osName.contains("mac") || osName.contains("darwin")) {
            return "darwin";
        } else if (osName.contains("windows")) {
            return "windows";
        }
        throw new UnsatisfiedLinkError("Unsupported OS: " + osName);
    }

    private static String detectArch() {
        String arch = System.getProperty("os.arch", "").toLowerCase(Locale.ROOT);
        if (arch.equals("x86_64") || arch.equals("amd64")) {
            return "x86_64";
        } else if (arch.equals("aarch64") || arch.equals("arm64")) {
            return "aarch64";
        }
        throw new UnsatisfiedLinkError("Unsupported architecture: " + arch);
    }

    private static String detectExtension() {
        String osName = System.getProperty("os.name", "").toLowerCase(Locale.ROOT);
        if (osName.contains("windows")) {
            return "dll";
        } else if (osName.contains("mac") || osName.contains("darwin")) {
            return "dylib";
        }
        return "so";
    }
}
