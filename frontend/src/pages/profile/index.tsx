import React, { useState, useEffect } from "react"
import style from "./styles.scss"
import { UserConsumer } from "~/utils/user-context.tsx"
import { TokenList, TokenDisplay } from "~components/token-list"
import { UserProfile } from "~components/user-profile"
import { RemoteData } from "~/utils/remote-data"
import "~/models/api"
import {
  show_user_self,
  UserView,
  AccessTokenView,
  list_tokens,
  logout,
} from "~/models/api"
import { Redirect } from "react-router"

const testResults: AccessTokenView[] = new Array(5)
  .fill({
    token_partial: "3423......2344",
    created_at: new Date(),
  })
  .map((item, idx) => ({ ...item, token_id: idx }))

export const UserProfilePage: React.FunctionComponent = () => {
  const [tokenDisplay, setTokenDisplay] = useState<string | void>(undefined)
  const [tokens, setTokens] = useState<RemoteData<AccessTokenView[]>>({
    type: "Not Asked",
  })
  const [user, setUser] = useState<RemoteData<UserView>>({
    type: "Not Asked",
  })

  useEffect(() => {
    if (tokens.type === "Not Asked") {
      setTokens({ type: "Started" })
      async function load() {
        let tokens = await list_tokens()
        setTokens({ type: "Done", data: tokens })
      }
      load()
    }
    if (user.type === "Not Asked") {
      setUser({ type: "Started" })
      async function load() {
        // TODO: redirect on error
        let user = await show_user_self()
        setUser({ type: "Done", data: user })
      }
      load()
    }
  }, [])

  return (
    <UserConsumer>
      {userContext =>
        userContext.user !== undefined ? (
          <div className={style.page}>
            <div className={style["profile-section"]}>
              <h2>Profile</h2>
              <div className={style["profile-section__content"]}>
                <UserProfile user={user} />
              </div>
            </div>
            <div className={style["token-section"]}>
              <div className={style["token-section__title"]}>
                <h2>Access Tokens</h2>
                <button className="button is-purple">Create Token</button>
              </div>
              {tokenDisplay !== undefined ? (
                <div className={style["token-display"]}>
                  <TokenDisplay token={tokenDisplay} />
                </div>
              ) : (
                <div />
              )}
              <TokenList tokens={tokens} />
            </div>
            <div className={style["logout"]}>
              <button
                className="button is-purple"
                onClick={() => onLogout(userContext.fetchUser)}
              >
                Logout
              </button>
            </div>
          </div>
        ) : (
          <Redirect to="/" />
        )
      }
    </UserConsumer>
  )
}

async function onLogout(fetchUser: () => void) {
  await logout()
  fetchUser()
}

export default UserProfilePage
