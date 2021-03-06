export type UserView = {
  id: number
  name: string
  email: string
  avatar?: string
}

export type AccessTokenView = {
  id: number
  token?: string
  token_partial: string
  created_at: string
}

export type GroupReq = {
  group: string
}

export type PackageReq = {
  group: string
  package: string
}

export type VersionReq = {
  group: string
  package: string
  version: string
}

export type PackageView = {
  group: string
  package: string
  latest_version: VersionView
  updated_at: string
  created_at: string
}

export type VersionView = {
  group: string
  package: string
  version: string
  yanked: boolean
  description?: string
  homepage?: string
  repository?: string
  license?: string
  keywords: string[]
  owners: UserView[]
  created_at: string
}

export type DownloadStatsView = {
  total: number
  season: number
}

export type GlobalStatsView = {
  package_count: number
  download_count: number
}

export const APIROOT = "https://api.elba.pub/api/v1"
// export const APIROOT = "http://localhost:17000/api/v1"
// export const APIROOT = "http://192.168.43.32:17000/api/v1"

export async function login_by_access_token(
  access_token: string
): Promise<void> {
  const _ = await send_request(
    `${APIROOT}/users/login?gh_access_token=${access_token}`,
    "GET"
  )
  return undefined
}

export async function logout(): Promise<void> {
  const _ = await send_request(`${APIROOT}/users/logout`, "GET")
  return undefined
}

export async function show_user_self(): Promise<UserView> {
  const json = await send_request(`${APIROOT}/users/metadata`, "GET")
  return json.user
}

export async function create_token(): Promise<AccessTokenView> {
  const json = await send_request(`${APIROOT}/users/tokens/create`, "PUT")
  return json.token
}

export async function list_tokens(): Promise<AccessTokenView[]> {
  const json = await send_request(`${APIROOT}/users/tokens`, "GET")
  return json.tokens
}

export async function remove_token(token_id: number): Promise<void> {
  const _ = await send_request(`${APIROOT}/users/tokens/${token_id}`, "DELETE")
  return undefined
}

export async function search(q: string): Promise<PackageReq[]> {
  const json = await send_request(`${APIROOT}/packages/search?q=${q}`, "GET")
  return json.packages
}

export async function list_groups(): Promise<GroupReq[]> {
  const json = await send_request(`${APIROOT}/packages/groups`, "GET")
  return json.groups
}

export async function list_packages(groupReq: GroupReq): Promise<PackageReq[]> {
  const json = await send_request(
    `${APIROOT}/packages/${groupReq.group}/packages`,
    "GET"
  )
  return json.packages
}

export async function list_versions(
  packageReq: PackageReq
): Promise<VersionReq[]> {
  const json = await send_request(
    `${APIROOT}/packages/${packageReq.group}/${packageReq.package}/versions`,
    "GET"
  )
  return json.versions
}

export async function show_package(
  package_req: PackageReq
): Promise<PackageView> {
  const json = await send_request(
    `${APIROOT}/packages/${package_req.group}/${package_req.package}/metadata`,
    "GET"
  )
  return json.package
}

export async function show_version(
  version_req: VersionReq
): Promise<VersionView> {
  const json = await send_request(
    `${APIROOT}/packages/${version_req.group}/${version_req.package}/${version_req.version}/metadata`,
    "GET"
  )
  return json.version
}

export async function show_readme(
  version_req: VersionReq
): Promise<string | null> {
  const readme = await fetch(
    `${APIROOT}/packages/${version_req.group}/${version_req.package}/${version_req.version}/readme`
  )
  if (readme.status == 200) {
    const text = await readme.text()
    return text
  } else {
    return null
  }
}

export async function download_stats(
  version_req: VersionReq
): Promise<DownloadStatsView> {
  const json = await send_request(
    `${APIROOT}/packages/${version_req.group}/${version_req.package}/${version_req.version}/download_stats`,
    "GET"
  )
  return json.download_stats
}

export async function show_global_stats(): Promise<GlobalStatsView> {
  const json = await send_request(`${APIROOT}/packages/global_stats`, "GET")
  return json.global_stats
}

async function send_request(url: string, method?: string): Promise<any> {
  const res = await fetch(url, {
    method: method,
    credentials: "include",
  })
  const json = await res.json()
  if (json.error !== undefined) {
    console.error(`[${json.error}] ${json.description}`)
    return undefined
  } else {
    return json
  }
}
