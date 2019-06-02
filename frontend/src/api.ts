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

export type PackageVersionReq = {
  group: string
  package: string
  version: string
}

export type PackageView = {
  group: string
  package: string
  latest_version: PackageVersionReq
  owners: UserView[]
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

const URLROOT = "https://api.elba.pub/api/v1"
// const URLROOT = "http://localhost:17000/api/v1"
// const URLROOT = "http://192.168.43.32:17000/api/v1"

export async function login_by_oauth(): Promise<void> {
  const _ = await send_request(`${URLROOT}/users/login/oauth`, "GET")
  return undefined
}

export async function login_by_access_token(
  access_token: string
): Promise<void> {
  const _ = await send_request(
    `${URLROOT}/users/login?gh_access_token=${access_token}`,
    "GET"
  )
  return undefined
}

export async function logout(): Promise<void> {
  const _ = await send_request(`${URLROOT}/users/logout`, "GET")
  return undefined
}

export async function show_user_self(): Promise<UserView> {
  const json = await send_request(`${URLROOT}/users/metadata`, "GET")
  return json.user
}

export async function create_token(): Promise<AccessTokenView> {
  const json = await send_request(`${URLROOT}/users/tokens/create`, "PUT")
  return json.token
}

export async function list_tokens(): Promise<AccessTokenView[]> {
  const json = await send_request(`${URLROOT}/users/tokens`, "GET")
  return json.tokens
}

export async function remove_token(token_id: number): Promise<void> {
  const _ = await send_request(`${URLROOT}/users/tokens/${token_id}`, "DELETE")
  return undefined
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
