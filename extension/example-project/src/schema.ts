import { z } from "zod";

type EnvType = "build-time" | "runtime";

export const envRegistry = z.registry<{ envType: EnvType; group: string }>();

export const schema = z.object({
    mySecret: z.string()
        .default("defaultValue")
        .describe("This is a secret value")
        .register(envRegistry, { envType: "runtime", group: "Secrets" }),
    debug: z.boolean()
        .default(false)
        .describe("Enable debug mode")
        .register(envRegistry, { envType: "build-time", group: "Settings" }),
    apiUrl: z.string()
        .describe("API endpoint URL")
        .register(envRegistry, { envType: "runtime", group: "API" }),
});
