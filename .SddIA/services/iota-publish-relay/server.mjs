/**
 * Relay HTTP de instancia: anclaje IOTA Testnet para iota-immutable-publisher (nativo).
 * Contrato POST: { action, network, payload } → { success, result: { transaction_digest, object_id? }, error? }
 *
 * Arranque:
 *   PATH=".tools/node-v22.16.0-linux-x64/bin:$PATH"
 *   cd .SddIA/services/iota-publish-relay && npm install && npm start
 *
 * Bóveda: IOTA_WALLET_SECRET + IOTA_ANCHOR_PACKAGE_ID en .SddIA/.dev/.env
 * Capsule: IOTA_PUBLISH_RELAY_URL=http://127.0.0.1:8787/v1/publish
 * Mock: SDDIA_LAB_MOCK_IOTA_URL debe quedar vacío (precede al relay si se setea).
 */
import * as dotenv from "dotenv";
import * as fs from "node:fs";
import * as http from "node:http";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import { decodeIotaPrivateKey } from "@iota/iota-sdk/cryptography";
import { getFullnodeUrl, IotaClient } from "@iota/iota-sdk/client";
import { Ed25519Keypair } from "@iota/iota-sdk/keypairs/ed25519";
import { Transaction } from "@iota/iota-sdk/transactions";
import { fromHex } from "@iota/iota-sdk/utils";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = path.resolve(__dirname, "../../..");

dotenv.config({ path: path.join(REPO_ROOT, ".dev/.env") });
dotenv.config({ path: path.join(REPO_ROOT, ".SddIA/.dev/.env"), override: true });

const HOST = process.env.IOTA_PUBLISH_RELAY_HOST || "127.0.0.1";
const PORT = Number(process.env.IOTA_PUBLISH_RELAY_PORT || "8787");
const PATH_PUBLISH = "/v1/publish";

function loadAnchorPackage() {
  const raw = fs.readFileSync(path.join(__dirname, "anchor-package.json"), "utf8");
  const parsed = JSON.parse(raw);
  if (!Array.isArray(parsed.modules) || parsed.modules.length === 0) {
    throw new Error("anchor-package.json inválido: modules vacío");
  }
  if (!Array.isArray(parsed.dependencies)) {
    throw new Error("anchor-package.json inválido: dependencies ausente");
  }
  return parsed;
}

function keypairFromEnvSecret() {
  const secret = process.env.IOTA_WALLET_SECRET;
  if (!secret || secret.trim().length === 0) {
    throw new Error("config-missing: IOTA_WALLET_SECRET");
  }
  const trimmed = secret.trim();
  if (trimmed.startsWith("iotaprivkey")) {
    const { secretKey } = decodeIotaPrivateKey(trimmed);
    return Ed25519Keypair.fromSecretKey(secretKey);
  }
  if (trimmed.startsWith("0x")) {
    return Ed25519Keypair.fromSecretKey(fromHex(trimmed));
  }
  if (trimmed.split(/\s+/).length >= 12) {
    return Ed25519Keypair.deriveKeypair(trimmed);
  }
  try {
    return Ed25519Keypair.fromSecretKey(fromHex(trimmed));
  } catch {
    return Ed25519Keypair.fromSecretKey(
      Uint8Array.from(Buffer.from(trimmed, "base64")),
    );
  }
}

function extractPublishedPackageId(objectChanges) {
  for (const change of objectChanges || []) {
    if (change.type === "published" && change.packageId) {
      return change.packageId;
    }
  }
  throw new Error("No se encontró packageId tras publicar módulo anchor");
}

async function ensureAnchorPackageId(client, keypair, sender) {
  const cached = process.env.IOTA_ANCHOR_PACKAGE_ID?.trim();
  if (cached) return cached;

  const anchorPackage = loadAnchorPackage();
  const tx = new Transaction();
  tx.setSender(sender);
  const [upgradeCap] = tx.publish({
    modules: anchorPackage.modules,
    dependencies: anchorPackage.dependencies,
  });
  tx.transferObjects([upgradeCap], sender);

  const publishExec = await client.signAndExecuteTransaction({
    signer: keypair,
    transaction: tx,
    options: { showObjectChanges: true },
  });
  await client.waitForTransaction({ digest: publishExec.digest });
  return extractPublishedPackageId(publishExec.objectChanges ?? []);
}

async function publishImmutableData(network, payload) {
  if (network !== "testnet") {
    throw new Error(`Red no soportada: ${network}. Solo testnet`);
  }
  const keypair = keypairFromEnvSecret();
  const sender = keypair.getPublicKey().toIotaAddress();
  const client = new IotaClient({ url: getFullnodeUrl("testnet") });
  const packageId = await ensureAnchorPackageId(client, keypair, sender);
  const moveTarget = `${packageId}::anchor::publish_immutable`;
  const payloadBytes = Array.from(new TextEncoder().encode(payload));

  const tx = new Transaction();
  tx.setSender(sender);
  tx.moveCall({
    target: moveTarget,
    arguments: [tx.pure.vector("u8", payloadBytes)],
  });

  const exec = await client.signAndExecuteTransaction({
    signer: keypair,
    transaction: tx,
    options: { showEffects: true, showObjectChanges: true },
  });
  await client.waitForTransaction({ digest: exec.digest });

  let object_id;
  for (const change of exec.objectChanges ?? []) {
    if (change.type === "created" && "objectId" in change) {
      object_id = change.objectId;
      break;
    }
  }
  return { transaction_digest: exec.digest, object_id };
}

function readBody(req) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    req.on("data", (c) => chunks.push(Buffer.isBuffer(c) ? c : Buffer.from(c)));
    req.on("end", () => resolve(Buffer.concat(chunks).toString("utf8")));
    req.on("error", reject);
  });
}

function sendJson(res, status, body) {
  const raw = JSON.stringify(body);
  res.writeHead(status, {
    "Content-Type": "application/json",
    "Content-Length": Buffer.byteLength(raw),
  });
  res.end(raw);
}

const server = http.createServer(async (req, res) => {
  if (req.method === "GET" && req.url === "/health") {
    sendJson(res, 200, { ok: true, service: "iota-publish-relay" });
    return;
  }
  if (req.method !== "POST" || req.url !== PATH_PUBLISH) {
    sendJson(res, 404, { success: false, error: "not-found" });
    return;
  }
  try {
    const raw = await readBody(req);
    const body = JSON.parse(raw || "{}");
    const action = body.action;
    const network = body.network;
    const payload = body.payload;
    if (action !== "publish_immutable_data") {
      sendJson(res, 400, { success: false, error: `Acción no soportada: ${action}` });
      return;
    }
    if (typeof network !== "string" || !network.trim()) {
      sendJson(res, 400, { success: false, error: "Campo obligatorio ausente: network" });
      return;
    }
    if (typeof payload !== "string" || !payload.trim()) {
      sendJson(res, 400, { success: false, error: "Campo obligatorio ausente: payload" });
      return;
    }
    const published = await publishImmutableData(network.trim(), payload);
    sendJson(res, 200, {
      success: true,
      result: {
        transaction_digest: published.transaction_digest,
        object_id: published.object_id ?? null,
      },
    });
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    sendJson(res, 500, { success: false, error: message, feedback: message });
  }
});

server.listen(PORT, HOST, () => {
  console.error(
    `[iota-publish-relay] listening http://${HOST}:${PORT}${PATH_PUBLISH}`,
  );
});
