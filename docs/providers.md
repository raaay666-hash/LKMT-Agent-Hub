# Provider boundaries (planned)

Synchronization requires a versioned manifest protocol and atomic compare-and-swap / concurrency resolution before it can claim cross-device lock safety. SQLite is local application state and must never be committed as a synchronization mechanism. GitHubProvider and FutureSelfHostedProvider are not implemented; no network or authentication operations exist. Local storage is used for the application database. NAS and cloud providers are not implemented. Never assume a network filesystem provides safe atomic distributed leases.

The `SyncProvider` interface and a local JSON-file implementation are experimental and not wired into the app UI or SQLite task workflow. GitHub and self-hosted implementations reject operations. The storage interface only describes provider categories; it does not access NAS or cloud.
