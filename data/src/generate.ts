import {
    mkdtemp,
    copyFile,
    rm,
    mkdir,
    readdir,
} from "node:fs/promises";
import { join, dirname } from "node:path";
import { downloadServerJars } from "./fetch/serverJar.ts";

const SUPPORTED_VERSIONS = [
    "26.2",
    "26.1"
];

async function execute(command: string[], cwd: string) {
    const proc = Bun.spawn(command, {
        cwd,
        stdout: "pipe",
        stderr: "pipe",
    });

    const exitCode = await proc.exited;

    if (exitCode !== 0)
        throw new Error(await new Response(proc.stderr).text());

    return await new Response(proc.stdout).text();
}

(async () => {
    const serverJarDirectory = "servers";
    const jarFiles = await downloadServerJars(
        SUPPORTED_VERSIONS,
        serverJarDirectory,
    );

    for (const version of jarFiles) {
        const outputDirectory = join(
            process.cwd(),
            "generated",
            `V${version.version.replaceAll(".", "_")}`,
        );

        if (await Bun.file(outputDirectory).exists()) {
            console.log(`Skipping version ${version.version}`);
            continue;
        }

        // Run the server to output the files
        const generatedDirectory = await mkdtemp(
            `/tmp/generated_${version.version}`,
        );
        const command = ["java", "-DbundlerMainClass=net.minecraft.data.Main", "-jar", version.fileName, "--reports", "--server", "--output", generatedDirectory];
        try {
            await execute(command, serverJarDirectory);
        } catch (e) {
            console.error(
                `An error occurred while processing version ${version.fileName}:`,
                e,
            );
        }
        console.log(`Generated ${version.version}: ${version.path}`);

        // Copy the generated data
        const dataDirectory = await move(
            generatedDirectory,
            outputDirectory,
            "data",
        );
        const reportsDirectory = await move(
            generatedDirectory,
            outputDirectory,
            "reports",
        );

        // Cleanup
        // await cleanDataDirectory(dataDirectory);
        // await cleanReportsDirectory(reportsDirectory);
        await rm(generatedDirectory, { recursive: true, force: true });
    }
})();

async function move(
    from: string,
    to: string,
    subdir: string,
): Promise<string> {
    const destination = join(to, subdir);
    await copyDir(join(from, subdir), destination);
    return destination;
}

async function copyDir(src: string, dest: string): Promise<void> {
    const entries = await readdir(src, {
        recursive: true,
        withFileTypes: true,
    });

    for (const entry of entries) {
        const srcPath = join(entry.parentPath, entry.name);
        const destPath = srcPath.replace(src, dest);
        const destDir = dirname(destPath);

        if (entry.isFile()) {
            await mkdir(destDir, { recursive: true });
            await copyFile(srcPath, destPath);
        }
    }
}