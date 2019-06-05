export type RemoteData<T> = { type: "Not Ready" } | { type: "Ready"; data: T }
