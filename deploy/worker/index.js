/**
 * Swift Package Registry (SE-0292) over an R2 bucket, for spm.nativeblocks.io.
 *
 * Only the registry needs a Worker. The Maven repo (maven.nativeblocks.io) and
 * the XCFramework downloads (dist.nativeblocks.io) are plain GETs that R2's
 * public custom domains serve directly, so they never reach this code.
 *
 * Every header below was checked against swiftpm's
 * Sources/PackageRegistry/RegistryClient.swift rather than the prose spec,
 * because the two disagree in ways that matter:
 *
 *   - Content-Type on responses is plain `application/json` / `text/x-swift` /
 *     `application/zip`. The `application/vnd.swift.registry.v1+json` form is
 *     what SwiftPM SENDS in Accept; returning it fails validateContentType().
 *   - `Digest` is never read by SwiftPM. Integrity comes from the release
 *     metadata's resources[].checksum, which it pins on first use.
 *   - `Link: rel="latest-version"` is parsed by nothing.
 *   - HEAD is never issued.
 *
 * Bucket layout (keys at root, so the public domains can share the bucket):
 *   nativeblocks/sdk                     release list
 *   nativeblocks/sdk/<version>           release metadata
 *   nativeblocks/sdk/<version>/Package.swift
 *   nativeblocks/sdk/<version>.zip       source archive
 */

const API_VERSION = "1";

/**
 * The one package this registry serves. Everything else 404s, which keeps the
 * Worker from handing out unrelated objects that share the bucket — the Maven
 * tree under io/ and the XCFrameworks under ios/ both live here too, and are
 * served by their own public domains.
 */
const PACKAGE_SCOPE = "nativeblocks";
const PACKAGE_NAME = "sdk";

const JSON_TYPE = "application/json";
const SWIFT_TYPE = "text/x-swift";
const ZIP_TYPE = "application/zip";
const PROBLEM_TYPE = "application/problem+json";

/** Registry indexes change on every release; everything else is immutable. */
const CACHE_INDEX = "public, max-age=60";
const CACHE_IMMUTABLE = "public, max-age=31536000, immutable";

function problem(status, detail) {
  return new Response(JSON.stringify({ status, detail }), {
    status,
    headers: {
      "Content-Type": PROBLEM_TYPE,
      "Content-Version": API_VERSION,
    },
  });
}

/**
 * Classify by path shape, not by substring. The release list and the release
 * metadata have no file extension, so matching on names like "releases" or
 * ".json" silently leaves them with no Content-Type at all.
 */
function classify(pathname) {
  const parts = pathname.replace(/^\/+/, "").split("/").filter(Boolean);

  if (parts[0] !== PACKAGE_SCOPE || parts[1] !== PACKAGE_NAME) return null;

  // /{scope}/{name}
  if (parts.length === 2) {
    return { key: parts.join("/"), type: JSON_TYPE, cache: CACHE_INDEX };
  }

  if (parts.length === 3) {
    const last = parts[2];
    // /{scope}/{name}/{version}.zip
    if (last.endsWith(".zip")) {
      return { key: parts.join("/"), type: ZIP_TYPE, cache: CACHE_IMMUTABLE };
    }
    // /{scope}/{name}/{version}
    return { key: parts.join("/"), type: JSON_TYPE, cache: CACHE_IMMUTABLE };
  }

  // /{scope}/{name}/{version}/Package.swift
  if (parts.length === 4 && parts[3].endsWith(".swift")) {
    return { key: parts.join("/"), type: SWIFT_TYPE, cache: CACHE_IMMUTABLE };
  }

  return null;
}

export default {
  async fetch(request, env) {
    if (request.method !== "GET" && request.method !== "HEAD") {
      return problem(405, "Method not allowed.");
    }

    const url = new URL(request.url);

    // SwiftPM may append ?swift-version=. We publish a single manifest and
    // advertise no alternates, so serve it regardless of the query.
    const route = classify(url.pathname);
    if (!route) return problem(404, "Not found.");

    const object =
      request.method === "HEAD"
        ? await env.DIST_BUCKET.head(route.key)
        : await env.DIST_BUCKET.get(route.key);

    if (!object) return problem(404, `No such resource: ${url.pathname}`);

    const headers = new Headers();
    headers.set("Content-Type", route.type);
    headers.set("Content-Version", API_VERSION);
    headers.set("Cache-Control", route.cache);
    headers.set("Access-Control-Allow-Origin", "*");

    if (object.httpEtag) headers.set("ETag", object.httpEtag);
    if (typeof object.size === "number") {
      headers.set("Content-Length", String(object.size));
    }

    return new Response(request.method === "HEAD" ? null : object.body, {
      status: 200,
      headers,
    });
  },
};
