import * as dotenv from "dotenv";
import * as fs from "node:fs";
import * as path from "node:path";
import { decodeIotaPrivateKey } from "@iota/iota-sdk/cryptography";
import { getFullnodeUrl, IotaClient } from "@iota/iota-sdk/client";
import { Ed25519Keypair } from "@iota/iota-sdk/keypairs/ed25519";
import { Transaction } from "@iota/iota-sdk/transactions";
import { fromHex } from "@iota/iota-sdk/utils";

dotenv.config({ path: path.join(__dirname, ".env") });

const TOOL_NAME = "iota-immutable-publisher";

type AnchorPackageBytecode = {
  modules: string[];
  dependencies: string[];
};

function loadAnchorPackage(): AnchorPackageBytecode {
  const raw = fs.readFileSync(
    path.join(__dirname, "anchor-package.json"),
    "utf8",
  );
  const parsed = JSON.parse(raw) as AnchorPackageBytecode;
  if (!Array.isArray(parsed.modules) || parsed.modules.length === 0) {
    throw new Error("anchor-package.json inválido: modules vacío");
  }
  if (!Array.isArray(parsed.dependencies)) {
    throw new Error("anchor-package.json inválido: dependencies ausente");
  }
  return parsed;
}

function extractPublishedPackageId(
  objectChanges: Array<{ type: string; packageId?: string }>,
): string {
  for (const change of objectChanges) {
    if (change.type === "published" && change.packageId) {
      return change.packageId;
    }
  }
  throw new Error(
    "No se encontró packageId en objectChanges tras publicar el módulo anchor",
  );
}

async function ensureAnchorPackageId(
  client: IotaClient,
  keypair: Ed25519Keypair,
  sender: string,
): Promise<string> {
  const cached = process.env.IOTA_ANCHOR_PACKAGE_ID?.trim();
  if (cached) {
    return cached;
  }

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

type InputPayload = {
  action: string;
  network: string;
  payload: string;
};

type CapsuleEnvelope = {
  success: boolean;
  exitCode: number;
  feedback: string;
  result: {
    transaction_digest?: string;
    object_id?: string;
  };
};

function emit(envelope: CapsuleEnvelope): never {
  console.log(JSON.stringify(envelope));
  process.exit(envelope.exitCode);
}

function argvPayloadString(): string {
  const parts = process.argv.slice(2);
  if (parts.length === 0) return "";
  if (parts.length === 1) return parts[0];
  const gluedComma = parts.join(",");
  if (gluedComma.trimEnd().endsWith("}")) return gluedComma;
  return parts.join(" ");
}

async function readInputJson(): Promise<unknown> {
  const arg = argvPayloadString();
  if (arg && arg.trim().length > 0) {
    return JSON.parse(arg) as unknown;
  }
  const chunks: Buffer[] = [];
  for await (const chunk of process.stdin) {
    chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk));
  }
  const raw = Buffer.concat(chunks).toString("utf8").trim();
  if (!raw) {
    throw new Error("Entrada vacía: se requiere JSON en argv[2] o stdin");
  }
  return JSON.parse(raw) as unknown;
}

function parseInput(body: unknown): InputPayload {
  if (!body || typeof body !== "object") {
    throw new Error("Payload inválido: se esperaba un objeto JSON");
  }
  const o = body as Record<string, unknown>;
  const action = o.action;
  const network = o.network;
  const payload = o.payload;
  if (typeof action !== "string" || action.length === 0) {
    throw new Error("Campo obligatorio ausente o inválido: action");
  }
  if (typeof network !== "string" || network.length === 0) {
    throw new Error("Campo obligatorio ausente o inválido: network");
  }
  if (typeof payload !== "string" || payload.length === 0) {
    throw new Error("Campo obligatorio ausente o inválido: payload");
  }
  return { action, network, payload };
}

function keypairFromEnvSecret(): Ed25519Keypair {
  const secret = process.env.IOTA_WALLET_SECRET;
  if (!secret || secret.trim().length === 0) {
    throw new Error(
      "IOTA_WALLET_SECRET no definido; cargar exclusivamente vía dotenv (.env local)",
    );
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

async function publishImmutableData(
  network: string,
  payload: string,
): Promise<{ transaction_digest: string; object_id?: string; feedback: string }> {
  if (network !== "testnet") {
    throw new Error(`Red no soportada: ${network}. Solo se admite testnet`);
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
    options: {
      showEffects: true,
      showObjectChanges: true,
    },
  });

  await client.waitForTransaction({ digest: exec.digest });

  let object_id: string | undefined;
  const changes = exec.objectChanges ?? [];
  for (const change of changes) {
    if (change.type === "created" && "objectId" in change) {
      object_id = change.objectId;
      break;
    }
  }

  let feedback = `Transacción inmutable publicada en testnet (${TOOL_NAME})`;
  if (!process.env.IOTA_ANCHOR_PACKAGE_ID?.trim()) {
    feedback += `; paquete anchor publicado (${packageId}). Opcional: fijar IOTA_ANCHOR_PACKAGE_ID en .env`;
  }

  return {
    transaction_digest: exec.digest,
    object_id,
    feedback,
  };
}

async function main(): Promise<void> {
  try {
    const raw = await readInputJson();
    const input = parseInput(raw);

    if (input.action !== "publish_immutable_data") {
      throw new Error(`Acción no soportada: ${input.action}`);
    }

    const published = await publishImmutableData(input.network, input.payload);

    emit({
      success: true,
      exitCode: 0,
      feedback: published.feedback,
      result: {
        transaction_digest: published.transaction_digest,
        ...(published.object_id ? { object_id: published.object_id } : {}),
      },
    });
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    emit({
      success: false,
      exitCode: 1,
      feedback: message,
      result: {},
    });
  }
}

void main();
