export type RemoteData<T> =
  | { type: "Not Asked" }
  | { type: "Started" }
  | { type: "Failed"; error: Error }
  | { type: "Done"; data: T }
