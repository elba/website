type UserView = {
  id: number
  name: string
  email: string
  avatar?: string
}

type AccessTokenView = {
  id: number
  token?: string
  token_partial: string
}

type PackageVersionReq = {
  group: string
  package: string
  version: string
}

type PackageView = {
  group: string
  package: string
  latest_version: PackageVersionReq
  owners: UserView[]
  updated_at: Date
  created_at: Date
}

type VersionView = {
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
  created_at: Date
}

const URLROOT = "https://api.elba.pub/api/v1"

async function login_by_oauth(): Promise<void> {
  const _ = await fetch(`${URLROOT}/users/login/oauth`)
  return undefined
}

async function show_user_self(): Promise<UserView> {
  const res = await fetch(`${URLROOT}/users/metadata`)
  const json = await res.json()
  return json.user
}

async function create_token(): Promise<AccessTokenView> {
  const res = await fetch(`${URLROOT}/users/tokens/create`, {
    method: "PUT",
  })
  const json = await res.json()
  return json.token
}

async function list_tokens(): Promise<AccessTokenView[]> {
  const res = await fetch(`${URLROOT}/users/tokens`)
  const json = await res.json()
  return json.tokens
}

async function remove_token(token_id: number): Promise<void> {
  const _ = await fetch(`${URLROOT}/users/tokens/${token_id}`, {
    method: "DELETE",
  })
  return undefined
}
