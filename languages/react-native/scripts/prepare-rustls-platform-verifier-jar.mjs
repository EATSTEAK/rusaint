import AdmZip from 'adm-zip'
import { execFileSync } from 'node:child_process'
import { existsSync, mkdirSync, readdirSync, rmSync, writeFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
const reactNativeDir = path.resolve(__dirname, '..')
const repoRoot = path.resolve(reactNativeDir, '..', '..')
const manifestPath = path.join(repoRoot, 'packages', 'rusaint-ffi', 'Cargo.toml')

const metadataOutput = execFileSync(
  'cargo',
  [
    'metadata',
    '--format-version', '1',
    '--manifest-path', manifestPath,
    '--filter-platform', 'aarch64-linux-android',
  ],
  {
    cwd: reactNativeDir,
    encoding: 'utf8',
    maxBuffer: 10 * 1024 * 1024,
  }
)

const metadata = JSON.parse(metadataOutput)
const androidPackage = metadata.packages.find(
  (pkg) => pkg.name === 'rustls-platform-verifier-android'
)

if (!androidPackage) {
  throw new Error('rustls-platform-verifier-android package not found in cargo metadata')
}

const version = androidPackage.version
const packageDir = path.dirname(androidPackage.manifest_path)
const sourceAar = path.join(
  packageDir,
  'maven',
  'rustls',
  'rustls-platform-verifier',
  version,
  `rustls-platform-verifier-${version}.aar`
)
const targetDir = path.join(reactNativeDir, 'android', 'libs')
const targetJar = path.join(targetDir, 'rustls-platform-verifier-vendored.jar')

if (!existsSync(sourceAar)) {
  throw new Error(`rustls-platform-verifier aar not found at ${sourceAar}`)
}

mkdirSync(targetDir, { recursive: true })
for (const entry of readdirSync(targetDir)) {
  if (
    (entry === 'rustls-platform-verifier-vendored.jar' ||
      /^rustls-platform-verifier-.*\.jar$/.test(entry)) &&
    entry !== path.basename(targetJar)
  ) {
    rmSync(path.join(targetDir, entry), { force: true })
  }
}
const zip = new AdmZip(sourceAar)
const classesJarEntry = zip.getEntry('classes.jar')

if (!classesJarEntry) {
  throw new Error(`classes.jar not found in ${sourceAar}`)
}

writeFileSync(targetJar, zip.readFile(classesJarEntry))

console.log(`Prepared ${path.relative(reactNativeDir, targetJar)}`)
