import { config } from "https://deno.land/x/dotenv/mod.ts";
import { serve } from "https://deno.land/std@0.159.0/http/server.ts";

const decoder = new TextDecoder();
async function $(command: string) {
  const p = Deno.run({ cmd: ["bash", "-c", command], stdout: "piped", stderr: "piped" });
  const [status, output, stdErrOutput] = await Promise.all([
    p.status(), p.output(), p.stderrOutput()
  ]);
  console.log(status.code, decoder.decode(output));
  console.error(decoder.decode(stdErrOutput));
}


function main() {
  const { ACCESS_TOKEN = "DUMMY" } = config();
  serve(async (req) => {
    const token = req.headers.get("authorization")?.replace("Bearer", "").trim() ?? "";
    if (ACCESS_TOKEN === token) {
      await $("docker compose pull");
      await $("docker compose up -d");
      return new Response("{}", { status: 200 });
    }
    return new Response("No access right.", { status: 403 });
  });
}

main();